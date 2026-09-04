use gameverse_client::m2::{AlphaAuthentication, Client, NewCharacter};
use gameverse_protocol::presence_v2::{
    Appearance, CombatPresentation, Locomotion, PlayerFrame, Transform,
};
use gameverse_rp::{
    auth,
    persistence::{CharacterRepository, EconomyRepository, PostgresStore},
};
use gameverse_transport::{generate_identity, server_endpoint};
use std::time::Duration;
use tokio::{sync::watch, time::timeout};

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn invite_to_spawn_and_disconnect_position_are_persistent() -> anyhow::Result<()> {
    let Ok(database_url) = std::env::var("DATABASE_URL") else {
        eprintln!("DATABASE_URL is unset; QUIC PostgreSQL acceptance skipped");
        return Ok(());
    };
    let store = PostgresStore::connect(&database_url, 8).await?;
    store.migrate().await?;
    let invite = auth::issue_invite();
    sqlx::query("INSERT INTO invites(code_hash) VALUES($1)")
        .bind(auth::invite_hash(&invite))
        .execute(store.pool())
        .await?;
    sqlx::query("INSERT INTO item_definitions(id,name,unit_weight_grams,usable) VALUES(1,'Water',500,true) ON CONFLICT(id) DO NOTHING")
        .execute(store.pool())
        .await?;
    sqlx::query("INSERT INTO shops(name) VALUES('market') ON CONFLICT(name) DO NOTHING")
        .execute(store.pool())
        .await?;
    sqlx::query("INSERT INTO shop_items(shop_id,item_id,price) SELECT id,1,120 FROM shops WHERE name='market' ON CONFLICT(shop_id,item_id) DO NOTHING")
        .execute(store.pool())
        .await?;
    sqlx::query("INSERT INTO jobs(code) VALUES('courier') ON CONFLICT(code) DO NOTHING")
        .execute(store.pool())
        .await?;

    let dir = tempfile::tempdir()?;
    let cert = dir.path().join("cert.der");
    let key = dir.path().join("key.der");
    generate_identity(&cert, &key)?;
    let endpoint = server_endpoint("127.0.0.1:0".parse()?, &cert, &key)?;
    let address = endpoint.local_addr()?;
    let (stop, rx) = watch::channel(false);
    let server_store = store.clone();
    let server = tokio::spawn(async move {
        gameverse_server::presence_m2::run_alpha(endpoint, server_store, rx).await
    });

    let mut client = Client::connect_alpha(
        address,
        &cert,
        AlphaAuthentication::Register {
            login: "network_tester".into(),
            password: "correct horse battery staple".into(),
            invite,
        },
        NewCharacter {
            first_name: "Anna".into(),
            last_name: "Volkova".into(),
            model_hash: 0x705e61f2,
        },
    )
    .await?;
    let account_id = client.account_id.unwrap();
    let character_id = client.config.character_id.unwrap();
    let refresh_token = client.refresh_token.clone().unwrap();
    client.start_delivery("job-start", "airport-a").await?;
    let (_, paid_cash, _) = client
        .finish_delivery("job-finish", "airport-a", "network-delivery-1")
        .await?;
    assert_eq!(paid_cash, 500);
    let (_, remaining_cash, _) = client
        .buy("shop-buy", "market", 1, 2, "network-purchase-1")
        .await?;
    assert_eq!(remaining_cash, 260);
    assert_eq!(client.inventory("inventory-after-buy").await?, vec![(1, 2)]);
    client.publish(PlayerFrame {
        sequence: 2,
        client_tick: 2,
        transform: Transform {
            position: [321.0, 654.0, 25.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            velocity: [0.0; 3],
        },
        appearance: Some(Appearance {
            model_hash: client.config.model_hash,
        }),
        locomotion: Locomotion::Idle,
        combat: CombatPresentation {
            aiming: false,
            shooting: false,
            reloading: false,
            dead: false,
            weapon_hash: 0,
            aim_target: None,
        },
        vehicle: None,
    })?;
    tokio::time::sleep(Duration::from_millis(100)).await;
    client.close().await?;
    tokio::time::sleep(Duration::from_millis(100)).await;
    let restored = store
        .characters(account_id)
        .await?
        .into_iter()
        .find(|value| value.id == character_id)
        .unwrap();
    assert_eq!(restored.position, [321.0, 654.0, 25.0]);
    assert_eq!(store.wallet(character_id).await?.cash, 260);

    let resumed = Client::connect_alpha(
        address,
        &cert,
        AlphaAuthentication::Resume { refresh_token },
        NewCharacter {
            first_name: "Unused".into(),
            last_name: "Character".into(),
            model_hash: 0x705e61f2,
        },
    )
    .await?;
    assert_eq!(resumed.config.character_id, Some(character_id));
    assert_eq!(resumed.config.spawn, [321.0, 654.0, 25.0]);
    resumed.close().await?;

    stop.send(true)?;
    timeout(Duration::from_secs(5), server).await???;
    Ok(())
}
