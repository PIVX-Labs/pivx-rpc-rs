//! Block explorer example
//!
//! This example demonstrates how to retrieve and explore block information.
//!
//! To run:
//! ```bash
//! cargo run --example block_explorer
//! ```

use pivx_rpc_rs::PivxRpcClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to PIVX node
    let client = PivxRpcClient::new(
        "http://localhost:51473".to_string(),
        Some("your_rpc_user".to_string()),
        Some("your_rpc_password".to_string()),
        3,    // max_parallel_requests
        10,   // max_retries
        1000, // timeout_ms
    );

    // Get the latest block
    println!("Fetching latest block...");
    let best_hash = client.getbestblockhash()?;
    println!("Best block hash: {}", best_hash);

    // Get block header
    println!("\nFetching block header...");
    let block_header = client.getblockheader(best_hash.clone())?;
    println!("Block #{}", block_header.height);
    println!("Version: {}", block_header.version);
    println!("Timestamp: {}", block_header.time);
    println!("Difficulty: {}", block_header.difficulty);
    println!("Nonce: {}", block_header.nonce);
    println!("Confirmations: {}", block_header.confirmations);

    // Get full block with transactions
    println!("\nFetching full block data...");
    let full_block = client.getblock(best_hash)?;
    println!("Block contains {} transaction(s)", full_block.tx.len());

    // Display transaction IDs
    println!("\nTransactions in this block:");
    for (i, tx) in full_block.tx.iter().enumerate() {
        // Transaction struct has txid as an Option<String>
        if let Some(ref txid) = tx.txid {
            println!("  {}. {}", i + 1, txid);
        }
    }

    // Get a specific block by height
    println!("\nFetching block at height 100...");
    let hash_100 = client.getblockhash(100)?;
    let block_100 = client.getblockheader(hash_100)?;
    println!("Block #100 hash: {}", block_100.hash);
    println!("Block #100 time: {}", block_100.time);

    Ok(())
}
