// SPDX-License-Identifier: MIT OR Apache-2.0

use core::error;
use core::fmt;
use core::fmt::Display;
use core::fmt::Formatter;

pub use ethos_bitcoind::GetNetworkInfo;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
/// Return type for the `gettxoutproof` rpc command, the internal is
/// the hex-encoded representation of the Merkle Block, as defined
/// by Bitcoin Core.
pub struct GetTxOutProof(pub String);

/// General information about our peers. Returned by get_peer_info
#[derive(Debug, Deserialize, Serialize)]
pub struct PeerInfo {
    /// This peer's ID in the peer manager.
    pub id: u32,
    /// The network address for this peer.
    pub address: String,
    /// Hex-encoded bitfield with the services this peer advertises.
    pub services: String,
    /// Human-readable names for the recognized services this peer advertises.
    #[serde(rename = "servicesnames")]
    pub services_names: Vec<String>,
    /// Whether this peer requested mempool transaction relay from us.
    #[serde(rename = "relaytxes")]
    pub relay_txs: bool,
    /// User agent is a string that represents the client being used by our peer. E.g.
    /// /Satoshi-26.0/ for bitcoin core version 26
    pub user_agent: String,
    /// Whether this peer connection is inbound.
    pub inbound: bool,
    /// Whether we selected this peer as a BIP152 high-bandwidth compact block relay peer.
    pub bip152_hb_to: bool,
    /// Whether this peer selected us as a BIP152 high-bandwidth compact block relay peer.
    pub bip152_hb_from: bool,
    /// This peer's height at the time we've opened a connection with them
    pub initial_height: u32,
    /// The peer time offset in seconds.
    #[serde(rename = "timeoffset")]
    pub time_offset: i64,
    /// The connection type of this peer
    ///
    /// We can connect with peers for different reasons. E.g. we can connect to a peer to
    /// see if it has a block we're missing, or just to check if that address is still alive.
    /// Possible values are: Feeler, Regular and Extra
    pub kind: String,
    /// The state of this peer
    ///
    /// Can be either Ready, Connecting or Banned
    pub state: String,
    /// Special permissions granted to this peer.
    pub permissions: Vec<String>,
    /// The transport protocol used with peer.
    pub transport_protocol: String,
}

/// Core-shaped `getblock` result (verbosity 0–3).
pub type GetBlockRes = ethos_bitcoind::GetBlockResponse;

/// Core-shaped `getblockheader` result.
pub type GetBlockHeaderRes = ethos_bitcoind::GetBlockHeaderResponse;

/// Core-shaped `getrawtransaction` result.
pub type GetRawTransactionRes = ethos_bitcoind::GetRawTransactionResponse;

/// A confidence enum to auxiliate rescan timestamp values.
///
/// Tells how much confidence you need for this rescan request. That is, the how conservative you want floresta to be when determining which block to start the rescan.
/// will make the rescan to start in a block that have an lower timestamp than the given in order to be more certain
/// about finding addresses and relevant transactions, a lower confidence will make the rescan to be closer to the given value.
///
/// This input is necessary to cover network variancy specially in testnet, for mainnet you can safely use low or medium confidences
/// depending on how much sure you are about the given timestamp covering the addresses you need.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[serde(rename_all = "lowercase")]
pub enum RescanConfidence {
    /// `high`: 99% confidence interval. Meaning 46 minutes in seconds.
    High,

    /// `medium` (default): 95% confidence interval. Meaning 30 minutes in seconds.
    Medium,

    /// `low`: 90% confidence interval. Meaning 23 minutes in seconds.
    Low,

    /// `exact`: Removes any lookback addition. Meaning 0 in seconds.
    Exact,
}

#[derive(Debug)]
/// All possible errors returned by the jsonrpc
pub enum Error {
    /// An error while deserializing our response
    Serde(serde_json::Error),

    #[cfg(feature = "with-jsonrpc")]
    /// An internal reqwest error
    JsonRpc(jsonrpc::Error),

    /// An error internal to our jsonrpc server
    Api(serde_json::Value),

    /// The server sent an empty response
    EmptyResponse,

    /// The provided verbosity level is invalid
    InvalidVerbosity,

    /// The user requested a rescan based on invalid values.
    InvalidRescanVal,

    /// The requested transaction output was not found
    TxOutNotFound,
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

#[cfg(feature = "with-jsonrpc")]
impl From<jsonrpc::Error> for Error {
    fn from(value: jsonrpc::Error) -> Self {
        Self::JsonRpc(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "with-jsonrpc")]
            Self::JsonRpc(e) => write!(f, "JsonRpc returned an error {e}"),
            Self::Api(e) => write!(f, "general jsonrpc error: {e}"),
            Self::Serde(e) => write!(f, "error while deserializing the response: {e}"),
            Self::EmptyResponse => write!(f, "got an empty response from server"),
            Self::InvalidVerbosity => write!(f, "invalid verbosity level"),
            Self::InvalidRescanVal => write!(f, "Invalid rescan values"),
            Self::TxOutNotFound => write!(f, "Transaction output was not found"),
        }
    }
}

