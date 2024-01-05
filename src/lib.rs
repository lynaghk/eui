pub mod schema;
pub use schema::Schema;

pub use eui_derive::Schema;

pub trait Status: Schema + serde::Serialize + Send + Clone {}

// TODO: understand why I need the 'static here.
pub trait Command: Schema + for<'a> serde::Deserialize<'a> + Send + Clone + 'static {}

use axum::{
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use tower_http::services::ServeDir;

//TODO, don't re-export.
pub use bevy_reflect;

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
    info!("EUI listening on {}", listener.local_addr().unwrap());

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
            // post(move |cmd: String| async move {
            //     if let Ok(cmd) = serde_json::from_str::<C>(&cmd) {
            //         let cmd = cmd.to_owned();

            //         let _ = command_tx.send(cmd).await;
            //     }
            //     "OK"
            // }),
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
