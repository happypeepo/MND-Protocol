use ethers::prelude::*;
use std::sync::Arc;
use std::str::FromStr;
use std::env;

pub async fn send_live_transaction(gateway_addr: &str, packed_payload: Vec<u8>) -> Result<(String, u64), Box<dyn std::error::Error>> {
    let rpc_url = env::var("RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8545".to_string());
    let provider = Provider::<Http>::try_from(rpc_url)?;
    
    // Read private key from environment or default to Anvil account 0
    let default_pk = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string();
    let private_key = env::var("PRIVATE_KEY").unwrap_or_else(|_| default_pk);
    let wallet: LocalWallet = private_key.parse::<LocalWallet>()?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = SignerMiddleware::new(provider, wallet.with_chain_id(chain_id));
    let client = Arc::new(client);

    let to_addr = Address::from_str(gateway_addr)?;
    
    println!("Dispatching Lattice-Compressed transaction to Gateway: {}", to_addr);
    
    let tx = TransactionRequest::new()
        .to(to_addr)
        .data(packed_payload)
        // Yul inline assembly and delegatecall to another contract require higher gas
        // The default estimation might fail on complex fallbacks
        .gas(500_000);

    let pending_tx = client.send_transaction(tx, None).await?;
    let tx_hash = format!("0x{:x}", pending_tx.tx_hash());
    println!("Transaction broadcasted! Tx Hash: {}", tx_hash);
    
    let receipt = pending_tx.await?.expect("Transaction failed");
    let gas_used = receipt.gas_used.unwrap_or_default().as_u64();
    println!("Mined in Block: {}", receipt.block_number.unwrap_or_default());
    println!("Gas Used: {}", gas_used);
    
    Ok((tx_hash, gas_used))
}
