use gameverse_client::m2::Client;
use gameverse_protocol::presence_v2::{
    Appearance, CombatPresentation, Locomotion, PlayerFrame, Transform,
};
use gameverse_transport::{generate_identity, server_endpoint};
use std::time::Duration;
use tokio::{sync::watch, time::timeout};

fn frame(sequence: u64, x: f32) -> PlayerFrame {
    PlayerFrame {
        sequence,
        client_tick: sequence,
        transform: Transform {
            position: [x, 0.0, 20.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            velocity: [0.0; 3],
        },
        appearance: Some(Appearance { model_hash: 1 }),
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
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn m2_negotiates_bootstraps_and_exchanges_datagram_deltas() {
    let dir = tempfile::tempdir().unwrap();
    let cert = dir.path().join("cert.der");
    let key = dir.path().join("key.der");
    generate_identity(&cert, &key).unwrap();
    let endpoint = server_endpoint("127.0.0.1:0".parse().unwrap(), &cert, &key).unwrap();
    let address = endpoint.local_addr().unwrap();
    let (stop, rx) = watch::channel(false);
    let server = tokio::spawn(gameverse_server::presence_m2::run(endpoint, rx));
    let a = Client::connect(address, &cert, None).await.unwrap();
    let b = Client::connect(address, &cert, None).await.unwrap();
    a.publish(frame(2, 0.0)).unwrap();
    b.publish(frame(2, 5.0)).unwrap();
    let delta = timeout(Duration::from_secs(5), a.read_frame())
        .await
        .unwrap()
        .unwrap();
    assert!(delta.deltas.iter().any(|value| value.id == b.entity));
    let old = b.entity;
    b.close().await.unwrap();
    let destroyed = timeout(Duration::from_secs(5), async {
        loop {
            let delta = a.read_frame().await.unwrap();
            if delta.deltas.iter().any(|value| {
                value.id == old && value.kind == gameverse_protocol::presence_v2::DeltaKind::Destroy
            }) {
                break delta;
            }
        }
    })
    .await
    .unwrap();
    assert!(destroyed.valid());
    a.close().await.unwrap();
    stop.send(true).unwrap();
    let metrics = timeout(Duration::from_secs(5), server)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    assert_eq!(metrics["players"], 0);
    assert_eq!(metrics["accepted_sessions"], 2);
}