/// Core-shaped `getmemoryinfo` result.
pub type GetMemInfoRes = ethos_bitcoind::GetMemoryInfoResponse;
/// Stats object arm of `getmemoryinfo`.
pub type GetMemInfoStats = ethos_bitcoind::GetMemoryInfoResponseGetMemoryInfoObject;
/// Locked-memory counters inside `getmemoryinfo` stats.
pub type MemInfoLocked = ethos_bitcoind::GetMemoryInfoLocked;
/// Active RPC command row in `getrpcinfo`.
pub type ActiveCommand = ethos_bitcoind::GetRpcInfoActiveCommands;
/// Core-shaped `getrpcinfo` result.
pub type GetRpcInfoRes = ethos_bitcoind::GetRpcInfoResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[serde(rename_all = "lowercase")]
/// Enum to represent the different subcommands for the addnode command
pub enum AddNodeCommand {
    /// Add a node to the addnode list (but not connect to it)
    Add,

    /// Remove a node from the addnode list (but not necessarily disconnect from it)
    Remove,

    /// Connect to a node once, but don't add it to the addnode list
    Onetry,
}

/// A simple implementation to convert the enum to a string.
/// Useful for get the subcommand name of addnode with
/// command.to_string()
impl Display for AddNodeCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let cmd = match self {
            Self::Add => "add",
            Self::Remove => "remove",
            Self::Onetry => "onetry",
        };
        write!(f, "{cmd}")
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod ethos_wire_tests {
    use std::collections::BTreeMap;

    use bitcoin::Amount;
    use ethos_bitcoind::GetBlockchainInfo;
    use ethos_bitcoind::GetMemoryInfoLocked;
    use ethos_bitcoind::GetMemoryInfoResponse;
    use ethos_bitcoind::GetMemoryInfoResponseGetMemoryInfoObject;
    use ethos_bitcoind::GetNetworkInfo;
    use ethos_bitcoind::GetRpcInfoResponse;
    use ethos_bitcoind::GetTxOut;
    use ethos_bitcoind::ScriptPubKey;
    use serde_json::json;

    #[test]
    fn get_tx_out_serializes_amount_as_btc_float_and_omits_none() {
        let out = GetTxOut {
            best_block: "00".repeat(32),
            confirmations: 1,
            value: Amount::from_sat(50_000_000),
            script_pubkey: ScriptPubKey {
                asm: String::new(),
                desc: "raw(00)".to_string(),
                hex: "00".to_string(),
                r#type: "nulldata".to_string(),
                address: None,
            },
            coinbase: true,
        };
        let v = serde_json::to_value(&out).expect("serialize GetTxOut");
        assert_eq!(v["value"], json!(0.5));
        assert!(
            v["scriptPubKey"].get("address").is_none(),
            "nested Option None must omit, not null: {v}"
        );
    }

    #[test]
    fn get_blockchain_info_omits_none_optionals() {
        let info = GetBlockchainInfo {
            chain: "regtest".to_string(),
            blocks: 0,
            headers: 0,
            best_block_hash: "00".repeat(32),
            difficulty: 1.0,
            automatic_pruning: None,
            bits: "1d00ffff".to_string(),
            chain_work: "00".to_string(),
            initial_block_download: false,
            median_time: 0,
            prune_height: None,
            prune_target_size: None,
            pruned: false,
            signet_challenge: None,
            size_on_disk: 0,
            target: "00".to_string(),
            time: 0,
            verification_progress: 1.0,
            warnings: vec![],
            backgroundvalidation: None,
        };
        let v = serde_json::to_value(&info).expect("serialize GetBlockchainInfo");
        assert!(v.get("backgroundvalidation").is_none());
        assert!(v.get("automatic_pruning").is_none());
        assert_eq!(v["bestblockhash"], json!("00".repeat(32)));
    }

    #[test]
    fn get_memory_info_stats_and_malloc_arms() {
        let stats = GetMemoryInfoResponse::Object(GetMemoryInfoResponseGetMemoryInfoObject {
            locked: GetMemoryInfoLocked {
                used: 1,
                free: 2,
                total: 3,
                locked: 3,
                chunks_used: 4,
                chunks_free: 5,
            },
        });
        let v = serde_json::to_value(&stats).expect("serialize memory stats");
        assert_eq!(v["locked"]["used"], json!(1));

        let malloc = GetMemoryInfoResponse::String("<malloc/>".to_string());
        let v = serde_json::to_value(&malloc).expect("serialize mallocinfo");
        assert_eq!(v, json!("<malloc/>"));
    }

    #[test]
    fn get_rpc_info_and_network_info_core_keys() {
        let rpc = GetRpcInfoResponse {
            active_commands: vec![],
            logpath: "/tmp/debug.log".to_string(),
        };
        let v = serde_json::to_value(&rpc).expect("serialize getrpcinfo");
        assert_eq!(v["logpath"], json!("/tmp/debug.log"));

        let net = GetNetworkInfo {
            version: 900,
            subversion: "/Floresta:0.9.0/".to_string(),
            protocol_version: 70016,
            local_services: "0".to_string(),
            local_services_names: vec![],
            local_relay: false,
            time_offset: 0,
            connections: 0,
            connections_in: 0,
            connections_out: 0,
            network_active: true,
            networks: vec![],
            relay_fee: 0.0,
            incremental_fee: 0.0,
            local_addresses: vec![],
            inv_buckets: BTreeMap::new(),
            asmap_version: None,
            tx_send_rate: 0,
            warnings: vec![],
        };
        let v = serde_json::to_value(&net).expect("serialize getnetworkinfo");
        assert_eq!(v["inv_buckets"], json!({}));
        assert_eq!(v["tx_send_rate"], json!(0));
        assert_eq!(v["protocolversion"], json!(70016));
    }
}
