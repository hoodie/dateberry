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
use once_cell::sync::OnceCell;
use request_calendar::RequestCalendar;
use serde::{Deserialize, Serialize};
use start_record::StartRecord;

pub mod favicon;
pub mod request_calendar;
pub mod start_record;

static STARTED: OnceCell<StartRecord> = OnceCell::new();

async fn favicon() -> axum::response::Result<([(HeaderName, &'static str); 1], String)> {
    // yapp, this is probably wrong for some people for half a day
    let today = chrono::Utc::now().date_naive();
    let month = &chrono::Month::from_u32(today.month()).unwrap().name()[0..3];
    let day = today.day();
    let svg = favicon::todays_calendar(month, &format!("{day}"))
        .map_err(|e| axum::response::ErrorResponse::from(e.to_string()))?;
    Ok(([(header::CONTENT_TYPE, "image/svg+xml")], svg))
}

async fn uptime() -> String {
    if let Some(StartRecord { instant, time }) = STARTED.get() {
        let timestamp = humantime::Timestamp::from(*time);
        let elapsed = timestamp
            .elapsed()
            .map(humantime::Duration::from)
            .map_or("🤷".into(), |ht| ht.to_string());
        let time = humantime::Timestamp::from(*time).to_string();
        let duration = humantime::Duration::from(std::time::Instant::now() - *instant).to_string();

        format!("startet at {time}\nrunning {duration}\nrunning {elapsed} (system time)")
    } else {
        String::from("I don't know")
    }
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

pub fn record_start_time() {
    if STARTED.set(StartRecord::now()).is_err() {
        println!("start time was already recorded");
    }
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    record_start_time();
    let router = setup_router();

    Ok(router.into())
}

pub fn setup_router() -> Router {
    Router::new()
        .route("/favicon.ico", get(favicon))
        .route("/favicon.svg", get(favicon))
        .route("/calendar.svg", get(favicon))
        .route("/calendar.:ext", get(calendar))
        .route("/calendar.:ext", post(calendar))
        .route("/uptime", get(uptime))
        .route("/", get(index))
}
