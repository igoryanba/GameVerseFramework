use gameverse_client::presence::Client;
use gameverse_protocol::presence::{PlayerState, Snapshot};
use gameverse_transport::{generate_identity, server_endpoint};
use std::time::Duration;
use tokio::{
    sync::watch,
    time::{sleep, timeout},
};
fn pose(x: f32) -> PlayerState {
    PlayerState {
        timestamp_ms: 1,
        position: [x, 2.0, 30.0],
        rotation: [0.0, 0.0, 0.70710677, 0.70710677],
        velocity: [1.0, 0.0, 0.0],
        model_hash: 0x705e61f2,
        health: 200,
        armor: 25,
        movement: 1,
        weapon_hash: 0xa2719263,
    }
}
async fn wait_for(client: &mut Client, predicate: impl Fn(&Snapshot) -> bool) -> Snapshot {
    timeout(Duration::from_secs(5), async {
        loop {
            let state = client.snapshots.borrow_and_update().clone();
            if predicate(&state) {
                return state;
            }
            client.snapshots.changed().await.unwrap();
        }
    })
    .await
    .unwrap()
}
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn poses_roundtrip_reject_m0_despawn_reconnect_and_shutdown() {
    let dir = tempfile::tempdir().unwrap();
    let cert = dir.path().join("cert.der");
    let key = dir.path().join("key.der");
    generate_identity(&cert, &key).unwrap();
    let endpoint = server_endpoint("127.0.0.1:0".parse().unwrap(), &cert, &key).unwrap();
    let address = endpoint.local_addr().unwrap();
    let (stop, rx) = watch::channel(false);
    let server = tokio::spawn(gameverse_server::presence::run(endpoint, rx));
    assert!(Client::connect_version(address, &cert, 0).await.is_err());
    let mut a = Client::connect(address, &cert).await.unwrap();
    // GTA can spend minutes loading: keep an unspawned client alive past the
    // transport's 15-second application-idle deadline without inventing a pose.
    for _ in 0..17 {
        a.heartbeat().await.unwrap();
        sleep(Duration::from_secs(1)).await;
    }
    assert!(a.snapshots.borrow().entities.is_empty());
    let mut b = Client::connect(address, &cert).await.unwrap();
    assert!(Client::connect(address, &cert).await.is_err());
    assert!(a.snapshots.borrow().entities.is_empty());
    a.publish(pose(1.0)).await.unwrap();
    b.publish(pose(4.0)).await.unwrap();
    let state = wait_for(&mut b, |s| s.entities.len() == 2).await;
    assert_eq!(
        state
            .entities
            .iter()
            .find(|e| e.id == a.entity)
            .unwrap()
            .state,
        pose(1.0)
    );
    let old = a.entity;
    a.close().await.unwrap();
    drop(a);
    wait_for(&mut b, |s| s.entities.len() == 1).await;
    let mut a = Client::connect(address, &cert).await.unwrap();
    assert_eq!(a.entity.slot, old.slot);
    assert!(a.entity.generation > old.generation);
    a.publish(pose(7.0)).await.unwrap();
    let new = wait_for(&mut b, |s| s.entities.len() == 2).await;
    assert!(!new.entities.iter().any(|e| e.id == old));
    a.close().await.unwrap();
    b.close().await.unwrap();
    sleep(Duration::from_millis(100)).await;
    stop.send(true).unwrap();
    let metrics = timeout(Duration::from_secs(5), server)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(metrics["players"], 0);
    assert_eq!(metrics["accepted_sessions"], 3);
    assert_eq!(metrics["disconnects"], 3);
}

#[tokio::test]
async fn ipc_rust_reads_csharp_golden_and_limits_frames() {
    use tokio::io::AsyncWriteExt;
    let bytes = include_bytes!("../../../adapters/gta5/protocol/hello-v1.frame");
    let (mut tx, mut rx) = tokio::io::duplex(128);
    tx.write_all(bytes).await.unwrap();
    assert!(matches!(
        gameverse_client::ipc::read(&mut rx).await.unwrap(),
        gameverse_protocol::adapter::Message::AdapterHello { version: 1, .. }
    ));
    tx.write_all(&65537_u32.to_be_bytes()).await.unwrap();
    assert!(gameverse_client::ipc::read(&mut rx).await.is_err());
}
