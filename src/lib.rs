//! # PIVX RPC Client
//!
//! A Rust client library for interacting with PIVX Core's JSON-RPC interface.
//!
//! PIVX (Private Instant Verified Transaction) is a privacy-focused proof-of-stake
//! cryptocurrency. This library provides type-safe bindings to the PIVX Core RPC API,
//! enabling developers to build wallets, explorers, and other blockchain applications.
//!
//! ## Features
//!
//! - **Type-safe RPC calls**: All RPC methods return properly typed Rust structures
//! - **PIVX-specific support**: Includes PoS staking, masternodes, and Sapling shield transactions
//! - **Comprehensive coverage**: Supports block queries, transactions, wallet operations, and more
//!
//! ## Quick Start
//!
//! ```no_run
//! use pivx_rpc_rs::PivxRpcClient;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a client connected to local PIVX Core node
//! let client = PivxRpcClient::new(
//!     "http://127.0.0.1:51473".to_string(),
//!     Some("rpcuser".to_string()),
//!     Some("rpcpassword".to_string()),
//!     3,    // max_retries
//!     10,   // retry_delay_ms
//!     1000  // timeout_ms
//! );
//!
//! // Get the current block count
//! let block_count = client.getblockcount()?;
//! println!("Current block height: {}", block_count);
//!
//! // Get the best block hash
//! let best_hash = client.getbestblockhash()?;
//! println!("Best block hash: {}", best_hash);
//!
//! // Get full block details
//! let block = client.getblock(best_hash)?;
//! println!("Block has {} transactions", block.tx.len());
//! # Ok(())
//! # }
//! ```
//!
//! ## PIVX Core Compatibility
//!
//! This library is tested against PIVX Core 5.3 through 5.6. Some RPCs may not be available
//! in older versions. Always check your PIVX Core version compatibility.
//!
//! ## Examples
//!
//! See the `examples/` directory for complete usage examples including:
//! - Basic blockchain queries
//! - Masternode operations
//! - Staking status checks
//!
//! ## Note on Error Handling
//!
//! This library uses modern error handling with typed errors from
//! the `throttled_json_rpc` crate, providing clear and actionable error
//! messages for all RPC operations.

// Allow clippy warning from throttled_json_rpc macro
#![allow(clippy::assign_op_pattern)]

// Error handling now uses modern thiserror-based RpcError
// 
#[macro_use]
extern crate serde;
#[macro_use]
extern crate pivx_throttled_jsonrpc as throttled_json_rpc;


use std::collections::HashMap;

// Error type: All methods return Result<T, RpcError>
// where RpcError provides detailed error context

/// Raw hex-encoded blockchain data (blocks, transactions, etc.).
pub type SerializedData = String;

/// Block header information from PIVX Core.
///
/// Represents a block header with key metadata. Returned by RPCs like
/// `getblockheader`.
///
/// Does not include full transaction data - use [`FullBlock`] for that.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    /// Block hash (hex-encoded)
    pub hash: String,
    /// Number of confirmations for this block
    pub confirmations: i64,
    /// Block height in the blockchain
    pub height: i64,
    /// Block version number
    pub version: i32,
    /// Merkle root hash (hex-encoded)
    pub merkleroot: String,
    /// Block timestamp (Unix epoch seconds)
    pub time: i64,
    /// Median timestamp of the last 11 blocks
    pub mediantime: i64,
    /// Block nonce value
    pub nonce: i64,
    /// Compact representation of the difficulty target
    pub bits: String,
    /// Block difficulty
    pub difficulty: f64,
    /// Total chainwork (hex-encoded)
    pub chainwork: String,
    /// Accumulator checkpoint (PIVX-specific, hex-encoded)
    #[serde(default)]
    pub acc_checkpoint: String,
    /// Sapling shield pool value statistics
    #[serde(default)]
    pub shield_pool_value: ShieldPoolValue,
    /// Hash of the previous block, if any
    pub previousblockhash: Option<String>,
}

