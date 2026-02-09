# Lightning-Fast Mempool Monitor

An expert-level tool written in Rust for developers who need sub-millisecond access to pending transactions. This monitor connects directly to an Ethereum node's mempool via WebSockets, providing a competitive edge for arbitrage, liquidations, and alpha discovery.

## Features
* **Zero-Latency Streams**: Uses the `ethers-rs` crate for high-speed WebSocket subscriptions.
* **Smart Filtering**: Filter transactions by gas price, target contract address, or specific function selectors.
* **Input Decoding**: Automatically decodes ABI data for identified transactions.
* **Async Core**: Built on the `tokio` runtime for non-blocking performance.

## Requirements
* Rust (latest stable)
* Ethereum Node with WebSocket support (e.g., Geth, Erigon, or Alchemy)

## Usage
1. Configure your WebSocket URL in `main.rs`.
2. Build the project: `cargo build --release`.
3. Run the monitor: `./target/release/mempool-monitor`.
