// src/models.rs

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

// ─────────────────── DB Row types ───────────────────

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IvrFlow {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub entry_node_id: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IvrNode {
    pub id: String,
    pub flow_id: String,
    pub node_type: String,
    pub label: Option<String>,
    pub config: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IvrBranch {
    pub id: String,
    pub node_id: String,
    pub digit: String,
    pub next_node_id: Option<String>,
    pub label: Option<String>,
    pub created_at: i64,
}

// ─────────────────── DTOs (Request) ───────────────────

#[derive(Debug, Deserialize)]
pub struct CreateFlowDto {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFlowDto {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SetEntryDto {
    pub node_id: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateNodeDto {
    pub node_type: String,
    pub label: Option<String>,
    pub config: Value,
}

#[derive(Debug, Deserialize)]
pub struct UpdateNodeDto {
    pub label: Option<String>,
    pub config: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBranchDto {
    pub digit: String,
    pub next_node_id: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBranchDto {
    pub digit: Option<String>,
    pub next_node_id: Option<String>,
    pub label: Option<String>,
}

// ─────────────────── Response types ───────────────────

#[derive(Debug, Serialize)]
pub struct NodeResponse {
    pub id: String,
    pub flow_id: String,
    pub node_type: String,
    pub label: Option<String>,
    pub config: Value,
    pub created_at: i64,
}

impl NodeResponse {
    pub fn from_row(node: IvrNode) -> Self {
        let config = serde_json::from_str(&node.config).unwrap_or(Value::Null);
        Self {
            id: node.id,
            flow_id: node.flow_id,
            node_type: node.node_type,
            label: node.label,
            config,
            created_at: node.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct NodeFull {
    #[serde(flatten)]
    pub node: NodeResponse,
    pub branches: Vec<IvrBranch>,
}

#[derive(Debug, Serialize)]
pub struct FlowFull {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub entry_node_id: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub nodes: Vec<NodeFull>,
}

#[derive(Debug, Serialize)]
pub struct DeletedResponse {
    pub deleted: String,
}

// ─────────────────── Validation ───────────────────

pub const VALID_NODE_TYPES: &[&str] = &[
    "menu", "play_audio", "record_audio",
    "receive_digits", "connect_call", "hangup",
];

pub const VALID_DIGITS: &[&str] = &[
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "*", "#", "timeout", "invalid",
];

pub fn is_valid_node_type(t: &str) -> bool { VALID_NODE_TYPES.contains(&t) }
pub fn is_valid_digit(d: &str) -> bool     { VALID_DIGITS.contains(&d) }