/// Complete block data including all transactions.
///
/// Returned by `getblock` RPC with verbosity level 2.
/// Contains full transaction details, unlike [`Block`] which only has header info.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FullBlock {
    /// Block hash (hex-encoded)
    pub hash: String,
    /// Number of confirmations
    pub confirmations: i32,
    /// Block size in bytes
    pub size: u32,
    /// Block height in the blockchain
    pub height: i64,
    /// Block version
    pub version: i32,
    /// Merkle root (hex-encoded)
    pub merkleroot: String,
    /// Accumulator checkpoint (PIVX-specific)
    #[serde(default)]
    pub acc_checkpoint: String,
    /// Final Sapling root (for shield transactions)
    #[serde(default)]
    pub finalsaplingroot: String,
    /// List of transactions in this block
    pub tx: Vec<Transaction>,
    /// Block timestamp (Unix epoch seconds)
    pub time: u32,
    /// Median time of last 11 blocks
    pub mediantime: u32,
    /// Nonce value
    pub nonce: i64,
    /// Difficulty target (compact format)
    pub bits: String,
    /// Block difficulty
    pub difficulty: f64,
    /// Total chainwork (hex-encoded)
    pub chainwork: String,
    /// Previous block hash
    pub previousblockhash: Option<String>,
    /// Next block hash (if not tip)
    pub nextblockhash: Option<String>,
    /// Stake modifier (PoS-specific)
    pub stakemodifier: Option<String>,
    /// Hash of proof-of-stake (PoS-specific)
    pub hashproofofstake: Option<String>,
}

/// PIVX transaction data.
///
/// Represents a transaction with inputs, outputs, and metadata.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    /// Transaction ID (hex-encoded)
    pub txid: Option<String>,
    /// Transaction version
    pub version: i32,
    /// Transaction type (PIVX-specific: 0=normal, etc.)
    #[serde(rename = "type")]
    pub tx_type: i32,
    /// Transaction size in bytes
    pub size: u32,
    /// Lock time (block height or timestamp)
    pub locktime: u32,
    /// Transaction inputs
    pub vin: Vec<Vin>,
    /// Transaction outputs
    pub vout: Vec<Vout>,
    /// Raw transaction hex
    pub hex: String,
    /// Block hash containing this transaction
    pub blockhash: Option<String>,
    /// Number of confirmations
    pub confirmations: Option<i32>,
    /// Transaction timestamp
    pub time: Option<i32>,
    /// Block timestamp
    pub blocktime: Option<i32>,
}

/// Detailed raw transaction information.
///
/// Extended transaction info returned by `getrawtransaction` with verbose flag.
/// Includes Sapling shield transaction data if applicable.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetRawTransactionInfo {
    /// Transaction ID
    pub txid: String,
    /// Transaction version
    pub version: u64,
    /// Transaction type
    pub r#type: u64,
    /// Transaction size in bytes
    pub size: u64,
    /// Lock time
    pub locktime: u64,
    /// Transaction inputs
    pub vin: Vec<VinTx>,
    /// Transaction outputs
    pub vout: Vec<Vout>,
    /// Raw transaction hex
    pub hex: String,
    /// Shield transaction value balance (PIV)
    pub value_balance: Option<f64>,
    /// Shield transaction value balance (satoshis)
    pub value_balance_sat: Option<u64>,
    /// Sapling spend descriptions
    pub vshield_spend: Option<Vec<VShieldSpend>>,
    /// Sapling output descriptions
    pub vshield_output: Option<Vec<VShieldOutput>>,
    /// Sapling binding signature
    pub binding_sig: Option<String>,
    /// List of shield addresses involved
    pub shielded_addresses: Option<Vec<String>>,
    /// Extra payload size (for special transactions)
    pub extra_payload_size: Option<u64>,
    /// Extra payload data (hex-encoded)
    pub extra_payload: Option<String>,
    /// Block hash containing this transaction
    pub blockhash: Option<String>,
    /// Number of confirmations
    pub confirmations: Option<u64>,
    /// Transaction time
    pub time: Option<u64>,
    /// Block time
    pub blocktime: Option<u64>,
}

/// Transaction detail for wallet transactions.
///
/// Represents details about how a transaction affects a wallet.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionDetail {
    /// Address involved in the transaction
    pub address: String,
    /// Category (send, receive, generate, etc.)
    pub category: String,
    /// Amount (PIV)
    pub amount: f64,
    /// Address label
    pub label: String,
    /// Output index
    pub vout: u64,
}

/// Sapling shield spend description.
///
/// Cryptographic proof data for a Sapling spend (spending from a shield address).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VShieldSpend {
    /// Value commitment
    #[serde(default)]
    pub cv: String,
    /// Anchor (root of note commitment tree)
    #[serde(default)]
    pub anchor: String,
    /// Nullifier (prevents double-spending)
    #[serde(default)]
    pub nullifier: String,
    /// Randomized public key
    #[serde(default)]
    pub rk: String,
    /// Zero-knowledge proof
    #[serde(default)]
    pub proof: String,
    /// Spend authorization signature
    #[serde(default)]
    pub spend_auth_sig: String,
}

