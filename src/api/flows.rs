// src/api/flows.rs
//
// CRUD برای IVR Flows
//
// GET    /api/flows              ← لیست همه فلوها
// POST   /api/flows              ← ایجاد فلو جدید
// GET    /api/flows/:flow_id     ← دریافت فلو کامل (با node ها و branch ها)
// PUT    /api/flows/:flow_id     ← بروزرسانی نام/توضیحات
// DELETE /api/flows/:flow_id     ← حذف فلو (cascade)
// PATCH  /api/flows/:flow_id/entry ← تعیین ورودی اولیه فلو

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

// ── GET /api/flows ────────────────────────────────────────────

pub async fn list(State(db): State<Db>) -> ApiResult<Json<Vec<IvrFlow>>> {
    let flows = sqlx::query_as::<_, IvrFlow>(
        "SELECT id, name, description, entry_node_id, created_at, updated_at
         FROM ivr_flows
         ORDER BY updated_at DESC"
    )
    .fetch_all(db.as_ref())
    .await?;

    Ok(Json(flows))
}

// ── POST /api/flows ───────────────────────────────────────────

pub async fn create(
    State(db): State<Db>,
    Json(dto): Json<CreateFlowDto>,
) -> ApiResult<Json<IvrFlow>> {
    if dto.name.trim().is_empty() {
        return Err(ApiError::BadRequest("نام فلو نمی‌تواند خالی باشد".to_string()));
    }

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp();

    sqlx::query(
        "INSERT INTO ivr_flows (id, name, description, entry_node_id, created_at, updated_at)
         VALUES (?, ?, ?, NULL, ?, ?)"
    )
    .bind(&id)
    .bind(dto.name.trim())
    .bind(&dto.description)
    .bind(now)
    .bind(now)
    .execute(db.as_ref())
    .await?;

    let flow = sqlx::query_as::<_, IvrFlow>(
        "SELECT id, name, description, entry_node_id, created_at, updated_at
         FROM ivr_flows WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(db.as_ref())
    .await?;

    println!("🎛️  Created IVR flow '{}' ({})", flow.name, flow.id);
    Ok(Json(flow))
}

// ── GET /api/flows/:flow_id ───────────────────────────────────

pub async fn get_full(
    State(db): State<Db>,
    Path(flow_id): Path<String>,
) -> ApiResult<Json<FlowFull>> {
    // فلو اصلی
    let flow = sqlx::query_as::<_, IvrFlow>(
        "SELECT id, name, description, entry_node_id, created_at, updated_at
         FROM ivr_flows WHERE id = ?"
    )
    .bind(&flow_id)
    .fetch_optional(db.as_ref())
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("فلو '{}' پیدا نشد", flow_id)))?;

    // همه node های این فلو
    let nodes = sqlx::query_as::<_, IvrNode>(
        "SELECT id, flow_id, node_type, label, config, created_at
         FROM ivr_nodes WHERE flow_id = ? ORDER BY created_at"
    )
    .bind(&flow_id)
    .fetch_all(db.as_ref())
    .await?;

    // همه branch های این فلو (یکجا با join)
    let branches = sqlx::query_as::<_, IvrBranch>(
        "SELECT b.id, b.node_id, b.digit, b.next_node_id, b.label, b.created_at
         FROM ivr_branches b
         JOIN ivr_nodes n ON n.id = b.node_id
         WHERE n.flow_id = ?
         ORDER BY b.node_id, b.digit"
    )
    .bind(&flow_id)
    .fetch_all(db.as_ref())
    .await?;

    // ترکیب node ها با branch های مربوطه
    let nodes_full: Vec<NodeFull> = nodes
        .into_iter()
        .map(|node| {
            let node_branches: Vec<IvrBranch> = branches
                .iter()
                .filter(|b| b.node_id == node.id)
                .cloned()
                .collect();
            NodeFull {
                node: NodeResponse::from_row(node),
                branches: node_branches,
            }
        })
        .collect();

    Ok(Json(FlowFull {
        id: flow.id,
        name: flow.name,
        description: flow.description,
        entry_node_id: flow.entry_node_id,
        created_at: flow.created_at,
        updated_at: flow.updated_at,
        nodes: nodes_full,
    }))
}

