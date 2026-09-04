use gameverse_rp::{
    auth,
    persistence::{
        AccountRepository, CharacterRepository, EconomyRepository, InventoryRepository,
        JobRepository, PostgresStore, SessionRepository,
    },
    Error,
};
use std::time::SystemTime;

#[tokio::test]
async fn persistent_single_player_vertical_slice() -> anyhow::Result<()> {
    let Ok(database_url) = std::env::var("DATABASE_URL") else {
        eprintln!("DATABASE_URL is unset; PostgreSQL acceptance skipped");
        return Ok(());
    };
    let store = PostgresStore::connect(&database_url, 4).await?;
    store.migrate().await?;

    let invite = auth::issue_invite();
    sqlx::query("INSERT INTO invites(code_hash) VALUES($1)")
        .bind(auth::invite_hash(&invite))
        .execute(store.pool())
        .await?;
    let account = store
        .register_account(&invite, "alpha_tester", "correct horse battery staple")
        .await?;
    assert_eq!(
        store.authenticate("alpha_tester", "wrong password").await?,
        None
    );
    assert_eq!(
        store
            .authenticate("ALPHA_TESTER", "correct horse battery staple")
            .await?,
        Some(account)
    );

    let first_session = store.issue_session(account, SystemTime::now()).await?;
    let resumed = store
        .resume_session(&first_session.tokens.refresh_token, SystemTime::now())
        .await?;
    assert_eq!(resumed.account_id, account);
    assert_ne!(
        resumed.tokens.refresh_token,
        first_session.tokens.refresh_token
    );
    assert!(store.revoke_session(&resumed.tokens.refresh_token).await?);

    let character = store
        .create_character(account, "Ivan", "Petrov", 0x705e61f2)
        .await?;
    sqlx::query(
        "INSERT INTO item_definitions(id,name,unit_weight_grams,usable) VALUES(1,'Water',500,true) ON CONFLICT(id) DO NOTHING",
    )
    .execute(store.pool())
    .await?;
    sqlx::query("INSERT INTO shops(name) VALUES('market') ON CONFLICT(name) DO NOTHING")
        .execute(store.pool())
        .await?;
    sqlx::query("INSERT INTO shop_items(shop_id,item_id,price) SELECT id,1,120 FROM shops WHERE name='market' ON CONFLICT(shop_id,item_id) DO UPDATE SET price=EXCLUDED.price")
        .execute(store.pool())
        .await?;
    sqlx::query("INSERT INTO jobs(code) VALUES('courier') ON CONFLICT(code) DO NOTHING")
        .execute(store.pool())
        .await?;

    store.start_delivery(character.id, "airport-a").await?;
    let catalog = store.shop_catalog("market").await?;
    assert!(catalog
        .iter()
        .any(|offer| offer.item_id == 1 && offer.price == 120));
    let paid = store
        .finish_delivery(character.id, "airport-a", "delivery-1")
        .await?;
    assert_eq!(paid.cash, 500);
    let replay = store
        .finish_delivery(character.id, "airport-a", "delivery-1")
        .await?;
    assert_eq!(replay, paid);
    let conflict = store
        .finish_delivery(character.id, "different-route", "delivery-1")
        .await
        .unwrap_err();
    assert!(conflict.downcast_ref::<Error>() == Some(&Error::TransactionConflict));

    let purchase = store
        .buy(character.id, "market", 1, 2, "purchase-1")
        .await?;
    assert_eq!(purchase.cash, 260);
    assert_eq!(store.inventory(character.id).await?, vec![(1, 2)]);
    store
        .save_position(character.id, [100.0, 200.0, 30.0], 725.0)
        .await?;
    let restored = store.characters(account).await?.pop().unwrap();
    assert_eq!(restored.position, [100.0, 200.0, 30.0]);
    assert_eq!(restored.heading, 5.0);
    assert_eq!(store.wallet(character.id).await?.cash, 260);
    Ok(())
}