/// Sapling shield output description.
///
/// Cryptographic data for creating a new Sapling note (receiving to a shield address).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VShieldOutput {
    /// Value commitment
    #[serde(default)]
    pub cv: String,
    /// Note commitment
    #[serde(default)]
    pub cmu: String,
    /// Ephemeral public key
    #[serde(default)]
    pub ephemeral_key: String,
    /// Encrypted note ciphertext
    #[serde(default)]
    pub enc_ciphertext: String,
    /// Encrypted outgoing ciphertext
    #[serde(default)]
    pub out_ciphertext: String,
    /// Zero-knowledge proof
    #[serde(default)]
    pub proof: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Vin {
    /// Coinbase transaction input (block reward)
    Coinbase(VinCoinbase),
    /// Regular transaction input (also covers coinstake)
    Tx(VinTx),
}

/// Transaction input referencing a previous output.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VinTx {
    /// Coinbase data (for coinbase inputs only)
    pub coinbase: Option<String>,
    /// Referenced transaction ID
    pub txid: Option<String>,
    /// Referenced output index
    pub vout: Option<i32>,
    /// Signature script
    pub script_sig: Option<ScriptSig>,
    /// Sequence number
    pub sequence: Option<i64>,
}

/// Coinbase transaction input (block reward).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VinCoinbase {
    /// Coinbase data (arbitrary data)
    pub coinbase: String,
    /// Sequence number
    pub sequence: i64,
}

/// Transaction output.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vout {
    /// Output value in PIV
    pub value: f64,
    /// Output index
    pub n: i32,
    /// Script public key (locking script)
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: ScriptPubKey,
}

/// Blockchain state and consensus information.
///
/// Returned by `getblockchaininfo` RPC.
#[derive(Serialize, Debug, serde::Deserialize)]
pub struct BlockChainInfo {
    /// Chain name (main, test, regtest)
    pub chain: String,
    /// Current number of blocks
    pub blocks: u64,
    /// Current number of headers
    pub headers: u64,
    /// Hash of the best (tip) block
    pub bestblockhash: String,
    /// Current difficulty
    pub difficulty: f64,
    /// Verification progress (0.0 to 1.0)
    pub verificationprogress: f64,
    /// Total chainwork (hex-encoded)
    pub chainwork: String,
    /// Sapling shield pool statistics
    #[serde(default)]
    pub shield_pool_value: ShieldPoolValue,
    /// Whether initial block download is in progress
    pub initial_block_downloading: bool,
    /// Softfork deployment status
    pub softforks: Vec<Softfork>,
    /// Network protocol upgrades status
    pub upgrades: Upgrades,
    /// Warning messages
    pub warnings: String,
}

/// Sapling shield pool value statistics.
///
/// Tracks the total value held in shield (private) addresses.
#[derive(Serialize, Debug, serde::Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ShieldPoolValue {
    /// Total value in shield pool (PIV)
    pub chain_value: f64,
    /// Change in shield pool value
    pub value_delta: f64,
}

/// Softfork deployment information.
///
/// Status of a backward-compatible protocol change.
#[derive(Serialize, Debug, serde::Deserialize)]
pub struct Softfork {
    /// Softfork identifier
    pub id: String,
    /// Version number
    pub version: u32,
    /// Rejection status
    pub reject: Reject,
}

/// Softfork rejection status.
#[derive(Serialize, Debug, serde::Deserialize)]
pub struct Reject {
    /// Whether blocks not following this softfork are rejected
    pub status: bool,
}

