use icalendar::{Component, EventLike};
use serde::Deserialize;

mod util {

    use std::{fmt, str::FromStr};

    use serde::{de, Deserialize, Deserializer};
    /// Serde deserialization decorator to map empty Strings to None,
    pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr,
        T::Err: fmt::Display,
    {
        let opt = Option::<String>::deserialize(de)?;
        match opt.as_deref() {
            None | Some("") => Ok(None),
            Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
        }
    }
}
#[derive(Debug, Deserialize)]
pub struct RequestCalendar {
    #[serde(default, deserialize_with = "util::empty_string_as_none")]
    pub summary: Option<String>,

    #[serde(default, deserialize_with = "util::empty_string_as_none")]
    pub description: Option<String>,

    #[serde(default, deserialize_with = "util::empty_string_as_none")]
    pub location: Option<String>,

    #[serde(default, deserialize_with = "util::empty_string_as_none")]
    pub start: Option<iso8601::DateTime>,

    #[serde(default, deserialize_with = "util::empty_string_as_none")]
    pub end: Option<iso8601::DateTime>,

    #[serde(default, deserialize_with = "util::empty_string_as_none")]
    pub duration: Option<String>,

    #[serde(default, deserialize_with = "util::empty_string_as_none")]
    pub timezone: Option<String>,
    // pub reminder: Option<String>,
}

impl From<&RequestCalendar> for icalendar::Event {
    fn from(req: &RequestCalendar) -> Self {
        let start = req.start.and_then(|start| match start.into_naive() {
            Some(start) => Some(start),
            None => {
                println!("failed to parse start {start}");
                None
            }
        });

        let end = req.end.and_then(|end| match end.into_naive() {
            Some(start) => Some(start),
            None => {
                println!("failed to parse end {end}");
                None
            }
        });
        let timezone = req
            .timezone
            .as_ref()
            .and_then(|tz| match tz.parse::<chrono_tz::Tz>() {
                Ok(tz) => Some(tz),
                Err(err) => {
                    println!("failed to parse tz {err}");
                    None
                }
            });

        let mut event = icalendar::Event::new();

        if let Some(summary) = &req.summary {
            event.summary(summary);
        }

        if let Some(description) = &req.description {
            event.description(description);
        }

        if let Some(location) = &req.location {
            event.location(location);
        }
        if let Some((start, timezone)) = start.zip(timezone) {
            event.starts((start, timezone));
        }
        if let Some((end, timezone)) = end.zip(timezone) {
            event.ends((end, timezone));
        }
        event
    }
}
