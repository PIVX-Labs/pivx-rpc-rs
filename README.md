# pivx-rpc-rs

[![Crates.io](https://img.shields.io/crates/v/pivx_rpc_rs.svg)](https://crates.io/crates/pivx_rpc_rs)
[![Documentation](https://docs.rs/pivx_rpc_rs/badge.svg)](https://docs.rs/pivx_rpc_rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.63%2B-blue.svg)](https://www.rust-lang.org)

A Rust library for interacting with PIVX Core via JSON-RPC.

## Features

- 🔌 Full JSON-RPC client for PIVX Core
- 📦 Comprehensive type definitions for all RPC responses
- 🎯 Type-safe API with Rust's strong typing
- 🔄 Automatic request throttling via `throttled_json_rpc`
- ⚡ Support for both mainnet and testnet
- 🔐 Authentication via RPC username/password
- 📚 Well-documented with examples

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pivx_rpc_rs = "0.1"
```

## Quick Start

```rust
use pivx_rpc_rs::PivxRpcClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to PIVX Core node
    let client = PivxRpcClient::new(
        "http://127.0.0.1:51473".to_string(),
        Some("rpcuser".to_string()),
        Some("rpcpassword".to_string()),
        3,      // max parallel requests
        10,     // max retries
        1000,   // timeout (ms)
    );

    // Get blockchain info
    let info = client.getblockchaininfo()?;
    println!("Chain: {}", info.chain);
    println!("Blocks: {}", info.blocks);
    println!("Difficulty: {}", info.difficulty);

    // Get best block
    let best_hash = client.getbestblockhash()?;
    let block = client.getblock(best_hash)?;
    println!("Latest block has {} transactions", block.tx.len());

    Ok(())
}
```

## Configuration

### RPC Connection

Your PIVX Core node must be running with RPC enabled. Configure it in `pivx.conf`:

```conf
server=1
rpcuser=your_username
rpcpassword=your_secure_password
rpcallowip=127.0.0.1
rpcport=51473
```

### Default Ports

- **Mainnet**: 51473
- **Testnet**: 51475

## Examples

The library includes several examples demonstrating common use cases:

- **basic_usage**: Connect and retrieve node information
- **block_explorer**: Fetch and explore block data
- **masternode_status**: Query masternode and staking status
- **transactions**: Work with transactions and UTXOs

Run an example:

```bash
cargo run --example basic_usage
```

## API Coverage

### Blockchain RPCs
- ✅ `getbestblockhash`
- ✅ `getblock`
- ✅ `getblockchaininfo`
- ✅ `getblockcount`
- ✅ `getblockhash`
- ✅ `getblockheader`

### Transaction RPCs
- ✅ `createrawtransaction`
- ✅ `getrawtransaction`
- ✅ `gettxout`
- ✅ `sendrawtransaction`
- ✅ `signrawtransaction`

### Wallet RPCs
- ✅ `dumpprivkey`
- ✅ `getnewaddress`
- ✅ `sendtoaddress`

### Masternode RPCs
- ✅ `getmasternodecount`
- ✅ `listmasternodes`
- ✅ `relaymasternodebroadcast`

### Staking RPCs
- ✅ `getstakingstatus`
- ✅ `listcoldutxos`
- ✅ `delegatoradd`

### Governance RPCs
- ✅ `getbudgetinfo`

### Network/Mining RPCs
- ✅ `getinfo`
- ✅ `getrawmempool`
- ✅ `getsupplyinfo`
- ✅ `generate` (regtest)

## Supported Types

All PIVX-specific data structures are fully typed:

- **Blocks**: `Block`, `FullBlock`
- **Transactions**: `Transaction`, `GetRawTransactionInfo`, `Vin`, `Vout`
- **Masternodes**: `MasternodeList`, `MasternodeCount`
- **Staking**: `PivxStatus`, `ColdUtxo`
- **Blockchain**: `BlockChainInfo`, `Softfork`, `Upgrades`
- **Governance**: `BudgetInfo`
- **Supply**: `MoneySupply`, `ShieldPoolValue`

See the [full documentation](https://docs.rs/pivx_rpc_rs) for details.

## Compatibility

| PIVX Core | pivx-rpc-rs |
|-----------|-------------|
| 5.x       | ✅ 0.1.x    |
| 4.x       | ⚠️ Partial  |

## Technical Notes

### Dependencies

This crate uses:
- `reqwest` 0.9 (blocking mode) for HTTP requests
- `throttled_json_rpc` for RPC client implementation
- `serde` 1.0 for serialization

**Note**: Due to dependency on `throttled_json_rpc`, this crate uses the older `failure` error handling crate. Future versions may migrate to `anyhow` or `thiserror` once the upstream dependency is updated.

### Precision

Financial values (balances, amounts) use `f64` for compatibility with PIVX Core's JSON-RPC responses. Be aware of floating-point precision limitations when handling monetary values.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Resources

- [PIVX Website](https://pivx.org/)
- [PIVX GitHub](https://github.com/PIVX-Project/PIVX)
- [PIVX RPC Documentation](https://github.com/PIVX-Project/PIVX/tree/master/doc)
- [Crate Documentation](https://docs.rs/pivx_rpc_rs)

## Disclaimer

This library is provided as-is. Always test thoroughly with small amounts before using in production, especially when dealing with private keys or sending transactions.


## Disclaimer

This library is provided as-is. Always test thoroughly with small amounts before using in production, especially when dealing with private keys or sending transactions.


