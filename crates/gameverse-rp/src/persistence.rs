//! PostgreSQL repositories. All multi-row domain changes begin in this layer.
use crate::{auth, AccountId, Character, CharacterId, Error, Receipt, Wallet};
use anyhow::{Context, Result};
use async_trait::async_trait;
use sqlx::{postgres::PgPoolOptions, PgPool, Postgres, Transaction};
use std::{collections::BTreeMap, time::SystemTime};
use uuid::Uuid;

#[derive(Clone)]
pub struct PostgresStore {
    pool: PgPool,
}

#[derive(Clone, Debug)]
pub struct StoredAccount {
    pub id: AccountId,
    pub login: String,
    pub password_hash: String,
    pub banned: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionGrant {
    pub session_id: Uuid,
    pub account_id: AccountId,
    pub tokens: auth::TokenPair,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShopOffer {
    pub item_id: u32,
    pub name: String,
    pub price: i64,
}

#[async_trait]
pub trait AccountRepository {
    async fn register_account(
        &self,
        invite: &str,
        login: &str,
        password: &str,
    ) -> Result<AccountId>;
    async fn account_by_login(&self, login: &str) -> Result<Option<StoredAccount>>;
}

#[async_trait]
pub trait SessionRepository {
    async fn issue_session(&self, account_id: AccountId, now: SystemTime) -> Result<SessionGrant>;
    async fn resume_session(&self, refresh_token: &str, now: SystemTime) -> Result<SessionGrant>;
    async fn revoke_session(&self, refresh_token: &str) -> Result<bool>;
}

#[async_trait]
pub trait CharacterRepository {
    async fn characters(&self, account_id: AccountId) -> Result<Vec<Character>>;
    async fn create_character(
        &self,
        account_id: AccountId,
        first_name: &str,
        last_name: &str,
        model_hash: u32,
    ) -> Result<Character>;
    async fn save_position(
        &self,
        character_id: CharacterId,
        position: [f32; 3],
        heading: f32,
    ) -> Result<()>;
}

#[async_trait]
pub trait EconomyRepository {
    async fn wallet(&self, character_id: CharacterId) -> Result<Wallet>;
    async fn finish_delivery(
        &self,
        character_id: CharacterId,
        route: &str,
        idempotency_key: &str,
    ) -> Result<Receipt>;
}

#[async_trait]
pub trait InventoryRepository {
    async fn inventory(&self, character_id: CharacterId) -> Result<Vec<(u32, u32)>>;
    async fn shop_catalog(&self, shop: &str) -> Result<Vec<ShopOffer>>;
    async fn buy(
        &self,
        character_id: CharacterId,
        shop: &str,
        item_id: u32,
        quantity: u32,
        idempotency_key: &str,
    ) -> Result<Receipt>;
}

#[async_trait]
pub trait JobRepository {
    async fn start_delivery(&self, character_id: CharacterId, route: &str) -> Result<()>;
}

#[async_trait]
pub trait AuditRepository {
    async fn record_audit(
        &self,
        actor: Option<AccountId>,
        action: &str,
        target_type: Option<&str>,
        target_id: Option<&str>,
        details: serde_json::Value,
    ) -> Result<()>;
}

impl PostgresStore {
    pub async fn connect(database_url: &str, max_connections: u32) -> Result<Self> {
        anyhow::ensure!(!database_url.trim().is_empty(), "DATABASE_URL is required");
        let pool = PgPoolOptions::new()
            .max_connections(max_connections.clamp(1, 32))
            .connect(database_url)
            .await
            .context("connect PostgreSQL")?;
        Ok(Self { pool })
    }

    pub fn from_pool(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn migrate(&self) -> Result<()> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .context("run GameVerse migrations")
    }

    pub async fn authenticate(&self, login: &str, password: &str) -> Result<Option<AccountId>> {
        let Some(account) = self.account_by_login(login).await? else {
            return Ok(None);
        };
        if account.banned || !auth::verify_password(password, &account.password_hash)? {
            return Ok(None);
        }
        Ok(Some(account.id))
    }
}

#[async_trait]
impl AccountRepository for PostgresStore {
    async fn register_account(
        &self,
        invite: &str,
        login: &str,
        password: &str,
    ) -> Result<AccountId> {
        let normalized = login.trim().to_ascii_lowercase();
        anyhow::ensure!((3..=64).contains(&normalized.len()), "invalid login");
        let password_hash = auth::hash_password(password)?;
        let invite_hash = auth::invite_hash(invite);
        let mut tx = self.pool.begin().await?;
        let invite_id: Option<i64> = sqlx::query_scalar(
            "SELECT id FROM invites WHERE code_hash=$1 AND redeemed_at IS NULL AND (expires_at IS NULL OR expires_at > now()) FOR UPDATE",
        )
        .bind(invite_hash)
        .fetch_optional(&mut *tx)
        .await?;
        let invite_id = invite_id.ok_or(Error::Forbidden)?;
        let account_id: i64 = sqlx::query_scalar(
            "INSERT INTO accounts(login,password_hash) VALUES($1,$2) RETURNING id",
        )
        .bind(normalized)
        .bind(password_hash)
        .fetch_one(&mut *tx)
        .await?;
        let updated = sqlx::query(
            "UPDATE invites SET redeemed_by=$1, redeemed_at=now() WHERE id=$2 AND redeemed_at IS NULL",
        )
        .bind(account_id)
        .bind(invite_id)
        .execute(&mut *tx)
        .await?;
        anyhow::ensure!(updated.rows_affected() == 1, "invite was already redeemed");
        tx.commit().await?;
        Ok(account_id as u64)
    }

    async fn account_by_login(&self, login: &str) -> Result<Option<StoredAccount>> {
        let row: Option<(i64, String, String, bool)> = sqlx::query_as(
            "SELECT a.id,a.login,a.password_hash,EXISTS(SELECT 1 FROM bans b WHERE b.account_id=a.id AND b.revoked_at IS NULL AND (b.expires_at IS NULL OR b.expires_at>now())) FROM accounts a WHERE a.login=$1",
        )
        .bind(login.trim().to_ascii_lowercase())
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|(id, login, password_hash, banned)| StoredAccount {
            id: id as u64,
            login,
            password_hash,
            banned,
        }))
    }
}

