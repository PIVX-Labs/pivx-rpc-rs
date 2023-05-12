#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate throttled_json_rpc;

use std::collections::HashMap;

pub type SerializedData = String;

#[derive(Deserialize, Clone, Debug)]
pub struct Block {
    pub hash: String,
    pub confirmations: i32,
    pub height: i32,
    pub version: i32,
    pub merkleroot: String,
    pub time: i32,
    pub mediantime: i32,
    pub nonce: i64,
    pub bits: String,
    pub difficulty: f32,
    pub chainwork: String,
    pub acc_checkpoint: String,
    pub shield_pool_value: ShieldPoolValue,
    pub previousblockhash: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct FullBlock {
    pub hash: String,
    pub confirmations: i32,
    pub size: u32,
    pub height: i64,
    pub version: i32,
    pub merkleroot: String,
    pub acc_checkpoint: String,
    pub finalsaplingroot: String,
    pub tx: Vec<String>,
    pub time: u32,
    pub mediantime: u32,
    pub nonce: i32,
    pub bits: String,
    pub difficulty: f32,
    pub chainwork: String,
    pub previousblockhash: Option<String>,
    pub nextblockhash: Option<String>,
    pub stakemodifier: Option<String>,
    pub hashproofofstake: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Transaction {
    pub txid: String,
    pub version: i32,
    #[serde(rename = "type")]
    pub tx_type: i32,
    pub size: u32,
    pub locktime: u32,
    pub vin: Vec<Vin>,
    pub vout: Vec<Vout>,
    pub hex: String,
    pub blockhash: Option<String>,
    pub confirmations: Option<i32>,
    pub time: Option<i32>,
    pub blocktime: Option<i32>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Vin {
    Coinbase(VinCoinbase),
    Coinstake(VinTx),
    Tx(VinTx),
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VinTx {
    pub txid: String,
    pub vout: i32,
    pub script_sig: ScriptSig,
    pub sequence: i32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct VinCoinbase {
    pub coinbase: String,
    pub sequence: i32,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vout {
    pub value: f32,
    pub n: i32,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: ScriptPubKey,
}

#[derive(Debug, serde::Deserialize)]
pub struct BlockChainInfo {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub bestblockhash: String,
    pub difficulty: f64,
    pub verificationprogress: f64,
    pub chainwork: String,
    pub shield_pool_value: ShieldPoolValue,
    pub initial_block_downloading: bool,
    pub softforks: Vec<Softfork>,
    pub upgrades: Upgrades,
    pub warnings: String,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct ShieldPoolValue {
    pub chainValue: f64,
    pub valueDelta: f64,
}

#[derive(Debug, serde::Deserialize)]
pub struct Softfork {
    pub id: String,
    pub version: u32,
    pub reject: Reject,
}

#[derive(Debug, serde::Deserialize)]
pub struct Reject {
    pub status: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct Upgrades {
    #[serde(rename = "PoS")]
    pub pos: Upgrade,
    #[serde(rename = "PoS v2")]
    pub pos_v2: Upgrade,
    pub Zerocoin: Upgrade,
    #[serde(rename = "Zerocoin v2")]
    pub zerocoin_v2: Upgrade,
    #[serde(rename = "BIP65")]
    pub bip65: Upgrade,
    #[serde(rename = "Zerocoin Public")]
    pub zerocoin_public: Upgrade,
    #[serde(rename = "PIVX v3.4")]
    pub pivx_v3_4: Upgrade,
    #[serde(rename = "PIVX v4.0")]
    pub pivx_v4_0: Upgrade,
    #[serde(rename = "v5 shield")]
    pub v5_shield: Upgrade,
    #[serde(rename = "PIVX v5.2")]
    pub pivx_v5_2: Upgrade,
    #[serde(rename = "PIVX v5.3")]
    pub pivx_v5_3: Upgrade,
    #[serde(rename = "PIVX v5.5")]
    pub pivx_v5_5: Upgrade,
}

#[derive(Debug, serde::Deserialize)]
pub struct Upgrade {
    pub activationheight: u64,
    pub status: String,
    pub info: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Tip {
    pub height: i32,
    pub hash: String,
    pub branchlen: i32,
    pub status: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MemPoolInfo {
    pub loaded: bool,
    pub size: i32,
    pub bytes: i32,
    pub usage: i32,
    pub mempoolminfee: i32,
    pub minrelaytxfee: i32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ScriptPubKey {
    pub asm: String,
    pub hex: String,
    #[serde(rename = "reqSigs")]
    pub req_sigs: Option<i64>,
    #[serde(rename = "type")]
    pub script_type: String,
    pub addresses: Option<Vec<String>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ScriptSig {
    pub asm: String,
    pub hex: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TxOut {
    pub bestblock: String,
    pub confirmations: i32,
    pub value: f32,
    pub script_pub_key: ScriptPubKey,
    pub coinbase: bool,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum GetTxOutReply {
    Null(()),
    TxOut(TxOut),
}

#[derive(Deserialize, Clone, Debug)]
pub struct TxOutSetInfo {
    pub height: u32,
    pub bestblock: String,
    pub transactions: u32,
    pub txouts: u32,
    pub hash_serialized_2: String,
    pub total_amount: f32,
    pub disk_size: u32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MemPoolTx {
    pub size: serde_json::Number,
    pub fee: serde_json::Number,
    pub modifiedfee: serde_json::Number,
    pub time: serde_json::Number,
    pub height: serde_json::Number,
    pub descendantcount: serde_json::Number,
    pub descendantsize: serde_json::Number,
    pub descendantfees: serde_json::Number,
    pub ancestorcount: serde_json::Number,
    pub ancestorsize: serde_json::Number,
    pub ancestorfees: serde_json::Number,
    pub wtxid: String,
    pub depends: Vec<String>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum RawMemPool {
    True(HashMap<String, MemPoolTx>),
    False(Vec<String>),
}

#[derive(Serialize, Clone, Debug)]
pub struct TxInput {
    pub txid: String,
    pub vout: i32,
    #[serde(rename = "Sequence")]
    pub sequence: Option<u32>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TxOutput {
    pub txid: String,
    pub vout: i32,
    pub script_pub_key: String,
    pub redeem_script: Option<String>,
    pub amount: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SignedTx {
    pub hex: String,
    pub complete: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MasternodeList {
    pub rank: i32,
    #[serde(rename = "type")]
    pub mn_type: String,
    pub network: String,
    pub txhash: String,
    pub outidx: i8,
    pub pubkey: String,
    pub status: String,
    pub addr: String,
    pub version: serde_json::Number,
    pub lastseen: serde_json::Number,
    pub activetime: serde_json::Number,
    pub lastpaid: f32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PivxStatus {
    staking_status: bool,
    staking_enabled: bool,
    coldstaking_enabled: bool,
    haveconnections: bool,
    mnsync: bool,
    walletunlocked: bool,
    stakeablecoins: i128,
    stakingbalance: f64,
    stakesplitthreshold: f64,
    lastattempt_age: i64,
    lastattempt_depth: i64,
    lastattempt_hash: String,
    lastattempt_coins: i128,
    lastattempt_tries: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MasternodeCount {
    total: i32,
    stable: i32,
    enabled: i32,
    inqueue: i32,
    ipv4: i32,
    ipv6: i32,
    onion: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetInfo {
    pub version: i32,
    pub protocolversion: i32,
    pub services: String,
    pub walletversion: i32,
    pub balance: f64,
    #[serde(rename = "staking status")]
    pub staking_status: String,
    pub blocks: i32,
    pub timeoffset: i32,
    pub connections: i32,
    pub proxy: String,
    pub difficulty: f64,
    pub testnet: bool,
    pub moneysupply: f64,
    pub transparentsupply: f64,
    pub shieldsupply: f64,
    pub keypoololdest: i64,
    pub keypoolsize: i32,
    pub paytxfee: f64,
    pub relayfee: f64,
    pub errors: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BudgetInfo {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(rename = "Hash")]
    pub hash: String,
    #[serde(rename = "FeeHash")]
    pub fee_hash: String,
    #[serde(rename = "BlockStart")]
    pub block_start: u32,
    #[serde(rename = "BlockEnd")]
    pub block_end: u32,
    #[serde(rename = "TotalPaymentCount")]
    pub total_payment_count: u32,
    #[serde(rename = "RemainingPaymentCount")]
    pub remaining_payment_count: u32,
    #[serde(rename = "PaymentAddress")]
    pub payment_address: String,
    #[serde(rename = "Ratio")]
    pub ratio: f64,
    #[serde(rename = "Yeas")]
    pub yeas: u32,
    #[serde(rename = "Nays")]
    pub nays: u32,
    #[serde(rename = "Abstains")]
    pub abstains: u32,
    #[serde(rename = "TotalPayment")]
    pub total_payment: f64,
    #[serde(rename = "MonthlyPayment")]
    pub monthly_payment: f64,
    #[serde(rename = "IsEstablished")]
    pub is_established: bool,
    #[serde(rename = "IsValid")]
    pub is_valid: bool,
    #[serde(rename = "Allotted")]
    pub allotted: f64,
}

jsonrpc_client!(pub struct BitcoinRpcClient {
    single:
        pub fn createrawtransaction(&self, inputs: &[TxInput], outputs: &HashMap<&str, f64>, locktime: Option<u32>) -> Result<String>;
        pub fn dumpprivkey(&self, address: &str) -> Result<String>;
        pub fn generate(&self, number: usize, iterations: Option<usize>) -> Result<Vec<String>>;
        pub fn getbestblockhash(&self) -> Result<String>;
        pub fn getinfo(&self) -> Result<GetInfo>;
        pub fn getblockchaininfo(&self) -> Result<BlockChainInfo>;
        pub fn getblockcount(&self) -> Result<u32>;
        pub fn getblock(&self, block_hash: String) -> Result<FullBlock>;
        pub fn getblockhash(&self, block_height: u32) -> Result<String>;
        pub fn getblockheader(&self, block_hash: String) -> Result<Block>;
        pub fn getbudgetinfo(&self) -> Result<Vec<BudgetInfo>>;
        pub fn getmasternodecount(&self) -> Result<MasternodeCount>;
        pub fn getnewaddress(&self, account: Option<&str>, address_type: Option<&str>) -> Result<String>;
        pub fn getrawmempool(&self, format: bool) -> Result<RawMemPool>;
        pub fn listmasternodes(&self, mn_addr: Option<&str>) -> Result<Vec<MasternodeList>>;
        pub fn sendrawtransaction(&self, transaction: &str, allow_high_fee: Option<bool>) -> Result<String>;
        pub fn sendtoaddress(&self, address: &str, amount: f64, comment: Option<&str>, comment_to: Option<&str>, include_fee: Option<bool>) -> Result<String>;
        pub fn signrawtransaction(&self, transaction: &str, outputs: Option<&[TxOutput]>, privkeys: Option<&[&str]>, sig_hash_type: Option<&str>) -> Result<SignedTx>;
        pub fn gettxout(&self, txid: &str, vout: u32, unconfirmed: bool) -> Result<Option<TxOut>>;
        pub fn getstakingstatus(&self) -> Result<PivxStatus>;
    enum:
        #[cfg(all(not(feature = "btc")))] pub fn getblockinfo(&self) -> Result<Zero(SerializedData)|One(Block)|Two(FullBlock)>;
    });