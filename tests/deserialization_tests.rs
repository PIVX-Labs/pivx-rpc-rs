//! Deserialization tests for PIVX RPC types
//!
//! These tests verify that JSON responses from PIVX Core can be correctly
//! deserialized into our Rust types.

use pivx_rpc_rs::*;

/// Load a fixture file from tests/fixtures/
fn load_fixture(filename: &str) -> String {
    std::fs::read_to_string(format!("tests/fixtures/{}", filename))
        .unwrap_or_else(|_| panic!("Failed to read fixture: {}", filename))
}

#[test]
fn test_deserialize_block_header() {
    let json = load_fixture("block_header.json");
    let block: Block = serde_json::from_str(&json).expect("Failed to deserialize Block");

    assert_eq!(
        block.hash,
        "0000000000000abcdef1234567890abcdef1234567890abcdef1234567890abc"
    );
    assert_eq!(block.confirmations, 12345);
    assert_eq!(block.height, 1000000);
    assert_eq!(block.version, 7);
    assert_eq!(block.nonce, 987654321);
    assert_eq!(block.difficulty, 1234.5678);
    assert_eq!(block.time, 1704240000);
    assert_eq!(block.mediantime, 1704239500);
    assert!(block.previousblockhash.is_some());
}

#[test]
fn test_deserialize_full_block() {
    let json = load_fixture("full_block.json");
    let block: FullBlock = serde_json::from_str(&json).expect("Failed to deserialize FullBlock");

    assert_eq!(
        block.hash,
        "0000000000000abcdef1234567890abcdef1234567890abcdef1234567890abc"
    );
    assert_eq!(block.height, 1000000);
    assert_eq!(block.size, 2048);
    assert_eq!(block.tx.len(), 1);

    // Check first transaction
    let tx = &block.tx[0];
    assert!(tx.txid.is_some());
    assert_eq!(tx.version, 2);
    assert_eq!(tx.vin.len(), 1);
    assert_eq!(tx.vout.len(), 1);

    // Check coinbase input
    match &tx.vin[0] {
        Vin::Coinbase(coinbase) => {
            assert_eq!(coinbase.coinbase, "03e0930a1f2f5669614254432f");
            assert_eq!(coinbase.sequence, 4294967295);
        }
        _ => panic!("Expected Coinbase variant"),
    }

    // Check output
    assert_eq!(tx.vout[0].value, 50.0);
    assert_eq!(tx.vout[0].n, 0);
}

#[test]
fn test_deserialize_transaction() {
    let json = load_fixture("transaction.json");
    let tx: GetRawTransactionInfo =
        serde_json::from_str(&json).expect("Failed to deserialize GetRawTransactionInfo");

    assert_eq!(
        tx.txid,
        "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"
    );
    assert_eq!(tx.version, 2);
    assert_eq!(tx.size, 225);
    assert_eq!(tx.vin.len(), 1);
    assert_eq!(tx.vout.len(), 2);

    // Check outputs
    assert_eq!(tx.vout[0].value, 10.5);
    assert_eq!(tx.vout[1].value, 39.25);
    assert_eq!(tx.vout[0].n, 0);
    assert_eq!(tx.vout[1].n, 1);
}

#[test]
fn test_deserialize_blockchain_info() {
    let json = load_fixture("blockchain_info.json");
    let info: BlockChainInfo =
        serde_json::from_str(&json).expect("Failed to deserialize BlockChainInfo");

    assert_eq!(info.chain, "main");
    assert_eq!(info.blocks, 1000000);
    assert_eq!(info.headers, 1000000);
    assert_eq!(info.difficulty, 1234.5678);
    assert!(!info.initial_block_downloading);

    // Check softforks
    assert_eq!(info.softforks.len(), 1);
    assert_eq!(info.softforks[0].id, "bip65");
    assert_eq!(info.softforks[0].version, 4);
}

