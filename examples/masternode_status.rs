//! Masternode status example
//!
//! This example demonstrates retrieving masternode and staking information.
//!
//! To run:
//! ```bash
//! cargo run --example masternode_status
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

    // Get masternode count
    println!("Getting masternode statistics...");
    let mn_count = client.getmasternodecount()?;
    println!("Total masternodes: {}", mn_count.total);
    println!("Enabled: {}", mn_count.enabled);
    println!("Stable: {}", mn_count.stable);
    println!("In queue: {}", mn_count.inqueue);
    println!("\nNetwork distribution:");
    println!("  IPv4: {}", mn_count.ipv4);
    println!("  IPv6: {}", mn_count.ipv6);
    println!("  Tor: {}", mn_count.onion);

    // List all masternodes
    println!("\nListing masternodes...");
    let masternodes = client.listmasternodes(None)?;
    println!("Found {} masternodes", masternodes.len());

    // Display first 5 masternodes
    println!("\nTop 5 masternodes:");
    for mn in masternodes.iter().take(5) {
        println!("  Rank #{}: {}", mn.rank, mn.addr);
        println!("    Status: {}", mn.status);
        println!("    Type: {}", mn.mn_type);
        println!("    Network: {}", mn.network);
        println!("    Last paid: {} PIV", mn.lastpaid);
        println!();
    }

    // Get staking status
    println!("Getting staking status...");
    let staking = client.getstakingstatus()?;
    println!("Staking active: {}", staking.staking_status);
    println!("Staking enabled: {}", staking.staking_enabled);
    println!("Cold staking enabled: {}", staking.coldstaking_enabled);
    println!("Wallet unlocked: {}", staking.walletunlocked);
    println!("Stakeable coins: {}", staking.stakeablecoins);
    println!("Staking balance: {} PIV", staking.stakingbalance);
    println!("Stake split threshold: {} PIV", staking.stakesplitthreshold);

    // Get money supply
    println!("\nGetting money supply...");
    let supply = client.getsupplyinfo(false)?;
    println!("Total supply: {} PIV", supply.totalsupply);
    println!("Transparent supply: {} PIV", supply.transparentsupply);
    println!("Shielded supply: {} PIV", supply.shieldsupply);

    Ok(())
}
