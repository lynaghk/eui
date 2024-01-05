pub mod schema;

use axum::{
    handler::HandlerWithoutStateExt, http::StatusCode, response::Html, routing::get, Router,
};
use tower_http::services::ServeDir;

pub use eui_derive::Schema;

//TODO, don't re-export.
pub use bevy_reflect;

use log::*;
use tokio::net::ToSocketAddrs;

async fn serve<T: schema::Schema, A: ToSocketAddrs>(addr: A) {
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
    <script> SCHEMA = {}; </script>
    <script src="js/main.js"></script>
  </body>
</html>
"#,
        serde_json::to_string(T::SCHEMA).unwrap()
    ));

    let app = Router::new()
        .route("/", get(html))
        .fallback_service(serve_dir);

    axum::serve(listener, app).await.unwrap();
}

pub fn serve_blocking<T: schema::Schema, A: ToSocketAddrs>(addr: A) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async { serve::<T, A>(addr).await })
}
