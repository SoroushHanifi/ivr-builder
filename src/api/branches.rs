// src/api/branches.rs
//
// مدیریت Branch های بین گره‌های IVR
//
// POST   /api/nodes/:node_id/branches   ← اضافه کردن branch به گره
// PUT    /api/branches/:branch_id       ← بروزرسانی branch
// DELETE /api/branches/:branch_id       ← حذف branch
//
// Digit های مجاز:
//   "0"-"9"    ← عدد فشرده شده
//   "*"        ← ستاره
//   "#"        ← هشتگ
//   "timeout"  ← بدون پاسخ در مهلت مقرر
//   "invalid"  ← عدد نامعتبر

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

// ── POST /api/nodes/:node_id/branches ────────────────────────

pub async fn create(
    State(db): State<Db>,
    Path(node_id): Path<String>,
    Json(dto): Json<CreateBranchDto>,
) -> ApiResult<Json<IvrBranch>> {
    // اعتبارسنجی digit
    if !is_valid_digit(&dto.digit) {
        return Err(ApiError::BadRequest(format!(
            "digit '{}' نامعتبر است. مقادیر مجاز: {}",
            dto.digit,
            VALID_DIGITS.join(", ")
        )));
    }

    // بررسی وجود node
    let node = sqlx::query_as::<_, IvrNode>(
        "SELECT id, flow_id, node_type, label, config, created_at FROM ivr_nodes WHERE id = ?"
    )
    .bind(&node_id)
    .fetch_optional(db.as_ref())
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("گره '{}' پیدا نشد", node_id)))?;

    // بررسی که next_node_id در همان فلو باشد
    if let Some(ref next_id) = dto.next_node_id {
        let next_in_same_flow: bool = sqlx::query_scalar(
            "SELECT COUNT(*) FROM ivr_nodes WHERE id = ? AND flow_id = ?"
        )
        .bind(next_id)
        .bind(&node.flow_id)
        .fetch_one(db.as_ref())
        .await
        .map(|c: i64| c > 0)
        .unwrap_or(false);

        if !next_in_same_flow {
            return Err(ApiError::BadRequest(
                format!("گره مقصد '{}' در همین فلو وجود ندارد", next_id)
            ));
        }

        // جلوگیری از self-loop
        if next_id == &node_id {
            return Err(ApiError::BadRequest(
                "یک گره نمی‌تواند به خودش ارجاع دهد".to_string()
            ));
        }
    }

    // بررسی تکراری نبودن digit در همین node
    let digit_exists: bool = sqlx::query_scalar(
        "SELECT COUNT(*) FROM ivr_branches WHERE node_id = ? AND digit = ?"
    )
    .bind(&node_id)
    .bind(&dto.digit)
    .fetch_one(db.as_ref())
    .await
    .map(|c: i64| c > 0)
    .unwrap_or(false);

    if digit_exists {
        return Err(ApiError::BadRequest(
            format!("digit '{}' قبلاً برای این گره تعریف شده", dto.digit)
        ));
    }

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp();

    sqlx::query(
        "INSERT INTO ivr_branches (id, node_id, digit, next_node_id, label, created_at)
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&node_id)
    .bind(&dto.digit)
    .bind(&dto.next_node_id)
    .bind(&dto.label)
    .bind(now)
    .execute(db.as_ref())
    .await?;

    // بروزرسانی updated_at فلو
    sqlx::query("UPDATE ivr_flows SET updated_at = ? WHERE id = ?")
        .bind(now)
        .bind(&node.flow_id)
        .execute(db.as_ref())
        .await?;

    println!(
        "🔀 Branch added: node '{}' + digit '{}' → '{:?}'",
        node_id, dto.digit, dto.next_node_id
    );

    let branch = sqlx::query_as::<_, IvrBranch>(
        "SELECT id, node_id, digit, next_node_id, label, created_at
         FROM ivr_branches WHERE id = ?"
    )
    .bind(&id)
    .fetch_one(db.as_ref())
    .await?;

    Ok(Json(branch))
}

