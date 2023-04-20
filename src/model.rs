use std::collections::HashMap; 
use std::io::{Error, ErrorKind};
use std::time::{Duration, SystemTime, SystemTimeError};
use serde::{Serialize, Deserialize};
use humantime::format_duration;

impl From<SystemTimeError> for Error {
    fn from(err: SystemTimeError) -> Error {
        Error::new(ErrorKind::Other, err)
    }
}

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
    start_time: SystemTime,
    end_time: Option<SystemTime>,
    notes: Option<String>,
}

impl Log {
    pub fn new(notes: Option<String>) -> Self {
        Self {
            start_time: SystemTime::now(),
            end_time: None,
            notes,
        }
    }

    pub fn is_running(&self) -> bool {
        self.end_time.is_none()
    }

    fn stop(&mut self) {
        if self.end_time.is_none() {
            self.end_time = Some(SystemTime::now());
        }
    }

    pub fn duration(&self) -> std::io::Result<String> {
        let dur: Duration = match self.end_time {
            Some(end_time) => self.start_time.duration_since(end_time),
            None => self.start_time.elapsed(),
        }?;
        Ok(format_duration(dur).to_string())
    }
}
