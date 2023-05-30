# pivx-rpc-rs
A Rust library for interacting with the PIVX Core RPC and it defines several data structures.

## Data Structures

- `Block`: Represents a block in a blockchain with various properties such as `hash`, `confirmations`, `height`, and more.
- `FullBlock`: Similar to `Block` but with additional properties like `size`, `tx`, and `nextblockhash`.
- `Transaction`: Represents a transaction with properties such as `txid`, `version`, `size`, `vin`, and `vout`.
- `Vin`: Represents an input to a transaction, which can be either a `Coinbase` or `Tx` input.
- `Vout`: Represents an output of a transaction with properties like `value` and `script_pub_key`.
- `BlockChainInfo`: Contains information about the blockchain such as `chain`, `blocks`, `headers`, and more.
- `ShieldPoolValue`: Represents the value of the shield pool with properties `chainValue` and `valueDelta`.
- `Softfork`: Represents a soft fork with properties like `id`, `version`, and `reject`.
- `Upgrades`: Contains information about various upgrades with properties like `pos`, `pos_v2`, and more.
- `Tip`: Represents a tip of the blockchain with properties like `height`, `hash`, `branchlen`, and `status`.
- `MemPoolInfo`: Contains information about the mempool such as `loaded`, `size`, `bytes`, and more.
- `ScriptPubKey`: Represents the script public key with properties like `asm`, `hex`, and `req_sigs`.
- `ScriptSig`: Represents the script signature with properties like `asm` and `hex`.
- `TxOut`: Represents a transaction output with properties like `bestblock`, `confirmations`, `value`, and more.
- `GetTxOutReply`: Represents the reply from the `gettxout` RPC call, which can be either `Null` or `TxOut`.
- `TxOutSetInfo`: Contains information about the transaction output set with properties like `height`, `bestblock`, and more.
- `MemPoolTx`: Represents a transaction in the mempool with properties like `size`, `fee`, `modifiedfee`, and more.
- `RawMemPool`: Represents the raw mempool response, which can be either `True` or `False`. (Needs updating)
- `TxInput`: Represents an input to a transaction with properties like `txid`, `vout`, and `sequence`.
- `TxOutput`: Represents an output of a transaction with properties like `txid`, `vout`, `script_pub_key`, and more.
- `SignedTx`: Represents a signed transaction with properties like `hex` and `complete`.
- `MasternodeList`: Represents a masternode with properties like `rank`, `mn_type`, `network`, and more.
- `PivxStatus`: Contains various status properties like `staking_status`, `staking_enabled`, and more.
- `MasternodeCount`: Contains the count of masternodes with properties like `total`, `stable`, `enabled`, and more.
- `GetInfo`: Contains information about the node with properties like `version`, `protocolversion`, `services`, and more.
- `BudgetInfo`: Represents budget information with properties like `name`, `url`, `hash`, and more.
- `ColdUtxo`: Represents a cold UTXO with properties like `txid`, `txidn`, `amount`, and more.
- `ListColdUtxos`: Represents a list of cold UTXOs.

## RPC Client

The RPC client module provides functions to interact with a remote RPC server. It allows sending RPC commands and retrieving responses from the server.

### Features

- Supports authentication with username and password.
- Handles JSON-RPC requests and responses.
- Provides convenient methods for common RPC commands.

### Example Usage

Here's an example of how to use the RPC client module:

```rust
use pivx_rpc_rs;

use pivx_rpc_rs::FullBlock;
use pivx_rpc_rs::BitcoinRpcClient;

fn main() {
    //Rpc settings
    let rpchost = String::from("http://127.0.0.1:51475");
    let rpcuser = String::from("rpcuser");
    let rpcpass = String::from("rpcpass");

    let client = BitcoinRpcClient::new(
        rpchost,
        Some(rpcuser),
        Some(rpcpass),
        3,
        10,
        1000
    );
    
    let block_hash = client.getbestblockhash();
    let block_info = client.getblock(block_hash.unwrap());
    println!("{:#?}",&block_info);
}
```


