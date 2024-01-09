pub mod schema;
pub use schema::Schema;

// Re-export annotation macro
pub use eui_derive::eui;

// Marker traits
pub trait Status: Schema + serde::Serialize + Send {}
pub trait Command: Schema + for<'a> serde::Deserialize<'a> + Send + 'static {}

use axum::{
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use tower_http::services::ServeDir;

use log::*;
use tokio::{
    net::ToSocketAddrs,
    sync::mpsc::{Receiver, Sender},
};

async fn serve<A: ToSocketAddrs, S: Status, C: Command>(
    addr: A,
    status_rx: Receiver<S>,
    command_tx: Sender<C>,
) {
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("Listening on {}", listener.local_addr().unwrap());

    async fn handle_404() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Not found")
    }

    let serve_dir = ServeDir::new(if option_env!("EUI_DEV").is_some() {
        "cljs_frontend/public_dev/"
    } else {
        "cljs_frontend/public_release/"
    })
    .not_found_service(handle_404.into_service());

    let html = Html(format!(
        r#"
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <link rel="stylesheet" type="text/css" href="style.css">
  </head>
  <body>
    <div id="app-container"></div>
    <script>
      SCHEMA_STATUS = {};
      SCHEMA_COMMAND = {};
    </script>
    <script src="js/main.js"></script>
  </body>
</html>
"#,
        serde_json::to_string(S::SCHEMA).unwrap(),
        serde_json::to_string(C::SCHEMA).unwrap(),
    ));

    let app = Router::new()
        .route("/", get(html))
        .route(
            "/cmd",
            post(move |Json(cmd): Json<C>| async move {
                let _ = command_tx.send(cmd).await;
                "OK"
            }),
        )
        .fallback_service(serve_dir);

    axum::serve(listener, app).await.unwrap();
}

pub fn serve_blocking<A: ToSocketAddrs, S: Status, C: Command>(
    addr: A,
    status_rx: Receiver<S>,
    command_tx: Sender<C>,
) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async { serve(addr, status_rx, command_tx).await })
}