/// PIVX network protocol upgrades status.
///
/// Tracks activation status of major PIVX protocol upgrades.
/// Each field represents a specific network upgrade with its activation details.
#[derive(Serialize, Debug, serde::Deserialize)]
pub struct Upgrades {
    /// Proof-of-Stake activation
    #[serde(default)]
    #[serde(rename = "PoS")]
    pub pos: Upgrade,
    /// PoS version 2 upgrade
    #[serde(default)]
    #[serde(rename = "PoS v2")]
    pub pos_v2: Upgrade,
    /// Zerocoin protocol activation
    #[serde(default)]
    #[serde(rename = "Zerocoin")]
    pub zerocoin: Upgrade,
    /// Zerocoin version 2
    #[serde(default)]
    #[serde(rename = "Zerocoin v2")]
    pub zerocoin_v2: Upgrade,
    /// BIP65 (CHECKLOCKTIMEVERIFY) activation
    #[serde(default)]
    #[serde(rename = "BIP65")]
    pub bip65: Upgrade,
    /// Public Zerocoin spends
    #[serde(default)]
    #[serde(rename = "Zerocoin Public")]
    pub zerocoin_public: Upgrade,
    /// PIVX v3.4 upgrade
    #[serde(default)]
    #[serde(rename = "PIVX v3.4")]
    pub pivx_v3_4: Upgrade,
    /// PIVX v4.0 upgrade
    #[serde(default)]
    #[serde(rename = "PIVX v4.0")]
    pub pivx_v4_0: Upgrade,
    /// Sapling shield transactions activation
    #[serde(default)]
    #[serde(rename = "v5 shield")]
    pub v5_shield: Upgrade,
    /// PIVX v5.2 upgrade
    #[serde(default)]
    #[serde(rename = "PIVX v5.2")]
    pub pivx_v5_2: Upgrade,
    /// PIVX v5.3 upgrade
    #[serde(default)]
    #[serde(rename = "PIVX v5.3")]
    pub pivx_v5_3: Upgrade,
    /// PIVX v5.5 upgrade
    #[serde(default)]
    #[serde(rename = "PIVX v5.5")]
    pub pivx_v5_5: Upgrade,
}

/// Protocol upgrade activation details.
#[derive(Serialize, Debug, serde::Deserialize, Default)]
pub struct Upgrade {
    /// Block height at which upgrade activates
    pub activationheight: u64,
    /// Activation status (active, pending, defined, etc.)
    pub status: String,
    /// Additional information about the upgrade
    pub info: String,
}

/// Blockchain tip information.
///
/// Information about a chain tip (potential best block).
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tip {
    /// Block height
    pub height: i32,
    /// Block hash
    pub hash: String,
    /// Length of branch (0 for main chain)
    pub branchlen: i32,
    /// Status (active, valid-fork, valid-headers, headers-only, invalid)
    pub status: String,
}

/// Memory pool information.
///
/// Statistics about unconfirmed transactions in the mempool.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MemPoolInfo {
    /// Whether mempool is loaded
    pub loaded: bool,
    /// Number of transactions in mempool
    pub size: i32,
    /// Total size of mempool in bytes
    pub bytes: i32,
    /// Memory usage of mempool
    pub usage: i32,
    /// Minimum fee rate for mempool acceptance
    pub mempoolminfee: i32,
    /// Minimum fee rate for relay
    pub minrelaytxfee: i32,
}

/// Script public key (locking script).
///
/// Defines the conditions that must be met to spend an output.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScriptPubKey {
    /// Human-readable script assembly
    pub asm: String,
    /// Hex-encoded script
    pub hex: String,
    /// Required number of signatures
    #[serde(rename = "reqSigs")]
    pub req_sigs: Option<i64>,
    /// Script type (pubkey, pubkeyhash, scripthash, multisig, etc.)
    #[serde(rename = "type")]
    pub script_type: Option<String>,
    /// Addresses associated with this script
    pub addresses: Option<Vec<String>>,
}

/// Script signature (unlocking script).
///
/// Provides data to satisfy the locking script conditions.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScriptSig {
    /// Human-readable script assembly
    pub asm: String,
    /// Hex-encoded script
    pub hex: String,
}

/// Unspent transaction output details.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TxOut {
    /// Hash of the block containing this output
    pub bestblock: String,
    /// Number of confirmations
    pub confirmations: i32,
    /// Output value in PIV
    pub value: f64,
    /// Script public key
    pub script_pub_key: ScriptPubKey,
    /// Whether this is a coinbase output
    pub coinbase: bool,
}

/// Response from `gettxout` RPC.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum GetTxOutReply {
    /// Output doesn't exist or is already spent
    Null(()),
    /// UTXO details
    TxOut(TxOut),
}

/// UTXO set statistics.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TxOutSetInfo {
    /// Block height of the UTXO set
    pub height: u32,
    /// Hash of the block at this height
    pub bestblock: String,
    /// Number of transactions in UTXO set
    pub transactions: u32,
    /// Number of unspent outputs
    pub txouts: u32,
    /// Serialized hash of UTXO set
    pub hash_serialized_2: String,
    /// Total amount of PIV in UTXO set
    pub total_amount: f64,
    /// Size of UTXO set on disk (bytes)
    pub disk_size: u32,
}

