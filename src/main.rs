// src/main.rs
//
// IVR Builder — سرویس REST API برای ساخت فلوهای IVR
//
// متغیرهای محیطی (.env یا OS):
//   DATABASE_PATH   مسیر SQLite مشترک با PBX  (پیش‌فرض: data/sip_users.db)
//   IVR_BIND_ADDR   آدرس listen                (پیش‌فرض: 0.0.0.0:8090)
//
// Swagger UI  → http://localhost:8090/swagger-ui
// OpenAPI JSON → http://localhost:8090/api-docs/openapi.json

mod error;
mod models;
mod db;
mod api;
mod openapi;

use std::sync::Arc;
use axum::{
    routing::{get, patch, post, put},
    response::Html,
    Json, Router,
};
use sqlx::SqlitePool;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("========================================");
    println!("   🎛️  IVR Builder API");
    println!("========================================\n");

    match dotenvy::dotenv() {
        Ok(path) => println!("📄 .env loaded from: {}", path.display()),
        Err(dotenvy::Error::Io(_)) => println!("ℹ️  No .env file found — using OS env vars"),
        Err(e) => eprintln!("⚠️  .env parse error: {}", e),
    }

    let db_path   = std::env::var("DATABASE_PATH")
        .unwrap_or_else(|_| "data/sip_users.db".to_string());
    let bind_addr = std::env::var("IVR_BIND_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:8090".to_string());

    println!("📁 Database : {}", db_path);
    println!("🌐 Bind     : {}", bind_addr);

    let pool = SqlitePool::connect(&format!("sqlite:{}?mode=rwc", db_path)).await?;
    db::init_schema(&pool).await?;

    let state: Arc<SqlitePool> = Arc::new(pool);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        // ── Swagger UI + OpenAPI spec ──
        .route("/swagger-ui",            get(swagger_ui))
        .route("/api-docs/openapi.json", get(openapi_json))
        // ── Flows ──
        .route("/api/flows",                get(api::flows::list).post(api::flows::create))
        .route("/api/flows/:flow_id",       get(api::flows::get_full)
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

    println!("\n✅ IVR Builder ready!");
    println!("   API     → http://{}/api/flows", bind_addr);
    println!("   Swagger → http://{}/swagger-ui\n", bind_addr);

    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn openapi_json() -> Json<serde_json::Value> {
    Json(openapi::openapi_spec())
}

async fn swagger_ui() -> Html<&'static str> {
    Html(SWAGGER_HTML)
}

const SWAGGER_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8"/>
  <meta name="viewport" content="width=device-width, initial-scale=1"/>
  <title>IVR Builder — Swagger UI</title>
  <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5/swagger-ui.css"/>
  <style>
    body { margin: 0; }
    #swagger-ui .topbar { background: #1e293b; }
  </style>
</head>
<body>
  <div id="swagger-ui"></div>
  <script src="https://unpkg.com/swagger-ui-dist@5/swagger-ui-bundle.js"></script>
  <script src="https://unpkg.com/swagger-ui-dist@5/swagger-ui-standalone-preset.js"></script>
  <script>
    window.onload = () => {
      SwaggerUIBundle({
        url: "/api-docs/openapi.json",
        dom_id: '#swagger-ui',
        presets: [SwaggerUIBundle.presets.apis, SwaggerUIStandalonePreset],
        layout: "StandaloneLayout",
        deepLinking: true,
        tryItOutEnabled: true,
        displayRequestDuration: true,
        filter: true,
      });
    };
  </script>
</body>
</html>"#;