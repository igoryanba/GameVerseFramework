use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use gameverse_rp::{auth, persistence::PostgresStore};

#[derive(Parser)]
#[command(
    name = "gameverse-admin",
    about = "GameVerse closed-alpha administration"
)]
struct Args {
    #[arg(long, env = "DATABASE_URL", hide_env_values = true)]
    database_url: String,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Migrate,
    Health,
    Invite {
        #[command(subcommand)]
        command: InviteCommand,
    },
    Account {
        #[command(subcommand)]
        command: AccountCommand,
    },
}

#[derive(Subcommand)]
enum InviteCommand {
    Create {
        #[arg(long)]
        created_by: Option<u64>,
        #[arg(long)]
        expires_hours: Option<u32>,
    },
}

#[derive(Subcommand)]
enum AccountCommand {
    Promote {
        #[arg(long)]
        account_id: u64,
        #[arg(long, value_parser = ["player", "moderator", "administrator"])]
        role: String,
    },
    Ban {
        #[arg(long)]
        account_id: u64,
        #[arg(long)]
        created_by: u64,
        #[arg(long)]
        reason: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let store = PostgresStore::connect(&args.database_url, 4).await?;
    match args.command {
        Command::Migrate => {
            store.migrate().await?;
            println!(
                "{}",
                serde_json::json!({"status":"ok","operation":"migrate"})
            );
        }
        Command::Health => {
            let value: i32 = sqlx::query_scalar("SELECT 1")
                .fetch_one(store.pool())
                .await?;
            anyhow::ensure!(value == 1, "unexpected database health result");
            println!("{}", serde_json::json!({"status":"ok","database":"ready"}));
        }
        Command::Invite { command } => match command {
            InviteCommand::Create {
                created_by,
                expires_hours,
            } => {
                let invite = auth::issue_invite();
                let expires_hours = expires_hours.map(i64::from);
                sqlx::query("INSERT INTO invites(code_hash,created_by,expires_at) VALUES($1,$2,CASE WHEN $3::bigint IS NULL THEN NULL ELSE now()+make_interval(hours=>$3::int) END)")
                    .bind(auth::invite_hash(&invite))
                    .bind(created_by.map(|id| id as i64))
                    .bind(expires_hours)
                    .execute(store.pool())
                    .await
                    .context("create invite")?;
                println!("{}", serde_json::json!({"status":"ok","invite":invite}));
            }
        },
        Command::Account { command } => match command {
            AccountCommand::Promote { account_id, role } => {
                let result = sqlx::query("UPDATE accounts SET role=$2 WHERE id=$1")
                    .bind(account_id as i64)
                    .bind(&role)
                    .execute(store.pool())
                    .await?;
                anyhow::ensure!(result.rows_affected() == 1, "account not found");
                println!(
                    "{}",
                    serde_json::json!({"status":"ok","account_id":account_id,"role":role})
                );
            }
            AccountCommand::Ban {
                account_id,
                created_by,
                reason,
            } => {
                anyhow::ensure!(
                    !reason.trim().is_empty() && reason.len() <= 512,
                    "invalid ban reason"
                );
                let mut tx = store.pool().begin().await?;
                sqlx::query("INSERT INTO bans(account_id,reason,created_by) VALUES($1,$2,$3)")
                    .bind(account_id as i64)
                    .bind(&reason)
                    .bind(created_by as i64)
                    .execute(&mut *tx)
                    .await?;
                sqlx::query("UPDATE sessions SET revoked_at=now() WHERE account_id=$1 AND revoked_at IS NULL")
                    .bind(account_id as i64)
                    .execute(&mut *tx)
                    .await?;
                tx.commit().await?;
                println!(
                    "{}",
                    serde_json::json!({"status":"ok","account_id":account_id,"banned":true})
                );
            }
        },
    }
    Ok(())
}
