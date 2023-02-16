use std::time::{Instant, SystemTime};

pub struct StartRecord {
    pub instant: Instant,
    pub time: SystemTime,
}

impl StartRecord {
    pub fn now() -> Self {
        StartRecord {
            instant: Instant::now(),
            time: SystemTime::now(),
        }
    }
}