/// Mempool transaction details.
///
/// Detailed information about a transaction in the memory pool.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MemPoolTx {
    /// Transaction size in bytes
    pub size: serde_json::Number,
    /// Transaction fee
    pub fee: serde_json::Number,
    /// Modified fee (includes descendants)
    pub modifiedfee: serde_json::Number,
    /// Time transaction entered mempool
    pub time: serde_json::Number,
    /// Block height when transaction entered mempool
    pub height: serde_json::Number,
    /// Number of descendant transactions
    pub descendantcount: serde_json::Number,
    /// Total size of descendant transactions
    pub descendantsize: serde_json::Number,
    /// Total fees of descendant transactions
    pub descendantfees: serde_json::Number,
    /// Number of ancestor transactions
    pub ancestorcount: serde_json::Number,
    /// Total size of ancestor transactions
    pub ancestorsize: serde_json::Number,
    /// Total fees of ancestor transactions
    pub ancestorfees: serde_json::Number,
    /// Witness transaction ID
    pub wtxid: String,
    /// Transaction IDs this transaction depends on
    pub depends: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum RawMemPool {
    /// Verbose mode: returns detailed transaction information
    Verbose(HashMap<String, MemPoolTx>),
    /// Non-verbose mode: returns only transaction IDs
    TxIds(Vec<String>),
}

/// Transaction input for raw transaction creation.
///
/// Specifies which output to spend in a new transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxInput {
    /// Transaction ID of the output to spend
    pub txid: String,
    /// Output index to spend
    pub vout: i32,
    /// Sequence number (optional)
    #[serde(rename = "Sequence")]
    pub sequence: Option<u32>,
}

/// Transaction output for signing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxOutput {
    /// Transaction ID
    pub txid: String,
    /// Output index
    pub vout: i32,
    /// Script public key (hex)
    pub script_pub_key: String,
    /// Redeem script for P2SH outputs (hex, optional)
    pub redeem_script: Option<String>,
    /// Output amount in PIV
    pub amount: f64,
}

/// Signed transaction result.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignedTx {
    /// Signed transaction in hex format
    pub hex: String,
    /// Whether the transaction is completely signed
    pub complete: bool,
}

/// Masternode information.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MasternodeList {
    /// Masternode rank
    pub rank: i32,
    /// Masternode type
    #[serde(rename = "type")]
    pub mn_type: String,
    /// Network type (IPv4, IPv6, Tor)
    pub network: String,
    /// Collateral transaction hash
    pub txhash: String,
    /// Collateral output index
    pub outidx: i8,
    /// Masternode public key
    pub pubkey: String,
    /// Masternode status
    pub status: String,
    /// Masternode IP address and port
    pub addr: String,
    /// Protocol version
    pub version: serde_json::Number,
    /// Last time masternode was seen
    pub lastseen: serde_json::Number,
    /// Time masternode has been active
    pub activetime: serde_json::Number,
    /// Last payment amount received
    pub lastpaid: f64,
}

/// PIVX staking status.
///
/// Comprehensive status information about PoS staking.
/// Returned by `getstakingstatus` RPC.
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct PivxStatus {
    /// Whether staking is currently active
    pub staking_status: bool,
    /// Whether staking is enabled in configuration
    pub staking_enabled: bool,
    /// Whether cold staking is enabled
    pub coldstaking_enabled: bool,
    /// Whether node has network connections
    pub haveconnections: bool,
    /// Whether masternode sync is complete
    pub mnsync: bool,
    /// Whether wallet is unlocked for staking
    pub walletunlocked: bool,
    /// Number of stakeable coins
    pub stakeablecoins: u64,
    /// Balance available for staking (PIV)
    pub stakingbalance: f64,
    /// Stake split threshold (PIV)
    pub stakesplitthreshold: f64,
    /// Age of last stake attempt (seconds)
    pub lastattempt_age: i64,
    /// Depth of last stake attempt (blocks)
    pub lastattempt_depth: i64,
    /// Hash of block for last stake attempt
    pub lastattempt_hash: String,
    /// Number of coins in last stake attempt
    pub lastattempt_coins: u64,
    /// Number of stake attempts
    pub lastattempt_tries: i64,
}

