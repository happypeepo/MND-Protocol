mod packer;
mod sender;

use axum::{
    routing::post,
    Json, Router,
    extract::State,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use hex::FromHex;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env variables from the contracts/.env if it exists
    let _ = dotenv::from_filename("../contracts/.env");

    let args: Vec<String> = env::args().collect();
    
    if args.len() > 2 && args[1] == "--serve" {
        run_server(args[2].clone()).await?;
    } else {
        println!("Usage: ./sidecar --serve <GATEWAY_ADDR>");
        println!("Example to run API proxy to Anvil:");
        println!("  ./sidecar --serve 0x5FbDB2315678afecb367f032d93F642f64180aa3");
    }

    Ok(())
}

async fn run_server(gateway_addr: String) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/intercept", post(intercept_handler))
        .layer(CorsLayer::permissive())
        .with_state(Arc::new(gateway_addr));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("📡 LatticePress Interceptor active on http://localhost:3000/intercept");
    println!("Waiting for frontend requests...");
    axum::serve(listener, app).await?;
    Ok(())
}

#[derive(Deserialize)]
struct InterceptReq {
    payload_hex: String,
}

#[derive(Serialize)]
struct InterceptRes {
    original_size: usize,
    packed_size: usize,
    tx_hash: String,
    gas_used: u64,
    compression_time_ms: f64,
}

async fn intercept_handler(State(gateway): State<Arc<String>>, Json(req): Json<InterceptReq>) -> Json<InterceptRes> {
    let clean_hex = req.payload_hex.trim_start_matches("0x");
    let payload = Vec::from_hex(clean_hex).unwrap_or_default();
    let original_size = payload.len();

    // 1. Pack with Dictionary + Zstd
    let packed = packer::pack_transaction(&payload).await.unwrap();
    let packed_size = packed.compressed_size; // Real Zstd size reduction
    let compression_time_ms = packed.compression_time_ms;

    // 2. Transmit to network (we send the execution_payload which is dict-only right now)
    let (tx_hash, gas_used) = sender::send_live_transaction(&gateway, packed.execution_payload)
        .await
        .unwrap_or(("Error".to_string(), 0));

    Json(InterceptRes {
        original_size,
        packed_size,
        tx_hash,
        gas_used,
        compression_time_ms,
    })
}
