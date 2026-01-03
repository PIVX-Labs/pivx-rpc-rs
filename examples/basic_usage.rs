//! Basic usage example for pivx-rpc-rs
//!
//! This example demonstrates connecting to a PIVX node and retrieving basic information.
//!
//! To run:
//! ```bash
//! cargo run --example basic_usage
//! ```

use pivx_rpc_rs::PivxRpcClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to PIVX node
    // Replace these values with your actual node credentials
    let client = PivxRpcClient::new(
        "http://localhost:51473".to_string(),
        Some("your_rpc_user".to_string()),
        Some("your_rpc_password".to_string()),
        3,    // max_parallel_requests
        10,   // max_retries
        1000, // timeout_ms
    );

    // Get general node information
    println!("Getting node information...");
    let info = client.getinfo()?;
    println!("Version: {}", info.version);
    println!("Protocol: {}", info.protocolversion);
    println!("Blocks: {}", info.blocks);
    println!("Connections: {}", info.connections);
    println!("Balance: {} PIV", info.balance);

    // Get blockchain information
    println!("\nGetting blockchain information...");
    let blockchain_info = client.getblockchaininfo()?;
    println!("Chain: {}", blockchain_info.chain);
    println!("Blocks: {}", blockchain_info.blocks);
    println!("Best block hash: {}", blockchain_info.bestblockhash);
    println!("Difficulty: {}", blockchain_info.difficulty);

    // Get best block hash
    println!("\nGetting best block hash...");
    let best_hash = client.getbestblockhash()?;
    println!("Best block: {}", best_hash);

    // Get block count
    println!("\nGetting block count...");
    let block_count = client.getblockcount()?;
    println!("Block height: {}", block_count);

    Ok(())
}
