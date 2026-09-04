//! Server-authoritative domain rules for the closed-alpha RP vertical slice.
//! Persistence adapters hash credentials and execute the bundled PostgreSQL migration.
use std::collections::{BTreeMap, BTreeSet};

pub type AccountId = u64;
pub type CharacterId = u64;
pub type ItemId = u32;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotFound,
    Forbidden,
    Duplicate,
    Invalid,
    InsufficientFunds,
    Capacity,
    TransactionConflict,
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::NotFound => "not found",
            Self::Forbidden => "access denied",
            Self::Duplicate => "duplicate value",
            Self::Invalid => "invalid value",
            Self::InsufficientFunds => "insufficient funds",
            Self::Capacity => "capacity exceeded",
            Self::TransactionConflict => "transaction conflict",
        })
    }
}
impl std::error::Error for Error {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Role {
    Player,
    Moderator,
    Administrator,
}

#[derive(Clone, Debug)]
pub struct Account {
    pub id: AccountId,
    pub login: String,
    /// Encoded Argon2id PHC string. Plaintext passwords never enter this domain model.
    pub password_hash: String,
    pub role: Role,
    pub banned: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Character {
    pub id: CharacterId,
    pub account_id: AccountId,
    pub first_name: String,
    pub last_name: String,
    pub model_hash: u32,
    pub appearance: BTreeMap<String, i16>,
    pub position: [f32; 3],
    pub heading: f32,
    pub instance_id: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LedgerEntry {
    pub transaction_id: u64,
    pub character_id: CharacterId,
    pub cash_delta: i64,
    pub bank_delta: i64,
    pub reason: String,
    pub idempotency_key: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Wallet {
    pub cash: i64,
    pub bank: i64,
}

#[derive(Clone, Debug)]
pub struct ItemDefinition {
    pub id: ItemId,
    pub name: String,
    pub unit_weight_grams: u32,
    pub usable: bool,
}

#[derive(Clone, Debug)]
pub struct ShopItem {
    pub item_id: ItemId,
    pub price: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Receipt {
    pub transaction_id: u64,
    pub cash: i64,
    pub bank: i64,
}

#[derive(Clone)]
struct AppliedTransaction {
    receipt: Receipt,
    fingerprint: String,
}

#[derive(Clone, Debug)]
pub struct AuditEvent {
    pub sequence: u64,
    pub actor: AccountId,
    pub action: String,
    pub target: Option<AccountId>,
}

#[derive(Default)]
pub struct AlphaDomain {
    next_id: u64,
    accounts: BTreeMap<AccountId, Account>,
    login_index: BTreeMap<String, AccountId>,
    invites: BTreeSet<String>,
    characters: BTreeMap<CharacterId, Character>,
    wallets: BTreeMap<CharacterId, Wallet>,
    catalog: BTreeMap<ItemId, ItemDefinition>,
    inventory: BTreeMap<CharacterId, BTreeMap<ItemId, u32>>,
    shops: BTreeMap<String, BTreeMap<ItemId, ShopItem>>,
    jobs: BTreeMap<CharacterId, String>,
    ledger: Vec<LedgerEntry>,
    receipts: BTreeMap<(CharacterId, String), AppliedTransaction>,
    muted: BTreeSet<AccountId>,
    audit: Vec<AuditEvent>,
}

impl AlphaDomain {
    fn id(&mut self) -> u64 {
        self.next_id += 1;
        self.next_id
    }

    pub fn add_invite_hash(&mut self, code_hash: impl Into<String>) -> Result<(), Error> {
        let value = code_hash.into();
        if value.len() < 32 || !self.invites.insert(value) {
            return Err(Error::Invalid);
        }
        Ok(())
    }

    pub fn register(
        &mut self,
        invite_hash: &str,
        login: &str,
        password_hash: &str,
    ) -> Result<AccountId, Error> {
        let normalized = login.trim().to_ascii_lowercase();
        if !self.invites.contains(invite_hash)
            || normalized.len() < 3
            || normalized.len() > 64
            || !password_hash.starts_with("$argon2id$")
        {
            return Err(Error::Invalid);
        }
        if self.login_index.contains_key(&normalized) {
            return Err(Error::Duplicate);
        }
        let id = self.id();
        self.invites.remove(invite_hash);
        self.login_index.insert(normalized.clone(), id);
        self.accounts.insert(
            id,
            Account {
                id,
                login: normalized,
                password_hash: password_hash.into(),
                role: Role::Player,
                banned: false,
            },
        );
        Ok(id)
    }

    pub fn create_character(
        &mut self,
        account_id: AccountId,
        first_name: &str,
        last_name: &str,
        model_hash: u32,
    ) -> Result<CharacterId, Error> {
        let account = self.accounts.get(&account_id).ok_or(Error::NotFound)?;
        if account.banned || model_hash == 0 || !valid_name(first_name) || !valid_name(last_name) {
            return Err(Error::Invalid);
        }
        if self
            .characters
            .values()
            .filter(|value| value.account_id == account_id)
            .count()
            >= 3
        {
            return Err(Error::Capacity);
        }
        let id = self.id();
        self.characters.insert(
            id,
            Character {
                id,
                account_id,
                first_name: first_name.into(),
                last_name: last_name.into(),
                model_hash,
                appearance: BTreeMap::new(),
                position: [0.0, 0.0, 72.0],
                heading: 0.0,
                instance_id: 0,
            },
        );
        self.wallets.insert(id, Wallet::default());
        self.inventory.insert(id, BTreeMap::new());
        Ok(id)
    }

    pub fn save_position(
        &mut self,
        character_id: CharacterId,
        position: [f32; 3],
        heading: f32,
    ) -> Result<(), Error> {
        if !position
            .iter()
            .all(|value| value.is_finite() && value.abs() <= 20_000.0)
            || !heading.is_finite()
        {
            return Err(Error::Invalid);
        }
        let character = self
            .characters
            .get_mut(&character_id)
            .ok_or(Error::NotFound)?;
        character.position = position;
        character.heading = heading.rem_euclid(360.0);
        Ok(())
    }

    pub fn define_item(&mut self, item: ItemDefinition) -> Result<(), Error> {
        if item.id == 0 || item.name.trim().is_empty() || item.unit_weight_grams == 0 {
            return Err(Error::Invalid);
        }
        self.catalog.insert(item.id, item);
        Ok(())
    }

    pub fn set_shop(&mut self, name: &str, items: Vec<ShopItem>) -> Result<(), Error> {
        if name.trim().is_empty() || items.is_empty() {
            return Err(Error::Invalid);
        }
        let mut catalog = BTreeMap::new();
        for item in items {
            if item.price <= 0 || !self.catalog.contains_key(&item.item_id) {
                return Err(Error::Invalid);
            }
            catalog.insert(item.item_id, item);
        }
        self.shops.insert(name.into(), catalog);
        Ok(())
    }

    pub fn credit(
        &mut self,
        character: CharacterId,
        cash: i64,
        bank: i64,
        reason: &str,
        key: &str,
    ) -> Result<Receipt, Error> {
        let fingerprint = format!("ledger:{cash}:{bank}:{reason}");
        self.transact(character, cash, bank, reason, key, &fingerprint)
    }

    fn replay(
        &self,
        character: CharacterId,
        key: &str,
        fingerprint: &str,
    ) -> Result<Option<Receipt>, Error> {
        match self.receipts.get(&(character, key.into())) {
            Some(applied) if applied.fingerprint == fingerprint => {
                Ok(Some(applied.receipt.clone()))
            }
            Some(_) => Err(Error::TransactionConflict),
            None => Ok(None),
        }
    }

    fn transact(
        &mut self,
        character: CharacterId,
        cash_delta: i64,
        bank_delta: i64,
        reason: &str,
        key: &str,
        fingerprint: &str,
    ) -> Result<Receipt, Error> {
        if key.is_empty() || key.len() > 128 || reason.is_empty() || reason.len() > 128 {
            return Err(Error::Invalid);
        }
        if let Some(receipt) = self.replay(character, key, fingerprint)? {
            return Ok(receipt);
        }
        let wallet = self.wallets.get(&character).ok_or(Error::NotFound)?;
        let cash = wallet.cash.checked_add(cash_delta).ok_or(Error::Invalid)?;
        let bank = wallet.bank.checked_add(bank_delta).ok_or(Error::Invalid)?;
        if cash < 0 || bank < 0 {
            return Err(Error::InsufficientFunds);
        }
        let transaction_id = self.id();
        *self.wallets.get_mut(&character).expect("wallet checked") = Wallet { cash, bank };
        let entry = LedgerEntry {
            transaction_id,
            character_id: character,
            cash_delta,
            bank_delta,
            reason: reason.into(),
            idempotency_key: key.into(),
        };
        let receipt = Receipt {
            transaction_id,
            cash,
            bank,
        };
        self.ledger.push(entry);
        self.receipts.insert(
            (character, key.into()),
            AppliedTransaction {
                receipt: receipt.clone(),
                fingerprint: fingerprint.into(),
            },
        );
        Ok(receipt)
    }

    pub fn buy(
        &mut self,
        character: CharacterId,
        shop: &str,
        item_id: ItemId,
        quantity: u32,
        key: &str,
    ) -> Result<Receipt, Error> {
        if quantity == 0 || quantity > 100 {
            return Err(Error::Invalid);
        }
        let fingerprint = format!("buy:{shop}:{item_id}:{quantity}");
        if let Some(receipt) = self.replay(character, key, &fingerprint)? {
            return Ok(receipt);
        }
        let offer = self
            .shops
            .get(shop)
            .and_then(|items| items.get(&item_id))
            .ok_or(Error::NotFound)?;
        let price = offer
            .price
            .checked_mul(i64::from(quantity))
            .ok_or(Error::Invalid)?;
        let definition = self.catalog.get(&item_id).ok_or(Error::NotFound)?;
        let inventory = self.inventory.get(&character).ok_or(Error::NotFound)?;
        let current_weight: u64 = inventory
            .iter()
            .map(|(id, count)| u64::from(self.catalog[id].unit_weight_grams) * u64::from(*count))
            .sum();
        let added_weight = u64::from(definition.unit_weight_grams) * u64::from(quantity);
        if current_weight + added_weight > 30_000 {
            return Err(Error::Capacity);
        }
        let receipt = self.transact(character, -price, 0, "shop_purchase", key, &fingerprint)?;
        *self
            .inventory
            .get_mut(&character)
            .expect("inventory checked")
            .entry(item_id)
            .or_default() += quantity;
        Ok(receipt)
    }

    pub fn start_delivery(&mut self, character: CharacterId, route: &str) -> Result<(), Error> {
        if !self.characters.contains_key(&character) || route.trim().is_empty() {
            return Err(Error::Invalid);
        }
        self.jobs.insert(character, route.into());
        Ok(())
    }

    pub fn finish_delivery(
        &mut self,
        character: CharacterId,
        route: &str,
        key: &str,
    ) -> Result<Receipt, Error> {
        let fingerprint = format!("delivery:{route}");
        if let Some(receipt) = self.replay(character, key, &fingerprint)? {
            return Ok(receipt);
        }
        if self.jobs.get(&character).map(String::as_str) != Some(route) {
            return Err(Error::Forbidden);
        }
        let receipt = self.transact(character, 500, 0, "courier_delivery", key, &fingerprint)?;
        self.jobs.remove(&character);
        Ok(receipt)
    }

    pub fn moderate(
        &mut self,
        actor: AccountId,
        target: AccountId,
        action: &str,
    ) -> Result<(), Error> {
        let role = self
            .accounts
            .get(&actor)
            .ok_or(Error::NotFound)?
            .role
            .clone();
        if !matches!(role, Role::Moderator | Role::Administrator) || actor == target {
            return Err(Error::Forbidden);
        }
        match action {
            "mute" => {
                self.muted.insert(target);
            }
            "unmute" => {
                self.muted.remove(&target);
            }
            "ban" => {
                self.accounts
                    .get_mut(&target)
                    .ok_or(Error::NotFound)?
                    .banned = true
            }
            "unban" if matches!(role, Role::Administrator) => {
                self.accounts
                    .get_mut(&target)
                    .ok_or(Error::NotFound)?
                    .banned = false
            }
            _ => return Err(Error::Invalid),
        }
        let sequence = self.id();
        self.audit.push(AuditEvent {
            sequence,
            actor,
            action: action.into(),
            target: Some(target),
        });
        Ok(())
    }

    pub fn wallet(&self, character: CharacterId) -> Option<&Wallet> {
        self.wallets.get(&character)
    }
    pub fn item_count(&self, character: CharacterId, item: ItemId) -> u32 {
        self.inventory
            .get(&character)
            .and_then(|items| items.get(&item))
            .copied()
            .unwrap_or(0)
    }
    pub fn ledger(&self) -> &[LedgerEntry] {
        &self.ledger
    }
}

fn valid_name(value: &str) -> bool {
    (2..=32).contains(&value.chars().count())
        && value
            .chars()
            .all(|character| character.is_alphabetic() || character == '-' || character == '\'')
}

#[cfg(test)]
mod tests {
    use super::*;

    fn alpha() -> (AlphaDomain, CharacterId) {
        let mut domain = AlphaDomain::default();
        let invite = "a".repeat(64);
        domain.add_invite_hash(&invite).unwrap();
        let account = domain
            .register(
                &invite,
                "tester",
                "$argon2id$v=19$m=19456,t=2,p=1$test$hash",
            )
            .unwrap();
        let character = domain
            .create_character(account, "John", "Smith", 0x705e61f2)
            .unwrap();
        (domain, character)
    }

    #[test]
    fn invite_is_single_use_and_names_are_bounded() {
        let (mut domain, _) = alpha();
        assert_eq!(
            domain.register(
                &"a".repeat(64),
                "other",
                "$argon2id$v=19$m=19456,t=2,p=1$x$y"
            ),
            Err(Error::Invalid)
        );
        assert_eq!(
            domain.create_character(1, "1", "Smith", 1),
            Err(Error::Invalid)
        );
    }

    #[test]
    fn vertical_slice_is_atomic_and_idempotent() {
        let (mut domain, character) = alpha();
        domain
            .define_item(ItemDefinition {
                id: 1,
                name: "Water".into(),
                unit_weight_grams: 500,
                usable: true,
            })
            .unwrap();
        domain
            .set_shop(
                "market",
                vec![ShopItem {
                    item_id: 1,
                    price: 120,
                }],
            )
            .unwrap();
        domain.start_delivery(character, "alpha").unwrap();
        let paid = domain
            .finish_delivery(character, "alpha", "delivery:1")
            .unwrap();
        assert_eq!(paid.cash, 500);
        let bought = domain.buy(character, "market", 1, 2, "purchase:1").unwrap();
        assert_eq!(bought.cash, 260);
        assert_eq!(domain.item_count(character, 1), 2);
        let replay = domain.buy(character, "market", 1, 2, "purchase:1").unwrap();
        assert_eq!(replay, bought);
        assert_eq!(domain.item_count(character, 1), 2);
        assert_eq!(domain.ledger().len(), 2);
        assert_eq!(
            domain.buy(character, "market", 1, 3, "purchase:1"),
            Err(Error::TransactionConflict)
        );
    }

    #[test]
    fn failed_purchase_changes_nothing() {
        let (mut domain, character) = alpha();
        domain
            .define_item(ItemDefinition {
                id: 1,
                name: "Heavy".into(),
                unit_weight_grams: 30_000,
                usable: false,
            })
            .unwrap();
        domain
            .set_shop(
                "market",
                vec![ShopItem {
                    item_id: 1,
                    price: 1,
                }],
            )
            .unwrap();
        assert_eq!(
            domain.buy(character, "market", 1, 2, "purchase:heavy"),
            Err(Error::Capacity)
        );
        assert_eq!(domain.item_count(character, 1), 0);
        assert!(domain.ledger().is_empty());
    }
}
