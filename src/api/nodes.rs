// src/api/nodes.rs
//
// CRUD برای IVR Nodes
//
// POST   /api/flows/:flow_id/nodes   ← ایجاد گره جدید در فلو
// GET    /api/nodes/:node_id         ← دریافت گره (با branch ها)
// PUT    /api/nodes/:node_id         ← بروزرسانی گره
// DELETE /api/nodes/:node_id         ← حذف گره (cascade branches)

use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use uuid::Uuid;
use std::sync::Arc;
use sqlx::SqlitePool;

use crate::{
    error::{ApiError, ApiResult},
    models::*,
};

type Db = Arc<SqlitePool>;

// ── POST /api/flows/:flow_id/nodes ────────────────────────────

pub async fn create(
    State(db): State<Db>,
    Path(flow_id): Path<String>,
    Json(dto): Json<CreateNodeDto>,
) -> ApiResult<Json<NodeFull>> {
    // اعتبارسنجی node_type
    if !is_valid_node_type(&dto.node_type) {
        return Err(ApiError::BadRequest(format!(
            "نوع گره '{}' نامعتبر است. انواع مجاز: {}",
            dto.node_type,
            VALID_NODE_TYPES.join(", ")
        )));
    }

    // بررسی وجود فلو
    let flow_exists: bool = sqlx::query_scalar(
        "SELECT COUNT(*) FROM ivr_flows WHERE id = ?"
    )
    .bind(&flow_id)
    .fetch_one(db.as_ref())
    .await
    .map(|c: i64| c > 0)
    .unwrap_or(false);

    if !flow_exists {
        return Err(ApiError::NotFound(format!("فلو '{}' پیدا نشد", flow_id)));
    }

    // اعتبارسنجی اضافی config بر اساس نوع
    validate_config(&dto.node_type, &dto.config)?;

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp();
    let config_json = serde_json::to_string(&dto.config)
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    sqlx::query(
        "INSERT INTO ivr_nodes (id, flow_id, node_type, label, config, created_at)
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&flow_id)
    .bind(&dto.node_type)
    .bind(&dto.label)
    .bind(&config_json)
    .bind(now)
    .execute(db.as_ref())
    .await?;

    // بروزرسانی updated_at فلو
    sqlx::query("UPDATE ivr_flows SET updated_at = ? WHERE id = ?")
        .bind(now)
        .bind(&flow_id)
        .execute(db.as_ref())
        .await?;

    println!("📦 Created node '{}' (type={}) in flow '{}'", id, dto.node_type, flow_id);

    let node = sqlx::query_as::<_, IvrNode>(
        "SELECT id, flow_id, node_type, label, config, created_at FROM ivr_nodes WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(db.as_ref())
    .await?;

    Ok(Json(NodeFull {
        node: NodeResponse::from_row(node),
        branches: vec![],
    }))
}

// ── GET /api/nodes/:node_id ───────────────────────────────────

pub async fn get(
    State(db): State<Db>,
    Path(node_id): Path<String>,
) -> ApiResult<Json<NodeFull>> {
    let node = fetch_node_full(&db, &node_id).await?;
    Ok(Json(node))
}

// ── PUT /api/nodes/:node_id ───────────────────────────────────

pub async fn update(
    State(db): State<Db>,
    Path(node_id): Path<String>,
    Json(dto): Json<UpdateNodeDto>,
) -> ApiResult<Json<NodeFull>> {
    let existing = sqlx::query_as::<_, IvrNode>(
        "SELECT id, flow_id, node_type, label, config, created_at FROM ivr_nodes WHERE id = ?"
    )
    .bind(&node_id)
    .fetch_optional(db.as_ref())
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("گره '{}' پیدا نشد", node_id)))?;

    let new_label = match dto.label {
        Some(l) if l.trim().is_empty() => None,
        other => other.or(existing.label),
    };

    let new_config_json = if let Some(new_config) = dto.config {
        validate_config(&existing.node_type, &new_config)?;
        serde_json::to_string(&new_config)
            .map_err(|e| ApiError::Internal(e.to_string()))?
    } else {
        existing.config.clone()
    };

    let now = Utc::now().timestamp();

    sqlx::query(
        "UPDATE ivr_nodes SET label = ?, config = ? WHERE id = ?"
    )
    .bind(&new_label)
    .bind(&new_config_json)
    .bind(&node_id)
    .execute(db.as_ref())
    .await?;

    // بروزرسانی updated_at فلو
    sqlx::query("UPDATE ivr_flows SET updated_at = ? WHERE id = ?")
        .bind(now)
        .bind(&existing.flow_id)
        .execute(db.as_ref())
        .await?;

    let node = fetch_node_full(&db, &node_id).await?;
    Ok(Json(node))
}

// ── DELETE /api/nodes/:node_id ────────────────────────────────

pub async fn delete(
    State(db): State<Db>,
    Path(node_id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    // اگر این node به عنوان entry_node_id ثبت شده، آن را null کن
    sqlx::query(
        "UPDATE ivr_flows SET entry_node_id = NULL, updated_at = ?
         WHERE entry_node_id = ?"
    )
    .bind(Utc::now().timestamp())
    .bind(&node_id)
    .execute(db.as_ref())
    .await?;

    let result = sqlx::query("DELETE FROM ivr_nodes WHERE id = ?")
        .bind(&node_id)
        .execute(db.as_ref())
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound(format!("گره '{}' پیدا نشد", node_id)));
    }

    println!("🗑️  Deleted node '{}'", node_id);
    Ok(Json(serde_json::json!({ "deleted": node_id })))
}

// ─────────────────── helpers ───────────────────

async fn fetch_node_full(db: &SqlitePool, node_id: &str) -> ApiResult<NodeFull> {
    let node = sqlx::query_as::<_, IvrNode>(
        "SELECT id, flow_id, node_type, label, config, created_at FROM ivr_nodes WHERE id = ?"
    )
    .bind(node_id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("گره '{}' پیدا نشد", node_id)))?;

    let branches = sqlx::query_as::<_, IvrBranch>(
        "SELECT id, node_id, digit, next_node_id, label, created_at
         FROM ivr_branches WHERE node_id = ? ORDER BY digit"
    )
    .bind(node_id)
    .fetch_all(db)
    .await?;

    Ok(NodeFull {
        node: NodeResponse::from_row(node),
        branches,
    })
}

/// اعتبارسنجی ساده config بر اساس node_type
fn validate_config(node_type: &str, config: &serde_json::Value) -> ApiResult<()> {
    match node_type {
        "connect_call" => {
            if config.get("account").and_then(|v| v.as_str()).is_none() {
                return Err(ApiError::BadRequest(
                    "connect_call نیاز به فیلد 'account' دارد".to_string()
                ));
            }
        }
        "play_audio" => {
            if config.get("audio_file").and_then(|v| v.as_str()).is_none() {
                return Err(ApiError::BadRequest(
                    "play_audio نیاز به فیلد 'audio_file' دارد".to_string()
                ));
            }
        }
        _ => {} // سایر انواع validation اختیاری
    }
    Ok(())
}
