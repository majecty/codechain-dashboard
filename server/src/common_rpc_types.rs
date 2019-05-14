use std::collections::HashMap;
use std::net::IpAddr;

use chrono::{DateTime, Utc};
use cprimitives::H256;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json;

pub type NodeName = String;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum NodeStatus {
    Starting,
    Run,
    Stop,
    Updating,
    Error,
    UFO,
}

impl Default for NodeStatus {
    fn default() -> NodeStatus {
        NodeStatus::Stop
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShellStartCodeChainRequest {
    pub env: String,
    pub args: String,
}

pub type ShellUpdateCodeChainRequest = (ShellStartCodeChainRequest, UpdateCodeChainRequest);

pub type Connection = (NodeName, NodeName);

#[derive(Debug, Serialize, Deserialize, PartialEq, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlockId {
    pub block_number: i64,
    pub hash: H256,
}

#[derive(Debug, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NodeVersion {
    pub version: String,
    pub hash: String,
    pub binary_checksum: String,
}

pub type PendingTransaction = serde_json::Value;

pub type Tag = String;

#[derive(Debug, Serialize, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WhiteList {
    pub list: Vec<(IpAddr, Tag)>,
    pub enabled: bool,
}

pub type BlackList = WhiteList;

pub type NetworkUsage = HashMap<String, i32>;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct HardwareUsage {
    pub total: i64,
    pub available: i64,
    pub percentage_used: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HardwareInfo {
    pub cpu_usage: Vec<f64>,
    pub disk_usage: HardwareUsage,
    pub memory_usage: HardwareUsage,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StructuredLog {
    pub level: String,
    pub target: String,
    pub message: String,
    pub timestamp: String,
    pub thread_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum UpdateCodeChainRequest {
    #[serde(rename_all = "camelCase")]
    Git {
        commit_hash: String,
    },
    #[serde(rename_all = "camelCase")]
    Binary {
        #[serde(rename = "binaryURL")]
        binary_url: String,
        binary_checksum: String,
    },
}


#[derive(Debug, Clone)]
pub enum GraphPeriod {
    Minutes5,
    Hour,
    Day,
}

impl Serialize for GraphPeriod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer, {
        serializer.serialize_str(match *self {
            GraphPeriod::Minutes5 => "minutes5",
            GraphPeriod::Hour => "hour",
            GraphPeriod::Day => "day",
        })
    }
}

impl<'de> Deserialize<'de> for GraphPeriod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "minutes5" => GraphPeriod::Minutes5,
            "hour" => GraphPeriod::Hour,
            "day" => GraphPeriod::Day,
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GraphCommonArgs {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    pub period: GraphPeriod,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphNetworkOutAllRow {
    pub node_name: String,
    pub time: DateTime<Utc>,
    pub value: f32,
}

pub type GraphNetworkOutAllAVGRow = GraphNetworkOutAllRow;