#[test]
fn test_deserialize_staking_status() {
    let json = load_fixture("staking_status.json");
    let status: PivxStatus = serde_json::from_str(&json).expect("Failed to deserialize PivxStatus");

    assert!(status.staking_status);
    assert!(status.staking_enabled);
    assert!(!status.coldstaking_enabled);
    assert!(status.haveconnections);
    assert!(status.mnsync);
    assert!(status.walletunlocked);
    assert_eq!(status.stakeablecoins, 150);
    assert_eq!(status.stakingbalance, 7500.0);
    assert_eq!(status.stakesplitthreshold, 2000.0);
    assert_eq!(status.lastattempt_age, 45);
    assert_eq!(status.lastattempt_depth, 2);
}

#[test]
fn test_deserialize_masternode() {
    let json = load_fixture("masternode.json");
    let mn: MasternodeList =
        serde_json::from_str(&json).expect("Failed to deserialize MasternodeList");

    assert_eq!(mn.rank, 1);
    assert_eq!(mn.mn_type, "masternode");
    assert_eq!(mn.network, "ipv4");
    assert_eq!(mn.status, "ENABLED");
    assert_eq!(mn.addr, "123.45.67.89:51472");
    assert_eq!(mn.outidx, 0);
    assert_eq!(mn.lastpaid, 10.0);
}

#[test]
fn test_deserialize_masternode_count() {
    let json = load_fixture("masternode_count.json");
    let count: MasternodeCount =
        serde_json::from_str(&json).expect("Failed to deserialize MasternodeCount");

    assert_eq!(count.total, 425);
    assert_eq!(count.stable, 420);
    assert_eq!(count.enabled, 418);
    assert_eq!(count.inqueue, 5);
    assert_eq!(count.ipv4, 380);
    assert_eq!(count.ipv6, 25);
    assert_eq!(count.onion, 20);
}

#[test]
fn test_deserialize_money_supply() {
    let json = load_fixture("money_supply.json");
    let supply: MoneySupply =
        serde_json::from_str(&json).expect("Failed to deserialize MoneySupply");

    assert_eq!(supply.update, 1704240000);
    assert_eq!(supply.transparentsupply, 45000000.0);
    assert_eq!(supply.shieldsupply, 5000000.0);
    assert_eq!(supply.totalsupply, 50000000.0);
}

#[test]
fn test_deserialize_vout() {
    let json = r#"{
        "value": 123.456,
        "n": 0,
        "scriptPubKey": {
            "asm": "OP_DUP OP_HASH160 abc123 OP_EQUALVERIFY OP_CHECKSIG",
            "hex": "76a914abc12388ac",
            "reqSigs": 1,
            "type": "pubkeyhash",
            "addresses": ["DMJRSsuU9zfyrvxVaAEFQqK4MxZg6vgeS6"]
        }
    }"#;

    let vout: Vout = serde_json::from_str(json).expect("Failed to deserialize Vout");

    assert_eq!(vout.value, 123.456);
    assert_eq!(vout.n, 0);
    assert_eq!(
        vout.script_pub_key.script_type,
        Some("pubkeyhash".to_string())
    );
    assert_eq!(vout.script_pub_key.req_sigs, Some(1));
}

#[test]
fn test_deserialize_vin_coinbase() {
    let json = r#"{
        "coinbase": "03e0930a",
        "sequence": 4294967295
    }"#;

    let vin: Vin = serde_json::from_str(json).expect("Failed to deserialize Vin::Coinbase");

    match vin {
        Vin::Coinbase(cb) => {
            assert_eq!(cb.coinbase, "03e0930a");
            assert_eq!(cb.sequence, 4294967295);
        }
        _ => panic!("Expected Coinbase variant"),
    }
}

#[test]
fn test_deserialize_raw_mempool_txids() {
    let json = r#"["txid1", "txid2", "txid3"]"#;

    let mempool: RawMemPool =
        serde_json::from_str(json).expect("Failed to deserialize RawMemPool::TxIds");

    match mempool {
        RawMemPool::TxIds(txids) => {
            assert_eq!(txids.len(), 3);
            assert_eq!(txids[0], "txid1");
        }
        _ => panic!("Expected TxIds variant"),
    }
}

#[test]
fn test_deserialize_get_txout_reply_null() {
    let json = "null";

    let reply: GetTxOutReply =
        serde_json::from_str(json).expect("Failed to deserialize GetTxOutReply::Null");

    match reply {
        GetTxOutReply::Null(_) => {}
        _ => panic!("Expected Null variant"),
    }
}