// ── PUT /api/branches/:branch_id ─────────────────────────────

pub async fn update(
    State(db): State<Db>,
    Path(branch_id): Path<String>,
    Json(dto): Json<UpdateBranchDto>,
) -> ApiResult<Json<IvrBranch>> {
    // دریافت branch موجود
    let existing = sqlx::query_as::<_, IvrBranch>(
        "SELECT id, node_id, digit, next_node_id, label, created_at
         FROM ivr_branches WHERE id = ?"
    )
    .bind(&branch_id)
    .fetch_optional(db.as_ref())
    .await?
    .ok_or_else(|| ApiError::NotFound(format!("branch '{}' پیدا نشد", branch_id)))?;

    // اگر digit عوض شد، اعتبارسنجی کن
    let new_digit = if let Some(ref d) = dto.digit {
        if !is_valid_digit(d) {
            return Err(ApiError::BadRequest(format!("digit '{}' نامعتبر است", d)));
        }
        // بررسی تکراری نبودن (به جز خودش)
        let conflict: bool = sqlx::query_scalar(
            "SELECT COUNT(*) FROM ivr_branches
             WHERE node_id = ? AND digit = ? AND id != ?"
        )
        .bind(&existing.node_id)
        .bind(d)
        .bind(&branch_id)
        .fetch_one(db.as_ref())
        .await
        .map(|c: i64| c > 0)
        .unwrap_or(false);

        if conflict {
            return Err(ApiError::BadRequest(
                format!("digit '{}' قبلاً برای این گره تعریف شده", d)
            ));
        }
        d.clone()
    } else {
        existing.digit.clone()
    };

    // اگر next_node_id عوض شد، بررسی کن در همان فلو باشد
    let new_next = if let Some(ref next_id) = dto.next_node_id {
        // پیدا کردن flow_id از طریق node
        let flow_id: String = sqlx::query_scalar(
            "SELECT flow_id FROM ivr_nodes WHERE id = ?"
        )
        .bind(&existing.node_id)
        .fetch_one(db.as_ref())
        .await
        .map_err(|_| ApiError::Internal("خطا در یافتن فلو".to_string()))?;

        let valid: bool = sqlx::query_scalar(
            "SELECT COUNT(*) FROM ivr_nodes WHERE id = ? AND flow_id = ?"
        )
        .bind(next_id)
        .bind(&flow_id)
        .fetch_one(db.as_ref())
        .await
        .map(|c: i64| c > 0)
        .unwrap_or(false);

        if !valid {
            return Err(ApiError::BadRequest(
                format!("گره مقصد '{}' در این فلو وجود ندارد", next_id)
            ));
        }
        Some(next_id.clone())
    } else {
        existing.next_node_id.clone()
    };

    let new_label = dto.label.or(existing.label);

    sqlx::query(
        "UPDATE ivr_branches SET digit = ?, next_node_id = ?, label = ? WHERE id = ?"
    )
    .bind(&new_digit)
    .bind(&new_next)
    .bind(&new_label)
    .bind(&branch_id)
    .execute(db.as_ref())
    .await?;

    let branch = sqlx::query_as::<_, IvrBranch>(
        "SELECT id, node_id, digit, next_node_id, label, created_at
         FROM ivr_branches WHERE id = ?"
    )
    .bind(&branch_id)
    .fetch_one(db.as_ref())
    .await?;

    Ok(Json(branch))
}

// ── DELETE /api/branches/:branch_id ──────────────────────────

pub async fn delete(
    State(db): State<Db>,
    Path(branch_id): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let result = sqlx::query("DELETE FROM ivr_branches WHERE id = ?")
        .bind(&branch_id)
        .execute(db.as_ref())
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound(format!("branch '{}' پیدا نشد", branch_id)));
    }

    println!("🗑️  Deleted branch '{}'", branch_id);
    Ok(Json(serde_json::json!({ "deleted": branch_id })))
}