/// Masternode count statistics.
///
/// Summary counts of masternodes by status and network type.
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct MasternodeCount {
    /// Total number of masternodes
    pub total: i32,
    /// Number of stable masternodes
    pub stable: i32,
    /// Number of enabled masternodes
    pub enabled: i32,
    /// Number of masternodes in activation queue
    pub inqueue: i32,
    /// Number of IPv4 masternodes
    pub ipv4: i32,
    /// Number of IPv6 masternodes
    pub ipv6: i32,
    /// Number of Tor (.onion) masternodes
    pub onion: i32,
}

/// General node and wallet information.
///
/// Comprehensive status returned by `getinfo` RPC.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetInfo {
    /// Node software version number
    pub version: i32,
    /// Network protocol version
    pub protocolversion: i32,
    /// Services provided by this node
    pub services: String,
    /// Wallet version number
    pub walletversion: i32,
    /// Total wallet balance (PIV)
    pub balance: f64,
    /// Current staking status
    #[serde(default)]
    #[serde(rename = "staking status")]
    pub staking_status: String,
    /// Number of blocks in local blockchain
    pub blocks: i32,
    /// Time offset from system clock
    #[serde(default)]
    pub timeoffset: i32,
    /// Number of peer connections
    pub connections: i32,
    /// Proxy server setting
    pub proxy: String,
    /// Current network difficulty
    pub difficulty: f64,
    /// Whether node is on testnet
    pub testnet: bool,
    /// Total money supply (PIV)
    #[serde(default)]
    pub moneysupply: f64,
    /// Transparent (non-shielded) supply (PIV)
    #[serde(default)]
    pub transparentsupply: f64,
    /// Shielded (privacy) supply (PIV)
    #[serde(default)]
    pub shieldsupply: f64,
    /// Timestamp of oldest key in pool
    pub keypoololdest: i64,
    /// Number of keys in keypool
    pub keypoolsize: i32,
    /// Transaction fee setting (PIV/kB)
    pub paytxfee: f64,
    /// Minimum relay fee (PIV/kB)
    pub relayfee: f64,
    /// Any error messages
    pub errors: String,
}

/// Budget proposal information.
///
/// Note: Field names use Title Case per PIVX RPC convention.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BudgetInfo {
    /// Proposal name
    #[serde(rename = "Name")]
    pub name: String,
    /// Proposal information URL
    #[serde(rename = "URL")]
    pub url: String,
    /// Proposal hash
    #[serde(rename = "Hash")]
    pub hash: String,
    /// Fee transaction hash
    #[serde(rename = "FeeHash")]
    pub fee_hash: String,
    /// Starting block for payments
    #[serde(rename = "BlockStart")]
    pub block_start: u32,
    /// Ending block for payments
    #[serde(rename = "BlockEnd")]
    pub block_end: u32,
    /// Total number of scheduled payments
    #[serde(rename = "TotalPaymentCount")]
    pub total_payment_count: u32,
    /// Number of remaining payments
    #[serde(rename = "RemainingPaymentCount")]
    pub remaining_payment_count: u32,
    /// Address to receive payments
    #[serde(rename = "PaymentAddress")]
    pub payment_address: String,
    /// Vote ratio (Yes / Total)
    #[serde(rename = "Ratio")]
    pub ratio: f64,
    /// Number of yes votes
    #[serde(rename = "Yeas")]
    pub yeas: u32,
    /// Number of no votes
    #[serde(rename = "Nays")]
    pub nays: u32,
    /// Number of abstain votes
    #[serde(rename = "Abstains")]
    pub abstains: u32,
    /// Total payment amount (PIV)
    #[serde(rename = "TotalPayment")]
    pub total_payment: f64,
    /// Monthly payment amount (PIV)
    #[serde(rename = "MonthlyPayment")]
    pub monthly_payment: f64,
    /// Whether proposal is established
    #[serde(rename = "IsEstablished")]
    pub is_established: bool,
    /// Whether proposal is valid
    #[serde(rename = "IsValid")]
    pub is_valid: bool,
    /// Amount allotted (PIV)
    #[serde(rename = "Allotted")]
    pub allotted: f64,
}

