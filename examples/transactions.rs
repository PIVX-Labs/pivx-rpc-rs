//! Transaction example
//!
//! This example demonstrates creating, signing, and sending transactions.
//!
//! **WARNING**: This example deals with real transactions. Use with caution!
//!
//! To run:
//! ```bash
//! cargo run --example transactions
//! ```

use pivx_rpc_rs::{PivxRpcClient, TxInput};
use std::collections::HashMap;

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

    // Example 1: Get transaction information
    println!("Example 1: Retrieving transaction information");
    println!("============================================");

    // You would replace this with an actual transaction ID from your blockchain
    let example_txid = "your_transaction_id_here".to_string();

    match client.getrawtransaction(example_txid.clone(), true) {
        Ok(tx_info) => {
            // GetRawTransactionInfo is a struct, not an enum
            println!("Transaction ID: {}", tx_info.txid);
            println!("Version: {}", tx_info.version);
            println!("Type: {}", tx_info.r#type);
            println!("Size: {}", tx_info.size);
            println!("Inputs: {}", tx_info.vin.len());
            println!("Outputs: {}", tx_info.vout.len());

            println!("\nOutputs:");
            for (i, vout) in tx_info.vout.iter().enumerate() {
                println!("  Output {}: {} PIV", i, vout.value);
            }
        }
        Err(e) => {
            println!("Note: Replace 'your_transaction_id_here' with a real txid");
            println!("Error: {:?}", e);
        }
    }

    // Example 2: Check mempool
    println!("\n\nExample 2: Checking memory pool");
    println!("================================");

    let mempool = client.getrawmempool(false)?;
    match mempool {
        pivx_rpc_rs::RawMemPool::TxIds(txids) => {
            println!("Transactions in mempool: {}", txids.len());
            if !txids.is_empty() {
                println!("First few transactions:");
                for txid in txids.iter().take(5) {
                    println!("  {}", txid);
                }
            }
        }
        pivx_rpc_rs::RawMemPool::Verbose(verbose) => {
            println!("Transactions in mempool: {}", verbose.len());
        }
    }

    // Example 3: Create a raw transaction (not broadcasted)
    println!("\n\nExample 3: Creating raw transaction (example only)");
    println!("==================================================");

    // This is just an example structure - do not broadcast without proper UTXOs!
    let inputs = vec![TxInput {
        txid: "example_input_txid".to_string(),
        vout: 0,
        sequence: None,
    }];

    let mut outputs = HashMap::new();
    outputs.insert("example_destination_address", 1.0);

    match client.createrawtransaction(&inputs, &outputs, None) {
        Ok(raw_tx) => {
            println!("Created raw transaction (hex): {}", &raw_tx[..60]);
            println!("... (truncated)");
            println!("\nNote: This is just an example. Do not broadcast!");
        }
        Err(e) => {
            println!("Expected error (example inputs are invalid): {:?}", e);
        }
    }

    // Example 4: Check UTXO
    println!("\n\nExample 4: Checking UTXO status");
    println!("================================");

    match client.gettxout("example_txid", 0, false) {
        Ok(Some(utxo)) => {
            println!("UTXO found!");
            println!("Value: {} PIV", utxo.value);
            println!("Confirmations: {}", utxo.confirmations);
        }
        Ok(None) => {
            println!("UTXO not found or already spent (expected for example)");
        }
        Err(e) => {
            println!("Expected error (example txid): {:?}", e);
        }
    }

    println!("\n\nNote: This example uses placeholder values.");
    println!("Replace with real transaction IDs for actual use.");

    Ok(())
}
