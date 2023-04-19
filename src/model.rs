use serde::{ Serialize, Deserialize };
use chrono::{ serde::{ ts_seconds, ts_seconds_option }, DateTime, Utc };

#[derive(Serialize, Deserialize)]
pub struct App {
    pub trackers: Vec<Tracker>,
    pub current: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct Tracker {
    title: String,
    logs: Vec<Log>,
}

impl Tracker {
    pub fn new(title: String) -> Self {
        Self {
            logs: Vec::new(),
            title,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Log {
    #[serde(with = "ts_seconds")]
    start_time: DateTime<Utc>,

    #[serde(with = "ts_seconds_option")]
    end_time: Option<DateTime<Utc>>,

    notes: Option<String>,
}

impl Log {
    pub fn new(notes: Option<String>) -> Self {
        Self {
            start_time: Utc::now(),
            end_time: None,
            notes
        }
    }

    fn stop(&mut self) {
        if self.end_time.is_none() {
            self.end_time = Some(Utc::now());
        }
    }

    fn duration(&self) -> Option<std::time::Duration> {
        self.end_time.map(|end| end.signed_duration_since(self.start_time).to_std().unwrap())
    }
}