// ── PUT /api/flows/:flow_id ───────────────────────────────────

pub async fn update(
    State(db): State<Db>,
    Path(flow_id): Path<String>,
    Json(dto): Json<UpdateFlowDto>,
) -> ApiResult<Json<IvrFlow>> {
    // بررسی وجود فلو
    let existing = sqlx::query_as::<_, IvrFlow>(
        "SELECT id, name, description, entry_node_id, created_at, updated_at
         FROM ivr_flows WHERE id = ?"
    )
    .bind(&flow_id)
    .fetch_optional(db.as_ref())
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("فلو '{}' پیدا نشد", flow_id)))?;

    let new_name = dto.name
        .map(|n| n.trim().to_string())
        .filter(|n| !n.is_empty())
        .unwrap_or(existing.name);

    let new_desc = dto.description.or(existing.description);
    let now = Utc::now().timestamp();

    sqlx::query(
        "UPDATE ivr_flows SET name = ?, description = ?, updated_at = ? WHERE id = ?"
    )
    .bind(&new_name)
    .bind(&new_desc)
    .bind(now)
    .bind(&flow_id)
    .execute(db.as_ref())
    .await?;

    let flow = sqlx::query_as::<_, IvrFlow>(
        "SELECT id, name, description, entry_node_id, created_at, updated_at
         FROM ivr_flows WHERE id = ?"
    )
    .bind(&flow_id)
    .fetch_one(db.as_ref())
    .await?;

    Ok(Json(flow))
}

// ── DELETE /api/flows/:flow_id ────────────────────────────────

pub async fn delete(
    State(db): State<Db>,
    Path(flow_id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let result = sqlx::query("DELETE FROM ivr_flows WHERE id = ?")
        .bind(&flow_id)
        .execute(db.as_ref())
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound(format!("فلو '{}' پیدا نشد", flow_id)));
    }

    println!("🗑️  Deleted IVR flow '{}'", flow_id);
    Ok(Json(serde_json::json!({ "deleted": flow_id })))
}

// ── PATCH /api/flows/:flow_id/entry ──────────────────────────

pub async fn set_entry(
    State(db): State<Db>,
    Path(flow_id): Path<String>,
    Json(dto): Json<SetEntryDto>,
) -> ApiResult<Json<IvrFlow>> {
    // بررسی وجود node در همین فلو
    let node_exists: bool = sqlx::query_scalar(
        "SELECT COUNT(*) FROM ivr_nodes WHERE id = ? AND flow_id = ?"
    )
    .bind(&dto.node_id)
    .bind(&flow_id)
    .fetch_one(db.as_ref())
    .await
    .map(|c: i64| c > 0)
    .unwrap_or(false);

    if !node_exists {
        return Err(ApiError::BadRequest(
            format!("گره '{}' در این فلو وجود ندارد", dto.node_id)
        ));
    }

    let now = Utc::now().timestamp();
    sqlx::query(
        "UPDATE ivr_flows SET entry_node_id = ?, updated_at = ? WHERE id = ?"
    )
    .bind(&dto.node_id)
    .bind(now)
    .bind(&flow_id)
    .execute(db.as_ref())
    .await?;

    let flow = sqlx::query_as::<_, IvrFlow>(
        "SELECT id, name, description, entry_node_id, created_at, updated_at
         FROM ivr_flows WHERE id = ?"
    )
    .bind(&flow_id)
    .fetch_optional(db.as_ref())
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("فلو '{}' پیدا نشد", flow_id)))?;

    println!("🎯 Flow '{}' entry set to node '{}'", flow_id, dto.node_id);
    Ok(Json(flow))
}
