use std::collections::HashMap;
use serde::{ Serialize, Deserialize };
use chrono::{ serde::{ ts_seconds, ts_seconds_option }, DateTime, Utc };

#[derive(Serialize, Deserialize)]
pub struct App {
    pub trackers: HashMap<String, Tracker>,
    pub current: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Tracker {
    title: String,
    pub logs: Vec<Log>,
}

impl Tracker {
    pub fn new(title: &str) -> Self {
        Self {
            title: String::from(title),
            logs: Vec::new(),
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
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
    fn new(notes: Option<String>) -> Self {
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
