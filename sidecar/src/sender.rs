use ethers::prelude::*;
use std::sync::Arc;
use std::str::FromStr;
use std::env;

pub async fn send_live_transaction(gateway_addr: &str, packed_payload: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = "https://testnet-rpc.monad.xyz";
    let provider = Provider::<Http>::try_from(rpc_url)?;
    
    // Read private key from environment
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set in the environment or .env file");
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
    println!("Transaction broadcasted! Tx Hash: https://testnet.monadexplorer.com/tx/0x{:x}", pending_tx.tx_hash());
    
    let receipt = pending_tx.await?.expect("Transaction failed");
    println!("Mined in Block: {}", receipt.block_number.unwrap());
    println!("Gas Used: {}", receipt.gas_used.unwrap());
    
    Ok(())
}