#[async_trait]
impl SessionRepository for PostgresStore {
    async fn issue_session(&self, account_id: AccountId, now: SystemTime) -> Result<SessionGrant> {
        let tokens = auth::issue_tokens(now)?;
        let session_id = Uuid::new_v4();
        let mut tx = self.pool.begin().await?;
        revoke_excess_sessions(&mut tx, account_id).await?;
        sqlx::query("INSERT INTO sessions(id,account_id,refresh_token_hash,expires_at) VALUES($1,$2,$3,to_timestamp($4::double precision/1000.0))")
            .bind(session_id)
            .bind(account_id as i64)
            .bind(auth::token_hash(&tokens.refresh_token))
            .bind(tokens.refresh_expires_at_ms as i64)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(SessionGrant {
            session_id,
            account_id,
            tokens,
        })
    }

    async fn resume_session(&self, refresh_token: &str, now: SystemTime) -> Result<SessionGrant> {
        let hash = auth::token_hash(refresh_token);
        let mut tx = self.pool.begin().await?;
        let row: Option<(Uuid, i64)> = sqlx::query_as("SELECT s.id,s.account_id FROM sessions s WHERE s.refresh_token_hash=$1 AND s.revoked_at IS NULL AND s.expires_at>now() AND NOT EXISTS(SELECT 1 FROM bans b WHERE b.account_id=s.account_id AND b.revoked_at IS NULL AND (b.expires_at IS NULL OR b.expires_at>now())) FOR UPDATE")
            .bind(&hash)
            .fetch_optional(&mut *tx)
            .await?;
        let (old_session, account_id) = row.ok_or(Error::Forbidden)?;
        let tokens = auth::issue_tokens(now)?;
        let session_id = Uuid::new_v4();
        sqlx::query("UPDATE sessions SET revoked_at=now() WHERE id=$1 AND revoked_at IS NULL")
            .bind(old_session)
            .execute(&mut *tx)
            .await?;
        sqlx::query("INSERT INTO sessions(id,account_id,refresh_token_hash,expires_at) VALUES($1,$2,$3,to_timestamp($4::double precision/1000.0))")
            .bind(session_id)
            .bind(account_id)
            .bind(auth::token_hash(&tokens.refresh_token))
            .bind(tokens.refresh_expires_at_ms as i64)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(SessionGrant {
            session_id,
            account_id: account_id as u64,
            tokens,
        })
    }

