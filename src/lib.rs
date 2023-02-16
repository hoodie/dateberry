use axum::{
    extract::{Path, Query, RawQuery},
    http::header::{self, HeaderName},
    response::Html,
    routing::{get, post},
    Router,
};
use chrono::Datelike;
use icalendar::{Calendar, Component};
use num_traits::FromPrimitive;
use request_calendar::RequestCalendar;
use serde::{Deserialize, Serialize};
use sync_wrapper::SyncWrapper;

pub mod favicon;
pub mod request_calendar;

async fn favicon() -> axum::response::Result<([(HeaderName, &'static str); 1], String)> {
    // yapp, this is probably wrong for some people for half a day
    let today = chrono::Utc::now().date_naive();
    let month = &chrono::Month::from_u32(today.month()).unwrap().name()[0..3];
    let day = today.day();
    let svg = favicon::todays_calendar(month, &format!("{day}"))
        .map_err(|e| axum::response::ErrorResponse::from(e.to_string()))?;
    Ok(([(header::CONTENT_TYPE, "image/svg+xml")], svg))
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
enum CalendarExtension {
    Ics,
    Txt,
}

async fn calendar(
    Query(calendar): Query<RequestCalendar>,
    RawQuery(query): RawQuery,
    Path(ext): Path<CalendarExtension>,
) -> axum::response::Result<([(HeaderName, &'static str); 1], String)> {
    let mut event = icalendar::Event::from(&calendar);
    let addr = "https://dateberry.shuttleapp.rs/";
    let footer = match query {
        Some(query) => {
            format!("generated with dateberry.\nfeel free to edit this here:\n{addr}?{query}",)
        }
        None => format!(
            "generated with dateberry.\nfeel free create your own here:\n{}",
            addr
        ),
    };
    let new_description = if let Some(description) = event.get_description() {
        format!("{description}\n\n--------\n{footer}")
    } else {
        footer
    };
    event.description(&new_description);

    let calendar = Calendar::from([event]);
    let mime_type = match ext {
        CalendarExtension::Ics => "text/calendar",
        CalendarExtension::Txt => "text/plain",
    };
    println!("{calendar}\n({mime_type})");
    Ok(([(header::CONTENT_TYPE, mime_type)], calendar.to_string()))
}

async fn index(Query(calendar): Query<RequestCalendar>) -> Html<&'static str> {
    let event = icalendar::Event::from(dbg!(&calendar));
    let calendar = Calendar::from([event]);
    println!("{calendar}");
    Html(include_str!("index.html"))
}

// async fn create_calendar() -> Html<&'static str> {
//     Html(include_str!("index.html"))
// }

#[shuttle_service::main]
async fn axum() -> shuttle_service::ShuttleAxum {
    let router = setup_router();

    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}

pub fn setup_router() -> Router {
    Router::new()
        .route("/favicon.ico", get(favicon))
        .route("/favicon.svg", get(favicon))
        .route("/calendar.svg", get(favicon))
        .route("/calendar.:ext", get(calendar))
        .route("/calendar.:ext", post(calendar))
        .route("/", get(index))
}
