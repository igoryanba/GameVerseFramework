use anyhow::Result;
use reqwest::Client;
use serde::Serialize;
use std::process::{Child, Command};
use std::time::{Duration, Instant};
use tokio::task;
use tokio::time::sleep;

const SERVER_PORT: u16 = 30121;
const BASE_URL: &str = "http://127.0.0.1";
const CONCURRENCY: usize = 200;
const REQUESTS_PER_CONN: usize = 5; // 200*5 = 1000 per iteration
const WS_CONCURRENCY: usize = 200;

#[derive(Debug, Serialize)]
struct PerfReport {
    elapsed_secs: u64,
    avg_tick_ms: f64,
    rss_mb: u64,
    avg_rtt_ms: f64,
    avg_ws_connect_ms: f64,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 GameVerse Performance Demo v0.1");

    // 1) Launch server in background
    let mut server = start_server()?;
    sleep(Duration::from_secs(3)).await; // crude wait for boot

    // 2) Warm-up single request
    let client = Client::new();
    let _ = client.get(format!("{BASE_URL}:{SERVER_PORT}/api/server/status"))
        .send().await?;

    // 3) Generate synthetic load
    println!("⏱  Running synthetic load: {} concurrent * {} req", CONCURRENCY, REQUESTS_PER_CONN);
    let start = Instant::now();
    let avg_rtt = run_load(&client).await?;

    // 3.1 WebSocket load
    let ws_rtt = run_ws_load().await?;

    // 4) Collect metrics
    let metrics = client
        .get(format!("{BASE_URL}:{SERVER_PORT}/api/server/metrics"))
        .send().await?
        .json::<serde_json::Value>().await?;

    let avg_tick_ms = metrics["avg_tick_ms"].as_f64().unwrap_or_default();
    let rss_mb = metrics["rss_mb"].as_u64().unwrap_or_default();

    // 5) Build report
    let report = PerfReport {
        elapsed_secs: start.elapsed().as_secs(),
        avg_tick_ms,
        rss_mb,
        avg_rtt_ms: avg_rtt,
        avg_ws_connect_ms: ws_rtt,
    };

    std::fs::create_dir_all("perf_reports")?;
    std::fs::write("perf_reports/perf_report.json", serde_json::to_string_pretty(&report)?)?;
    println!("✅ perf_report.json saved!\n{:#?}", report);

    // Shutdown server
    let _ = client.post(format!("{BASE_URL}:{SERVER_PORT}/api/server/shutdown"))
        .send().await;
    let _ = server.wait();
    Ok(())
}

fn start_server() -> Result<Child> {
    let child = Command::new("gameverse")
        .args(["server", "start", "--background", "--dummy-resources", "10"])
        .spawn()?;
    Ok(child)
}

async fn run_load(client: &Client) -> Result<f64> {
    let mut handles = Vec::with_capacity(CONCURRENCY);
    for _ in 0..CONCURRENCY {
        let c = client.clone();
        handles.push(task::spawn(async move {
            let mut total = 0.0;
            for _ in 0..REQUESTS_PER_CONN {
                let t0 = Instant::now();
                let _ = c.get(format!("{BASE_URL}:{SERVER_PORT}/api/server/status"))
                    .send().await;
                total += t0.elapsed().as_secs_f64()*1000.0;
            }
            total / REQUESTS_PER_CONN as f64
        }));
    }
    let mut sum = 0.0;
    for h in handles { sum += h.await?; }
    Ok(sum / CONCURRENCY as f64)
}

async fn run_ws_load() -> Result<f64> {
    use tokio_tungstenite::connect_async;
    use url::Url;
    let mut handles = Vec::with_capacity(WS_CONCURRENCY);
    let url = Url::parse(&format!("ws://127.0.0.1:{SERVER_PORT}/api/server/logs/stream")).unwrap();
    for _ in 0..WS_CONCURRENCY {
        let u = url.clone();
        handles.push(tokio::spawn(async move {
            let t0 = Instant::now();
            if let Ok((_stream, _)) = connect_async(u).await {
                // we just count connect time
                t0.elapsed().as_secs_f64()*1000.0
            } else { 0.0 }
        }));
    }
    let mut sum = 0.0;
    for h in handles { sum += h.await?; }
    Ok(sum / WS_CONCURRENCY as f64)
} 