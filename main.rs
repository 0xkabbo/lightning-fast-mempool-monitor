use ethers::prelude::*;
use eyre::Result;
use std::sync::Arc;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    // Replace with your Ethereum WebSocket RPC URL
    let ws_url = "wss://eth-mainnet.g.alchemy.com/v2/YOUR_API_KEY";
    
    let provider = Provider::<Ws>::connect(ws_url).await?;
    let provider = Arc::new(provider);

    println!("Connected to Mempool. Monitoring pending transactions...");

    // Subscribe to all pending transactions
    let mut stream = provider.subscribe_pending_txs().await?;

    while let Some(tx_hash) = stream.next().await {
        let provider_clone = Arc::clone(&provider);
        
        tokio::spawn(async move {
            if let Ok(Some(tx)) = provider_clone.get_transaction(tx_hash).await {
                // Filter: Only show transactions interacting with Uniswap V2 Router
                let uniswap_v2 = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".parse::<Address>().unwrap();
                
                if tx.to == Some(uniswap_v2) {
                    println!("--- New Swap Detected ---");
                    println!("Hash: {:?}", tx.hash);
                    println!("From: {:?}", tx.from);
                    println!("Gas Price: {:?} gwei", tx.gas_price.unwrap_or_default() / 1_000_000_000u64);
                    println!("Input Data: {:?}", tx.input);
                }
            }
        });
    }

    Ok(())
}