    async fn revoke_session(&self, refresh_token: &str) -> Result<bool> {
        let result = sqlx::query("UPDATE sessions SET revoked_at=now() WHERE refresh_token_hash=$1 AND revoked_at IS NULL")
            .bind(auth::token_hash(refresh_token))
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() == 1)
    }
}

#[async_trait]
impl CharacterRepository for PostgresStore {
    async fn characters(&self, account_id: AccountId) -> Result<Vec<Character>> {
        let rows: Vec<(i64, i64, String, String, i64, i32, serde_json::Value, f64, f64, f64, f64)> = sqlx::query_as(
            "SELECT c.id,c.account_id,c.first_name,c.last_name,c.model_hash,c.instance_id,a.components,p.x,p.y,p.z,p.heading FROM characters c JOIN appearances a ON a.character_id=c.id JOIN character_positions p ON p.character_id=c.id WHERE c.account_id=$1 ORDER BY c.id",
        )
        .bind(account_id as i64)
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter()
            .map(|row| character_from_row(row).context("decode character appearance"))
            .collect()
    }

    async fn create_character(
        &self,
        account_id: AccountId,
        first_name: &str,
        last_name: &str,
        model_hash: u32,
    ) -> Result<Character> {
        anyhow::ensure!(
            valid_character_name(first_name) && valid_character_name(last_name),
            "invalid character name"
        );
        anyhow::ensure!(model_hash != 0, "invalid model hash");
        let mut tx = self.pool.begin().await?;
        let account_exists: Option<i64> =
            sqlx::query_scalar("SELECT id FROM accounts WHERE id=$1 FOR UPDATE")
                .bind(account_id as i64)
                .fetch_optional(&mut *tx)
                .await?;
        anyhow::ensure!(account_exists.is_some(), "account not found");
        let count: i64 = sqlx::query_scalar("SELECT count(*) FROM characters WHERE account_id=$1")
            .bind(account_id as i64)
            .fetch_one(&mut *tx)
            .await?;
        anyhow::ensure!(count < 3, "character capacity exceeded");
        let id: i64 = sqlx::query_scalar("INSERT INTO characters(account_id,first_name,last_name,model_hash) VALUES($1,$2,$3,$4) RETURNING id")
            .bind(account_id as i64)
            .bind(first_name)
            .bind(last_name)
            .bind(i64::from(model_hash))
            .fetch_one(&mut *tx)
            .await?;
        sqlx::query("INSERT INTO appearances(character_id) VALUES($1)")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        sqlx::query(
            "INSERT INTO character_positions(character_id,x,y,z,heading) VALUES($1,$2,$3,$4,$5)",
        )
        .bind(id)
        .bind(-1037.7_f64)
        .bind(-2737.7_f64)
        .bind(20.17_f64)
        .bind(330.0_f64)
        .execute(&mut *tx)
        .await?;
        sqlx::query("INSERT INTO wallets(character_id) VALUES($1)")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        sqlx::query("INSERT INTO inventories(character_id) VALUES($1)")
            .bind(id)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        Ok(Character {
            id: id as u64,
            account_id,
            first_name: first_name.into(),
            last_name: last_name.into(),
            model_hash,
            appearance: BTreeMap::new(),
            position: [-1037.7, -2737.7, 20.17],
            heading: 330.0,
            instance_id: 0,
        })
    }