/// Cold staking UTXO.
#[derive(Debug, Deserialize, Serialize)]
pub struct ColdUtxo {
    /// Transaction ID containing the UTXO
    pub txid: String,
    /// Output index in transaction
    pub txidn: u32,
    /// Amount of PIV in UTXO
    pub amount: f64,
    /// Number of confirmations
    pub confirmations: u32,
    /// Address of the cold staker
    #[serde(rename = "cold-staker")]
    pub cold_staker: String,
    /// Address of the coin owner
    #[serde(rename = "coin-owner")]
    pub coin_owner: String,
    /// Whether staker is whitelisted
    pub whitelisted: bool,
}

/// List of cold staking UTXOs.
#[derive(Debug, Deserialize, Serialize)]
pub struct ListColdUtxos {
    /// Vector of cold staking UTXOs
    pub coldutxos: Vec<ColdUtxo>,
}

/// PIVX money supply information.
///
/// Breakdown of total supply by transparency type.
#[derive(Debug, Deserialize, Serialize)]
pub struct MoneySupply {
    /// Last update timestamp (Unix epoch)
    pub update: i64,
    /// Transparent supply (PIV)
    pub transparentsupply: f64,
    /// Shielded supply (PIV)
    pub shieldsupply: f64,
    /// Total supply (PIV)
    pub totalsupply: f64,
}

