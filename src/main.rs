// src/main.rs
//
// IVR Builder — سرویس REST API برای ساخت فلوهای IVR
//
// متغیرهای محیطی:
//   DATABASE_PATH   مسیر SQLite مشترک با PBX  (پیش‌فرض: data/sip_users.db)
//   IVR_BIND_ADDR   آدرس listen                (پیش‌فرض: 0.0.0.0:8090)
//
// Endpoints:
//   Flows:
//     GET    /api/flows
//     POST   /api/flows
//     GET    /api/flows/:id
//     PUT    /api/flows/:id
//     DELETE /api/flows/:id
//     PATCH  /api/flows/:id/entry
//   Nodes:
//     POST   /api/flows/:id/nodes
//     GET    /api/nodes/:id
//     PUT    /api/nodes/:id
//     DELETE /api/nodes/:id
//   Branches:
//     POST   /api/nodes/:id/branches
//     PUT    /api/branches/:id
//     DELETE /api/branches/:id

mod error;
mod models;
mod db;
mod api;

use std::sync::Arc;
use axum::{
    Router,
    routing::{delete, get, patch, post, put},
};
use sqlx::SqlitePool;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("========================================");
    println!("   🎛️  IVR Builder API");
    println!("========================================\n");

    let db_path = std::env::var("DATABASE_PATH")
        .unwrap_or_else(|_| "data/sip_users.db".to_string());
    let bind_addr = std::env::var("IVR_BIND_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:8090".to_string());

    println!("📁 Database : {}", db_path);
    println!("🌐 Bind     : {}", bind_addr);

    // اتصال به دیتابیس مشترک
    let pool = SqlitePool::connect(&format!("sqlite:{}?mode=rwc", db_path)).await?;
    db::init_schema(&pool).await?;

    let state: Arc<SqlitePool> = Arc::new(pool);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        // ── Flows ──
        .route("/api/flows",               get(api::flows::list).post(api::flows::create))
        .route("/api/flows/:flow_id",      get(api::flows::get_full)
                                               .put(api::flows::update)
                                               .delete(api::flows::delete))
        .route("/api/flows/:flow_id/entry", patch(api::flows::set_entry))
        // ── Nodes ──
        .route("/api/flows/:flow_id/nodes", post(api::nodes::create))
        .route("/api/nodes/:node_id",       get(api::nodes::get)
                                                .put(api::nodes::update)
                                                .delete(api::nodes::delete))
        // ── Branches ──
        .route("/api/nodes/:node_id/branches", post(api::branches::create))
        .route("/api/branches/:branch_id",     put(api::branches::update)
                                                   .delete(api::branches::delete))
        .layer(cors)
        .with_state(state);

    println!("\n✅ IVR Builder ready — http://{}\n", bind_addr);

    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
