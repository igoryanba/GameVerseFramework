use gameverse_client::m2::{AlphaAuthentication, Client, NewCharacter};
use gameverse_client::{ipc, ipc_m2, ui};
use gameverse_protocol::presence_v2::{
    Appearance, CombatPresentation, Locomotion, PlayerFrame, Transform,
};
use gameverse_protocol::{adapter, presence::PlayerState};
use gameverse_rp::{
    auth,
    persistence::{CharacterRepository, EconomyRepository, PostgresStore},
};
use gameverse_transport::{generate_identity, server_endpoint};
use serde_json::{json, Value};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
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

async fn ui_command(
    tx: &mut (impl tokio::io::AsyncWrite + Unpin),
    rx: &mut (impl tokio::io::AsyncRead + Unpin),
    id: &str,
    command: &str,
    payload: Value,
) -> anyhow::Result<ui::UiResponse> {
    ui::write(
        tx,
        &ui::UiRequest {
            schema_version: ui::VERSION,
            request_id: id.into(),
            command: command.into(),
            payload,
        },
    )
    .await?;
    Ok(ui::read(rx).await?)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn ui_bridge_adapter_quic_and_postgres_form_one_alpha_path() -> anyhow::Result<()> {
    let Ok(database_url) = std::env::var("DATABASE_URL") else {
        eprintln!("DATABASE_URL is unset; bridge PostgreSQL acceptance skipped");
        return Ok(());
    };
    let store = PostgresStore::connect(&database_url, 8).await?;
    store.migrate().await?;
    let suffix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let invite = auth::issue_invite();
    sqlx::query("INSERT INTO invites(code_hash) VALUES($1)")
        .bind(auth::invite_hash(&invite))
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

    let (adapter_bridge, adapter_test) = tokio::io::duplex(256 * 1024);
    let (ui_bridge, ui_test) = tokio::io::duplex(256 * 1024);
    let cert_for_bridge = cert.clone();
    let bridge = tokio::spawn(async move {
        ipc_m2::serve_streams(
            adapter_bridge,
            ui_bridge,
            address,
            &cert_for_bridge,
            Instant::now() + Duration::from_secs(30),
        )
        .await
    });
    let (mut adapter_rx, mut adapter_tx) = tokio::io::split(adapter_test);
    let (mut ui_rx, mut ui_tx) = tokio::io::split(ui_test);

    ui::write(
        &mut ui_tx,
        &ui::UiRequest {
            schema_version: ui::VERSION,
            request_id: "hello".into(),
            command: "ui.hello".into(),
            payload: json!({"ui_build":"acceptance"}),
        },
    )
    .await?;
    ipc::write(
        &mut adapter_tx,
        &adapter::Message::AdapterHello {
            version: adapter::VERSION,
            backend: "acceptance".into(),
        },
    )
    .await?;
    ipc::write(
        &mut adapter_tx,
        &adapter::Message::GameInfo {
            edition: "enhanced".into(),
            build: adapter::GAME_VERSION.into(),
        },
    )
    .await?;
    let hello: ui::UiResponse = ui::read(&mut ui_rx).await?;
    assert!(hello.ok);
    assert_eq!(hello.request_id, "hello");

    let login = format!("bridge_{suffix}");
    let auth_response = ui_command(
        &mut ui_tx,
        &mut ui_rx,
        "register-1",
        "auth.register",
        json!({"login":login,"password":"correct horse battery staple","invite":invite}),
    )
    .await?;
    assert!(auth_response.ok, "{:?}", auth_response);
    let refresh_token = auth_response.payload["refresh_token"]
        .as_str()
        .expect("refresh token")
        .to_owned();

    let characters = ui_command(
        &mut ui_tx,
        &mut ui_rx,
        "characters-1",
        "characters.create",
        json!({"first_name":"Anna","last_name":"Volkova","model_hash":1885233650_u32}),
    )
    .await?;
    let character_id = characters.payload["characters"][0]["id"]
        .as_u64()
        .expect("created character");
    ui::write(
        &mut ui_tx,
        &ui::UiRequest {
            schema_version: ui::VERSION,
            request_id: "select-1".into(),
            command: "characters.select".into(),
            payload: json!({"character_id":character_id}),
        },
    )
    .await?;
    let begin = ipc::read(&mut adapter_rx).await?;
    let first_generation = match begin {
        adapter::Message::SessionBegin { entity, config, .. } => {
            assert_eq!(config.character_id, Some(character_id));
            entity.generation
        }
        value => anyhow::bail!("expected session begin, got {value:?}"),
    };
    ipc::write(
        &mut adapter_tx,
        &adapter::Message::AdapterStatus {
            event: "session_ready".into(),
            id: None,
        },
    )
    .await?;
    assert!(matches!(
        ipc::read(&mut adapter_rx).await?,
        adapter::Message::SessionActive { .. }
    ));
    let active: ui::UiResponse = ui::read(&mut ui_rx).await?;
    assert!(active.ok);
    assert_eq!(active.payload["stage"], "active");

    assert!(
        ui_command(
            &mut ui_tx,
            &mut ui_rx,
            "job-start-1",
            "job.start",
            json!({"route":"alpha-route"}),
        )
        .await?
        .ok
    );
    let paid = ui_command(
        &mut ui_tx,
        &mut ui_rx,
        "job-finish-1",
        "job.finish",
        json!({"route":"alpha-route","idempotency_key":"bridge-delivery-1"}),
    )
    .await?;
    assert_eq!(paid.payload["cash"], 500);
    let catalog = ui_command(
        &mut ui_tx,
        &mut ui_rx,
        "catalog-1",
        "shop.catalog",
        json!({}),
    )
    .await?;
    assert!(!catalog.payload["items"].as_array().unwrap().is_empty());
    let bought = ui_command(
        &mut ui_tx,
        &mut ui_rx,
        "buy-1",
        "shop.buy",
        json!({"shop":"market","item_id":1,"quantity":2,"idempotency_key":"bridge-buy-1"}),
    )
    .await?;
    assert_eq!(bought.payload["cash"], 260);
    let inventory = ui_command(
        &mut ui_tx,
        &mut ui_rx,
        "inventory-1",
        "inventory.request",
        json!({}),
    )
    .await?;
    assert_eq!(inventory.payload["items"][0]["quantity"], 2);

    ipc::write(
        &mut adapter_tx,
        &adapter::Message::LocalPlayerState {
            sequence: 1,
            state: PlayerState {
                timestamp_ms: 1,
                position: [321.0, 654.0, 25.0],
                rotation: [0.0, 0.0, 0.0, 1.0],
                velocity: [0.0; 3],
                model_hash: 0x705e61f2,
                movement: 0,
                health: 100,
                armor: 0,
                weapon_hash: 0,
            },
        },
    )
    .await?;
    tokio::time::sleep(Duration::from_millis(150)).await;
    drop(ui_tx);
    drop(ui_rx);
    drop(adapter_tx);
    drop(adapter_rx);
    let _ = timeout(Duration::from_secs(5), bridge).await?;
    tokio::time::sleep(Duration::from_millis(150)).await;

    let restored = store
        .characters(auth_response.payload["account_id"].as_u64().unwrap())
        .await?
        .into_iter()
        .find(|value| value.id == character_id)
        .unwrap();
    assert_eq!(restored.position, [321.0, 654.0, 25.0]);

    let mut resumed = Client::connect_alpha(
        address,
        &cert,
        AlphaAuthentication::Resume {
            refresh_token: refresh_token.clone(),
        },
        NewCharacter {
            first_name: "Unused".into(),
            last_name: "Character".into(),
            model_hash: 0x705e61f2,
        },
    )
    .await?;
    assert!(resumed.entity.generation > first_generation);
    assert_eq!(resumed.config.spawn, [321.0, 654.0, 25.0]);
    resumed.logout("logout-1").await?;
    let rejected = Client::connect_alpha(
        address,
        &cert,
        AlphaAuthentication::Resume { refresh_token },
        NewCharacter {
            first_name: "Unused".into(),
            last_name: "Character".into(),
            model_hash: 0x705e61f2,
        },
    )
    .await;
    assert!(rejected.is_err(), "revoked refresh token was accepted");
    stop.send(true)?;
    timeout(Duration::from_secs(5), server).await???;
    Ok(())
}
