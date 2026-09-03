use gameverse_client::Client;
use gameverse_protocol::{Message, Snapshot};
use gameverse_transport::{
    client_endpoint, generate_identity, read_message, server_endpoint, write_message,
};
use std::time::Duration;
use tokio::{
    sync::watch,
    time::{sleep, timeout},
};

struct Server {
    _dir: tempfile::TempDir,
    cert: std::path::PathBuf,
    address: std::net::SocketAddr,
    stop: watch::Sender<bool>,
    task: tokio::task::JoinHandle<anyhow::Result<serde_json::Value>>,
}
impl Server {
    fn start() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let cert = dir.path().join("cert.der");
        let key = dir.path().join("key.der");
        generate_identity(&cert, &key).unwrap();
        let endpoint = server_endpoint("127.0.0.1:0".parse().unwrap(), &cert, &key).unwrap();
        let address = endpoint.local_addr().unwrap();
        let (stop, rx) = watch::channel(false);
        let task = tokio::spawn(gameverse_server::run(endpoint, rx));
        Self {
            _dir: dir,
            cert,
            address,
            stop,
            task,
        }
    }
    async fn finish(self) -> serde_json::Value {
        self.stop.send(true).unwrap();
        timeout(Duration::from_secs(5), self.task)
            .await
            .unwrap()
            .unwrap()
            .unwrap()
    }
}
async fn state_after(client: &mut Client, tick: u64) -> Snapshot {
    timeout(Duration::from_secs(5), async {
        loop {
            let state = client.snapshots.borrow_and_update().clone();
            if state.tick >= tick {
                return state;
            }
            client.snapshots.changed().await.unwrap();
        }
    })
    .await
    .unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn two_clients_movement_convergence_capacity_and_reconnect() {
    let server = Server::start();
    let mut a = Client::connect(server.address, &server.cert).await.unwrap();
    let mut b = Client::connect(server.address, &server.cert).await.unwrap();
    let error = Client::connect(server.address, &server.cert)
        .await
        .err()
        .unwrap();
    assert!(error.to_string().contains("server full"));
    a.input([1.0, 0.0]).await.unwrap();
    b.input([0.0, 1.0]).await.unwrap();
    let moved = state_after(&mut a, 10).await;
    let own = moved.entities.iter().find(|e| e.id == a.entity).unwrap();
    let other = moved.entities.iter().find(|e| e.id == b.entity).unwrap();
    assert!(own.position[0] > 0.0 && own.position[1] == 0.0);
    assert!(other.position[1] > 0.0 && other.position[0] == 0.0);
    a.input([0.0, 0.0]).await.unwrap();
    b.input([0.0, 0.0]).await.unwrap();
    let stopped = state_after(&mut a, moved.tick + 6).await;
    let other = state_after(&mut b, stopped.tick).await;
    assert_eq!(stopped.entities, other.entities);
    let old_session = a.session;
    let old_entity = a.entity;
    a.close().await.unwrap();
    drop(a);
    sleep(Duration::from_millis(150)).await;
    let despawn = state_after(&mut b, other.tick + 6).await;
    assert_eq!(despawn.entities.len(), 1);
    let mut a = Client::connect(server.address, &server.cert).await.unwrap();
    assert_ne!(a.session, old_session);
    assert_eq!(a.entity.slot, old_entity.slot);
    assert!(a.entity.generation > old_entity.generation);
    assert_eq!(a.snapshots.borrow().entities.len(), 2);
    a.close().await.unwrap();
    b.close().await.unwrap();
    let report = server.finish().await;
    assert_eq!(report["players"], 0);
    assert_eq!(report["accepted_sessions"], 3);
    assert!(report["max_input_depth"].as_u64().unwrap() <= 128);
}

#[tokio::test]
async fn incompatible_version_and_untrusted_certificate_are_rejected() {
    let server = Server::start();
    let err = Client::connect_version(server.address, &server.cert, 99)
        .await
        .err()
        .unwrap();
    assert!(err.to_string().contains("unsupported protocol"));
    let other = tempfile::tempdir().unwrap();
    let cert = other.path().join("other.der");
    generate_identity(&cert, &other.path().join("other.key")).unwrap();
    assert!(Client::connect(server.address, &cert).await.is_err());
    server.finish().await;
}

#[tokio::test]
async fn handshake_timeout_releases_resources() {
    let server = Server::start();
    let endpoint = client_endpoint(&server.cert).unwrap();
    let connection = endpoint
        .connect(server.address, "localhost")
        .unwrap()
        .await
        .unwrap();
    let (mut send, mut recv) = connection.open_bi().await.unwrap();
    // Open the stream but deliberately leave the first length prefix incomplete.
    send.write_all(&[0]).await.unwrap();
    assert!(timeout(Duration::from_secs(7), read_message(&mut recv))
        .await
        .unwrap()
        .is_err());
    let report = server.finish().await;
    assert_eq!(report["accepted_sessions"], 0);
}

#[tokio::test]
async fn queue_overflow_and_idle_disconnect_are_bounded() {
    let server = Server::start();
    let endpoint = client_endpoint(&server.cert).unwrap();
    let connection = endpoint
        .connect(server.address, "localhost")
        .unwrap()
        .await
        .unwrap();
    let (mut send, mut recv) = connection.open_bi().await.unwrap();
    write_message(&mut send, &Message::Hello { version: 0 })
        .await
        .unwrap();
    assert!(matches!(
        read_message(&mut recv).await.unwrap(),
        Message::Welcome { .. }
    ));
    let mut flood = Vec::new();
    for sequence in 1..=1024 {
        flood.extend(
            gameverse_protocol::encode(&Message::Input {
                sequence,
                direction: [0.0, 0.0],
            })
            .unwrap(),
        );
    }
    let _ = send.write_all(&flood).await;
    let closed = timeout(Duration::from_secs(5), connection.closed()).await;
    assert!(closed.is_ok());
    sleep(Duration::from_millis(150)).await;
    let client = Client::connect(server.address, &server.cert).await.unwrap();
    // Server snapshots do not count as client application activity.
    let mut snapshots = client.snapshots.clone();
    timeout(Duration::from_secs(18), async {
        while snapshots.changed().await.is_ok() {}
    })
    .await
    .unwrap();
    let report = server.finish().await;
    assert!(report["max_input_depth"].as_u64().unwrap() <= 128);
    assert_eq!(report["players"], 0);
}