    async fn save_position(
        &self,
        character_id: CharacterId,
        position: [f32; 3],
        heading: f32,
    ) -> Result<()> {
        anyhow::ensure!(
            position
                .iter()
                .all(|value| value.is_finite() && value.abs() <= 20_000.0)
                && heading.is_finite(),
            "invalid position"
        );
        let result = sqlx::query("UPDATE character_positions SET x=$2,y=$3,z=$4,heading=$5,confirmed_at=now() WHERE character_id=$1")
            .bind(character_id as i64)
            .bind(f64::from(position[0]))
            .bind(f64::from(position[1]))
            .bind(f64::from(position[2]))
            .bind(f64::from(heading.rem_euclid(360.0)))
            .execute(&self.pool)
            .await?;
        anyhow::ensure!(result.rows_affected() == 1, "character not found");
        Ok(())
    }
}

#[async_trait]
impl EconomyRepository for PostgresStore {
    async fn wallet(&self, character_id: CharacterId) -> Result<Wallet> {
        let row: Option<(i64, i64)> =
            sqlx::query_as("SELECT cash,bank FROM wallets WHERE character_id=$1")
                .bind(character_id as i64)
                .fetch_optional(&self.pool)
                .await?;
        let (cash, bank) = row.ok_or(Error::NotFound)?;
        Ok(Wallet { cash, bank })
    }

    async fn finish_delivery(
        &self,
        character_id: CharacterId,
        route: &str,
        idempotency_key: &str,
    ) -> Result<Receipt> {
        let fingerprint = format!("delivery:{route}");
        let mut tx = self.pool.begin().await?;
        if let Some(receipt) =
            replay_receipt(&mut tx, character_id, idempotency_key, &fingerprint).await?
        {
            return Ok(receipt);
        }
        let active: Option<String> = sqlx::query_scalar("SELECT jp.state->>'route' FROM job_progress jp JOIN jobs j ON j.id=jp.job_id WHERE jp.character_id=$1 AND j.code='courier' FOR UPDATE")
            .bind(character_id as i64)
            .fetch_optional(&mut *tx)
            .await?;
        anyhow::ensure!(active.as_deref() == Some(route), "delivery is not active");
        let (cash, bank): (i64, i64) = sqlx::query_as("UPDATE wallets SET cash=cash+500,revision=revision+1 WHERE character_id=$1 RETURNING cash,bank")
            .bind(character_id as i64)
            .fetch_one(&mut *tx)
            .await?;
        let transaction_id: i64 = sqlx::query_scalar("INSERT INTO ledger_entries(character_id,cash_delta,bank_delta,reason,idempotency_key) VALUES($1,500,0,'courier_delivery',$2) RETURNING id")
            .bind(character_id as i64)
            .bind(idempotency_key)
            .fetch_one(&mut *tx)
            .await?;
        sqlx::query("DELETE FROM job_progress WHERE character_id=$1")
            .bind(character_id as i64)
            .execute(&mut *tx)
            .await?;
        let receipt = Receipt {
            transaction_id: transaction_id as u64,
            cash,
            bank,
        };
        save_receipt(
            &mut tx,
            character_id,
            idempotency_key,
            &fingerprint,
            &receipt,
        )
        .await?;
        tx.commit().await?;
        Ok(receipt)
    }
}

