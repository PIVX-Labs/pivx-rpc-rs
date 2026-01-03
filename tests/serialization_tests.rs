//! Basic serialization tests
//!
//! These tests verify that types can be serialized and deserialized.

use pivx_rpc_rs::*;

#[test]
fn test_serialize_masternode_count() {
    let count = MasternodeCount {
        total: 425,
        stable: 420,
        enabled: 418,
        inqueue: 5,
        ipv4: 380,
        ipv6: 25,
        onion: 20,
    };

    let json = serde_json::to_string(&count).expect("Failed to serialize");
    assert!(json.contains("425"));
    assert!(json.contains("420"));
}

#[test]
fn test_serialize_money_supply() {
    let supply = MoneySupply {
        update: 1704240000,
        transparentsupply: 45000000.0,
        shieldsupply: 5000000.0,
        totalsupply: 50000000.0,
    };

    let json = serde_json::to_string(&supply).expect("Failed to serialize");
    assert!(json.contains("1704240000"));
    assert!(json.contains("45000000"));
}

#[test]
fn test_serialize_tx_input() {
    let input = TxInput {
        txid: "abcdef1234567890".to_string(),
        vout: 0,
        sequence: Some(4294967294),
    };

    let json = serde_json::to_string(&input).expect("Failed to serialize");
    let deserialized: TxInput = serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(input.txid, deserialized.txid);
    assert_eq!(input.vout, deserialized.vout);
}

#[test]
fn test_serialize_signed_tx() {
    let signed = SignedTx {
        hex: "0200000001abcdef".to_string(),
        complete: true,
    };

    let json = serde_json::to_string(&signed).expect("Failed to serialize");
    let deserialized: SignedTx = serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(signed.hex, deserialized.hex);
    assert_eq!(signed.complete, deserialized.complete);
}

#[test]
fn test_serialize_cold_utxo() {
    let utxo = ColdUtxo {
        txid: "abcdef1234567890".to_string(),
        txidn: 0,
        amount: 100.5,
        confirmations: 10,
        cold_staker: "DMJRSsuU9zfyrvxVaAEFQqK4MxZg6vgeS6".to_string(),
        coin_owner: "D7VFR83SQbiezrW72hjcWJtcfip5krte2Z".to_string(),
        whitelisted: true,
    };

    let json = serde_json::to_string(&utxo).expect("Failed to serialize");
    let deserialized: ColdUtxo = serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(utxo.txid, deserialized.txid);
    assert_eq!(utxo.amount, deserialized.amount);
    assert_eq!(utxo.whitelisted, deserialized.whitelisted);
}

#[test]
fn test_vin_coinbase_serialization() {
    let vin = Vin::Coinbase(VinCoinbase {
        coinbase: "03e0930a".to_string(),
        sequence: 4294967295,
    });

    let json = serde_json::to_string(&vin).expect("Failed to serialize");
    let deserialized: Vin = serde_json::from_str(&json).expect("Failed to deserialize");

    match (vin, deserialized) {
        (Vin::Coinbase(orig), Vin::Coinbase(new)) => {
            assert_eq!(orig.coinbase, new.coinbase);
            assert_eq!(orig.sequence, new.sequence);
        }
        _ => panic!("Type mismatch"),
    }
}

#[test]
fn test_raw_mempool_txids_serialization() {
    let mempool = RawMemPool::TxIds(vec![
        "txid1".to_string(),
        "txid2".to_string(),
        "txid3".to_string(),
    ]);

    let json = serde_json::to_string(&mempool).expect("Failed to serialize");
    let deserialized: RawMemPool = serde_json::from_str(&json).expect("Failed to deserialize");

    match (mempool, deserialized) {
        (RawMemPool::TxIds(orig), RawMemPool::TxIds(new)) => {
            assert_eq!(orig.len(), new.len());
            assert_eq!(orig[0], new[0]);
        }
        _ => panic!("Type mismatch"),
    }
}

#[test]
fn test_get_txout_reply_null_serialization() {
    let reply = GetTxOutReply::Null(());

    let json = serde_json::to_string(&reply).expect("Failed to serialize");
    let deserialized: GetTxOutReply = serde_json::from_str(&json).expect("Failed to deserialize");

    match (reply, deserialized) {
        (GetTxOutReply::Null(_), GetTxOutReply::Null(_)) => {}
        _ => panic!("Type mismatch"),
    }
}
