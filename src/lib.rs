use axum::{response::Html, routing::get, Router};

use sync_wrapper::SyncWrapper;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn index() -> Html<&'static str> {
    Html(include_str!("../app/dist/index.html"))
}

#[shuttle_service::main]
async fn axum() -> shuttle_service::ShuttleAxum {
    let router = Router::new()
        .route("/hello", get(hello_world))
        .route("/", get(index));

    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}