#[async_trait]
impl InventoryRepository for PostgresStore {
    async fn inventory(&self, character_id: CharacterId) -> Result<Vec<(u32, u32)>> {
        let rows: Vec<(i32, i32)> = sqlx::query_as(
            "SELECT item_id,quantity FROM inventory_items WHERE character_id=$1 ORDER BY item_id",
        )
        .bind(character_id as i64)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|(item, quantity)| (item as u32, quantity as u32))
            .collect())
    }

    async fn shop_catalog(&self, shop: &str) -> Result<Vec<ShopOffer>> {
        anyhow::ensure!(!shop.is_empty() && shop.len() <= 64, "invalid shop");
        let rows: Vec<(i32, String, i64)> = sqlx::query_as(
            "SELECT d.id,d.name,si.price FROM shops s JOIN shop_items si ON si.shop_id=s.id JOIN item_definitions d ON d.id=si.item_id WHERE s.name=$1 AND s.enabled ORDER BY d.id LIMIT 256",
        )
        .bind(shop)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows
            .into_iter()
            .map(|(item_id, name, price)| ShopOffer {
                item_id: item_id as u32,
                name,
                price,
            })
            .collect())
    }

    async fn buy(
        &self,
        character_id: CharacterId,
        shop: &str,
        item_id: u32,
        quantity: u32,
        idempotency_key: &str,
    ) -> Result<Receipt> {
        anyhow::ensure!((1..=100).contains(&quantity), "invalid quantity");
        let fingerprint = format!("buy:{shop}:{item_id}:{quantity}");
        let mut tx = self.pool.begin().await?;
        if let Some(receipt) =
            replay_receipt(&mut tx, character_id, idempotency_key, &fingerprint).await?
        {
            return Ok(receipt);
        }
        let offer: Option<(i64, i32)> = sqlx::query_as("SELECT si.price,d.unit_weight_grams FROM shops s JOIN shop_items si ON si.shop_id=s.id JOIN item_definitions d ON d.id=si.item_id WHERE s.name=$1 AND s.enabled AND si.item_id=$2")
            .bind(shop)
            .bind(item_id as i32)
            .fetch_optional(&mut *tx)
            .await?;
        let (unit_price, unit_weight) = offer.ok_or(Error::NotFound)?;
        let (cash, bank, max_weight): (i64, i64, i32) = sqlx::query_as("SELECT w.cash,w.bank,i.max_weight_grams FROM wallets w JOIN inventories i ON i.character_id=w.character_id WHERE w.character_id=$1 FOR UPDATE")
            .bind(character_id as i64)
            .fetch_one(&mut *tx)
            .await?;
        let current_weight: i64 = sqlx::query_scalar("SELECT COALESCE(sum(ii.quantity*d.unit_weight_grams),0) FROM inventory_items ii JOIN item_definitions d ON d.id=ii.item_id WHERE ii.character_id=$1")
            .bind(character_id as i64)
            .fetch_one(&mut *tx)
            .await?;
        let total_price = unit_price
            .checked_mul(i64::from(quantity))
            .context("price overflow")?;
        let added_weight = i64::from(unit_weight) * i64::from(quantity);
        anyhow::ensure!(cash >= total_price, "insufficient funds");
        anyhow::ensure!(
            current_weight + added_weight <= i64::from(max_weight),
            "inventory capacity exceeded"
        );
        let new_cash = cash - total_price;
        sqlx::query("UPDATE wallets SET cash=$2,revision=revision+1 WHERE character_id=$1")
            .bind(character_id as i64)
            .bind(new_cash)
            .execute(&mut *tx)
            .await?;
        sqlx::query("INSERT INTO inventory_items(character_id,item_id,quantity) VALUES($1,$2,$3) ON CONFLICT(character_id,item_id) DO UPDATE SET quantity=inventory_items.quantity+EXCLUDED.quantity")
            .bind(character_id as i64)
            .bind(item_id as i32)
            .bind(quantity as i32)
            .execute(&mut *tx)
            .await?;
        sqlx::query("UPDATE inventories SET revision=revision+1 WHERE character_id=$1")
            .bind(character_id as i64)
            .execute(&mut *tx)
            .await?;
        let transaction_id: i64 = sqlx::query_scalar("INSERT INTO ledger_entries(character_id,cash_delta,bank_delta,reason,idempotency_key) VALUES($1,$2,0,'shop_purchase',$3) RETURNING id")
            .bind(character_id as i64)
            .bind(-total_price)
            .bind(idempotency_key)
            .fetch_one(&mut *tx)
            .await?;
        let receipt = Receipt {
            transaction_id: transaction_id as u64,
            cash: new_cash,
            bank,
        };
        save_receipt(
            &mut tx,
            character_id,
            idempotency_key,
            &fingerprint,
            &receipt,
        )
        .await?;
        tx.commit().await?;
        Ok(receipt)
    }
}