// PIVX JSON-RPC Client
//
// Primary interface for communicating with a PIVX Core node via JSON-RPC.
//
// Connection Example:
// ```
// use pivx_rpc_rs::PivxRpcClient;
//
// let client = PivxRpcClient::new(
//     "http://localhost:51473".to_string(),
//     Some("rpcuser".to_string()),
//     Some("rpcpassword".to_string())
// );
// ```
//
// All methods return `Result<T>` where the error type is from the
// `throttled_json_rpc` crate using the `failure` error handling pattern.
//
// See individual method documentation below for usage details.
jsonrpc_client!(pub struct PivxRpcClient {
single:
    /// Creates a raw transaction from inputs and outputs.
    ///
    /// # Arguments
    /// * `inputs` - Transaction inputs to spend
    /// * `outputs` - Map of addresses to amounts (PIV)
    /// * `locktime` - Optional locktime for transaction
    ///
    /// # Returns
    /// Hex-encoded raw transaction
    pub fn createrawtransaction(&self, inputs: &[TxInput], outputs: &HashMap<&str, f64>, locktime: Option<u32>) -> Result<String>;

    /// Exports the private key for an address.
    ///
    /// **WARNING**: Requires wallet to be unlocked. Handle with care.
    ///
    /// # Arguments
    /// * `address` - The PIVX address
    ///
    /// # Returns
    /// WIF-encoded private key
    pub fn dumpprivkey(&self, address: &str) -> Result<String>;

    /// Adds an address to the cold staking delegator whitelist.
    ///
    /// # Arguments
    /// * `address` - Address to whitelist as delegator
    /// * `label` - Optional label for the address
    ///
    /// # Returns
    /// `true` if successfully added
    pub fn delegatoradd(&self, address: &str, label: Option<&str>) -> Result<bool>;

    /// Generates blocks immediately (regtest mode only).
    ///
    /// # Arguments
    /// * `number` - Number of blocks to generate
    /// * `iterations` - Optional max iterations per block
    ///
    /// # Returns
    /// Hashes of generated blocks
    pub fn generate(&self, number: usize, iterations: Option<usize>) -> Result<Vec<String>>;

    /// Returns the hash of the best (tip) block in the longest blockchain.
    pub fn getbestblockhash(&self) -> Result<String>;

    /// Returns general information about the node and wallet.
    pub fn getinfo(&self) -> Result<GetInfo>;

    /// Returns comprehensive blockchain state information.
    pub fn getblockchaininfo(&self) -> Result<BlockChainInfo>;

    /// Returns the height of the most-work fully-validated chain.
    pub fn getblockcount(&self) -> Result<i64>;

    /// Returns full block data including transactions.
    ///
    /// # Arguments
    /// * `block_hash` - Hash of the block to retrieve
    pub fn getblock(&self, block_hash: String) -> Result<FullBlock>;

    /// Returns the block hash for a given height.
    ///
    /// # Arguments
    /// * `block_height` - Height in the main chain
    pub fn getblockhash(&self, block_height: i64) -> Result<String>;

    /// Returns block header information.
    ///
    /// # Arguments
    /// * `block_hash` - Hash of the block
    pub fn getblockheader(&self, block_hash: String) -> Result<Block>;

    /// Returns information about all budget proposals.
    pub fn getbudgetinfo(&self) -> Result<Vec<BudgetInfo>>;

    /// Returns masternode count by various categories.
    pub fn getmasternodecount(&self) -> Result<MasternodeCount>;

    /// Generates a new address for receiving payments.
    ///
    /// # Arguments
    /// * `account` - Optional account name (deprecated in newer versions)
    /// * `address_type` - Optional address type (e.g., "legacy")
    pub fn getnewaddress(&self, account: Option<&str>, address_type: Option<&str>) -> Result<String>;

    /// Returns all transaction IDs in the memory pool.
    ///
    /// # Arguments
    /// * `format` - If `true`, returns verbose format; if `false`, returns transaction IDs only
    pub fn getrawmempool(&self, format: bool) -> Result<RawMemPool>;

    /// Returns raw transaction information.
    ///
    /// # Arguments
    /// * `txid` - The transaction ID
    /// * `verbose` - If `true`, returns detailed info; if `false`, returns hex string
    pub fn getrawtransaction(&self, txid: String, verbose: bool) -> Result<GetRawTransactionInfo>;

    /// Returns money supply information.
    ///
    /// # Arguments
    /// * `update` - Whether to update cached values
    pub fn getsupplyinfo(&self, update: bool) -> Result<MoneySupply>;

    /// Returns list of masternodes.
    ///
    /// # Arguments
    /// * `mn_addr` - Optional masternode address to filter by
    pub fn listmasternodes(&self, mn_addr: Option<&str>) -> Result<Vec<MasternodeList>>;

    /// Returns list of cold staking UTXOs.
    pub fn listcoldutxos(&self) -> Result<Vec<ListColdUtxos>>;

    /// Submits a raw transaction to the network.
    ///
    /// # Arguments
    /// * `transaction` - Hex-encoded signed transaction
    /// * `allow_high_fee` - Allow high fees (default: false)
    ///
    /// # Returns
    /// Transaction ID if successful
    pub fn sendrawtransaction(&self, transaction: &str, allow_high_fee: Option<bool>) -> Result<String>;

    /// Sends PIV to an address.
    ///
    /// **NOTE**: Requires wallet to be unlocked.
    ///
    /// # Arguments
    /// * `address` - Destination PIVX address
    /// * `amount` - Amount to send (PIV)
    /// * `comment` - Optional comment for transaction
    /// * `comment_to` - Optional comment about recipient
    /// * `include_fee` - Whether to deduct fee from amount
    ///
    /// # Returns
    /// Transaction ID
    pub fn sendtoaddress(&self, address: &str, amount: f64, comment: Option<&str>, comment_to: Option<&str>, include_fee: Option<bool>) -> Result<String>;

    /// Signs inputs for a raw transaction.
    ///
    /// # Arguments
    /// * `transaction` - Hex-encoded transaction
    /// * `outputs` - Optional previous outputs
    /// * `privkeys` - Optional private keys for signing
    /// * `sig_hash_type` - Optional signature hash type (default: "ALL")
    pub fn signrawtransaction(&self, transaction: &str, outputs: Option<&[TxOutput]>, privkeys: Option<&[&str]>, sig_hash_type: Option<&str>) -> Result<SignedTx>;

    /// Returns details about an unspent transaction output.
    ///
    /// # Arguments
    /// * `txid` - Transaction ID
    /// * `vout` - Output index
    /// * `unconfirmed` - Whether to include unconfirmed transactions
    ///
    /// # Returns
    /// `Some(TxOut)` if output exists and is unspent, `None` otherwise
    pub fn gettxout(&self, txid: &str, vout: u32, unconfirmed: bool) -> Result<Option<TxOut>>;

    /// Returns PoS staking status.
    pub fn getstakingstatus(&self) -> Result<PivxStatus>;

    /// Relays a masternode broadcast message.
    ///
    /// # Arguments
    /// * `hex_mnb` - Hex-encoded masternode broadcast
    pub fn relaymasternodebroadcast(&self, hex_mnb: &str) -> Result<String>;
enum:
    /// Returns block information in various formats.
    ///
    /// The return type depends on verbosity:
    /// - `Zero(SerializedData)`: Raw block hex
    /// - `One(Block)`: Block header only
    /// - `Two(FullBlock)`: Full block with transactions
    pub fn getblockinfo(&self) -> Result<Zero(SerializedData)|One(Block)|Two(FullBlock)>;
});
