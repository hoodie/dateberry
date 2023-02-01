use axum::{
    http::header::{self, HeaderName},
    response::Html,
    routing::get,
    Router,
};
use chrono::Datelike;
use num_traits::FromPrimitive;
use sync_wrapper::SyncWrapper;

pub mod favicon;

async fn favicon() -> axum::response::Result<([(HeaderName, &'static str); 1], String)> {
    // yapp, this is probably wrong for some people for half a day
    let today = chrono::Utc::now().date_naive();
    let month = &chrono::Month::from_u32(today.month()).unwrap().name()[0..3];
    let day = today.day();
    let svg = favicon::todays_calendar(month, &format!("{day}"))
        .map_err(|e| axum::response::ErrorResponse::from(e.to_string()))?;
    Ok(([(header::CONTENT_TYPE, "image/svg+xml")], svg))
}

async fn index() -> Html<&'static str> {
    Html(include_str!("index.html"))
}

#[shuttle_service::main]
async fn axum() -> shuttle_service::ShuttleAxum {
    let router = setup_router();

    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}

pub fn setup_router() -> Router {
    Router::new()
        .route("/calendar.svg", get(favicon))
        .route("/", get(index))
}
