use chrono::Datelike;
use dateberry::favicon::todays_calendar;
use num_traits::FromPrimitive;

fn main() {
    let today = chrono::Utc::now().date_naive();
    let month = &chrono::Month::from_u32(today.month()).unwrap().name()[0..3];
    let day = today.day();
    println!("{}", todays_calendar(month, &format!("{day}")).unwrap())
}
