use std::collections::HashMap; 
use chrono::{DateTime, Timelike, Local};
use serde::{Serialize, Deserialize};

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
    start_time: DateTime<Local>,
    end_time: Option<DateTime<Local>>,
    notes: Option<String>,
}

impl Log {
    pub fn new(notes: Option<String>) -> Self {
        Self {
            start_time: Local::now(),
            end_time: None,
            notes,
        }
    }

    pub fn is_running(&self) -> bool {
        self.end_time.is_none()
    }

    pub fn stop(&mut self) -> Option<String> {
        if self.end_time.is_none() {
            self.end_time = Some(Local::now());
            Some(self.duration())
        } else {
            None
        }
    }

    pub fn time0(&self) -> String {
        format!(
            "{}:{:02}:{:02}", 
            self.start_time.hour(),
            self.start_time.minute(),
            self.start_time.second()
        )
    }

    pub fn time1(&self) -> Option<String> {
        self.end_time.and_then(|t|
            Some(format!(
                "{}:{:02}:{:02}", 
                t.hour(),
                t.minute(),
                t.second()
            ))
        )
    }

    pub fn duration(&self) -> String {
        let duration: i64 = match self.end_time {
            Some(end_time) => end_time,
            _ => Local::now(),
        }.signed_duration_since(self.start_time).num_seconds();
        let hours = duration / 3600;
        let minutes = (duration % 3600) / 60;
        let seconds = duration % 60;

        match (hours, minutes, seconds) {
            (0, 0, s) => format!("{}s", s),
            (0, m, s) => format!("{}:{:02}", m, s),
            (h, m, s) => format!("{:02}:{:02}:{:02}", h, m, s), 
        }
    }
}