#[async_trait]
impl JobRepository for PostgresStore {
    async fn start_delivery(&self, character_id: CharacterId, route: &str) -> Result<()> {
        anyhow::ensure!(
            !route.trim().is_empty() && route.len() <= 64,
            "invalid route"
        );
        let result = sqlx::query("INSERT INTO job_progress(character_id,job_id,state) SELECT $1,id,jsonb_build_object('route',$2::text) FROM jobs WHERE code='courier' AND enabled ON CONFLICT(character_id) DO UPDATE SET job_id=EXCLUDED.job_id,state=EXCLUDED.state,revision=job_progress.revision+1,updated_at=now()")
            .bind(character_id as i64)
            .bind(route)
            .execute(&self.pool)
            .await?;
        anyhow::ensure!(result.rows_affected() == 1, "courier job is unavailable");
        Ok(())
    }
}

#[async_trait]
impl AuditRepository for PostgresStore {
    async fn record_audit(
        &self,
        actor: Option<AccountId>,
        action: &str,
        target_type: Option<&str>,
        target_id: Option<&str>,
        details: serde_json::Value,
    ) -> Result<()> {
        anyhow::ensure!(
            !action.is_empty() && action.len() <= 128,
            "invalid audit action"
        );
        sqlx::query("INSERT INTO audit_events(actor_account_id,action,target_type,target_id,details) VALUES($1,$2,$3,$4,$5)")
            .bind(actor.map(|value| value as i64))
            .bind(action)
            .bind(target_type)
            .bind(target_id)
            .bind(details)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

async fn revoke_excess_sessions(
    tx: &mut Transaction<'_, Postgres>,
    account_id: AccountId,
) -> Result<()> {
    sqlx::query("UPDATE sessions SET revoked_at=now() WHERE id IN (SELECT id FROM sessions WHERE account_id=$1 AND revoked_at IS NULL AND expires_at>now() ORDER BY created_at DESC OFFSET 4)")
        .bind(account_id as i64)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

type CharacterRow = (
    i64,
    i64,
    String,
    String,
    i64,
    i32,
    serde_json::Value,
    f64,
    f64,
    f64,
    f64,
);

fn character_from_row(row: CharacterRow) -> Result<Character> {
    Ok(Character {
        id: row.0 as u64,
        account_id: row.1 as u64,
        first_name: row.2,
        last_name: row.3,
        model_hash: u32::try_from(row.4).context("model hash range")?,
        instance_id: u32::try_from(row.5).context("instance range")?,
        appearance: serde_json::from_value(row.6)?,
        position: [row.7 as f32, row.8 as f32, row.9 as f32],
        heading: row.10 as f32,
    })
}

fn valid_character_name(value: &str) -> bool {
    (2..=32).contains(&value.chars().count())
        && value
            .chars()
            .all(|character| character.is_alphabetic() || character == '-' || character == '\'')
}

async fn replay_receipt(
    tx: &mut Transaction<'_, Postgres>,
    character_id: CharacterId,
    key: &str,
    fingerprint: &str,
) -> Result<Option<Receipt>> {
    anyhow::ensure!(
        !key.is_empty() && key.len() <= 128,
        "invalid idempotency key"
    );
    let row: Option<(String, serde_json::Value)> = sqlx::query_as("SELECT request_fingerprint,response FROM command_receipts WHERE character_id=$1 AND idempotency_key=$2 FOR UPDATE")
        .bind(character_id as i64)
        .bind(key)
        .fetch_optional(&mut **tx)
        .await?;
    match row {
        Some((stored, _)) if stored != fingerprint => Err(Error::TransactionConflict.into()),
        Some((_, response)) => Ok(Some(serde_json::from_value(response)?)),
        None => Ok(None),
    }
}

async fn save_receipt(
    tx: &mut Transaction<'_, Postgres>,
    character_id: CharacterId,
    key: &str,
    fingerprint: &str,
    receipt: &Receipt,
) -> Result<()> {
    sqlx::query("INSERT INTO command_receipts(character_id,idempotency_key,request_fingerprint,response) VALUES($1,$2,$3,$4)")
        .bind(character_id as i64)
        .bind(key)
        .bind(fingerprint)
        .bind(serde_json::to_value(receipt)?)
        .execute(&mut **tx)
        .await?;
    Ok(())
}
