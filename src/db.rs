// src/db.rs
//
// اضافه کردن جداول IVR به دیتابیس مشترک با PBX
// این تابع ایمن است — از IF NOT EXISTS استفاده می‌کند

use sqlx::{SqlitePool, Error as SqlxError};

pub async fn init_schema(pool: &SqlitePool) -> Result<(), SqlxError> {
    // فعال کردن foreign keys در SQLite
    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(pool)
        .await?;

    // ── جدول فلوها ──
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS ivr_flows (
            id           TEXT PRIMARY KEY,
            name         TEXT NOT NULL,
            description  TEXT,
            entry_node_id TEXT,
            created_at   INTEGER NOT NULL,
            updated_at   INTEGER NOT NULL
        )"
    )
    .execute(pool)
    .await?;

    // ── جدول گره‌ها ──
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS ivr_nodes (
            id          TEXT PRIMARY KEY,
            flow_id     TEXT NOT NULL REFERENCES ivr_flows(id) ON DELETE CASCADE,
            node_type   TEXT NOT NULL,
            label       TEXT,
            config      TEXT NOT NULL DEFAULT '{}',
            created_at  INTEGER NOT NULL
        )"
    )
    .execute(pool)
    .await?;

    // index روی flow_id برای گره‌ها
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_ivr_nodes_flow ON ivr_nodes(flow_id)"
    )
    .execute(pool)
    .await?;

    // ── جدول branch ها ──
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS ivr_branches (
            id           TEXT PRIMARY KEY,
            node_id      TEXT NOT NULL REFERENCES ivr_nodes(id) ON DELETE CASCADE,
            digit        TEXT NOT NULL,
            next_node_id TEXT REFERENCES ivr_nodes(id) ON DELETE SET NULL,
            label        TEXT,
            created_at   INTEGER NOT NULL,
            UNIQUE(node_id, digit)
        )"
    )
    .execute(pool)
    .await?;

    // index روی node_id برای branch ها
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_ivr_branches_node ON ivr_branches(node_id)"
    )
    .execute(pool)
    .await?;

    println!("✅ IVR schema ready (flows, nodes, branches)");
    Ok(())
}
