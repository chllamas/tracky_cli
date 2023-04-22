use std::collections::HashMap; 
use chrono::{DateTime, Timelike, Local};
use serde::{Serialize, Deserialize};
use termion::color;

#[allow(unused)]
pub enum TrackerError {
    NoneSelected,
    DoesNotExist,
    AlreadyExists,
    AlreadyRunning,
    IsNotRunning,
}

#[derive(Serialize, Deserialize)]
pub struct App {
    trackers: HashMap<String, Tracker>,
    curr: Option<String>,
}

#[allow(dead_code)]
#[allow(unused)]
impl App {
    pub fn new() -> Self {
        Self {
            trackers: HashMap::new(),
            curr: None
        }
    }

    pub fn current(&self) -> Option<&Tracker> {
        self.curr
            .as_ref()
            .and_then(|title| self.trackers.get(title))
    }

    pub fn output_state_of_tracker(&self, title: Option<&str>) -> Result<String, TrackerError> {
        todo!()
    }

    pub fn new_tracker<'a>(&'a mut self, title: &'a str) -> Result<&str, TrackerError> {
        if !self.trackers.contains_key(title) {
            self.trackers.insert(String::from(title), Tracker::new(title));
            if self.curr.is_none() {
                self.curr = Some(String::from(title));
            }
            Ok(title)
        } else {
            Err(TrackerError::AlreadyExists)
        }
    }

    pub fn del_tracker(&mut self, title: Option<&str>) -> Result<&str, TrackerError> {
        match title {
            Some(t) => todo!(),
            None => todo!(),
        }
    }

    pub fn run_tracker(&mut self, title: Option<&str>) -> Result<String, TrackerError> {
        let t: (&str, bool) = match (title, self.curr.as_deref()) {
            (Some(t), _) => Ok((t, false)),
            (_, Some(t)) => Ok((t, true)),
            _ => Err(TrackerError::NoneSelected),
        }?;

        if let Some(tracker) = self.trackers.get(t.0) {
            
            Ok(t.0.to_string())
        } else if t.1 {
            self.curr = None;
            Err(TrackerError::NoneSelected)
        } else {
            Err(TrackerError::DoesNotExist)
        }f 
    }

    pub fn end_tracker(&mut self, title: Option<&str>) -> Result<&str, TrackerError> {
        todo!()
    }

    pub fn swp_tracker<'a>(&'a mut self, title: &'a str) -> Result<&'a str, TrackerError> {
        if self.trackers.contains_key(title) {
            self.curr = Some(String::from(title));
            Ok(title)
        } else {
            Err(TrackerError::DoesNotExist)
        }
    }

    fn print_all_logs(&self, tracker: &Tracker) {
        for log in tracker.logs.iter() {
            if log.is_running() {
                println!("{} {}", log.duration(), log.get_note());
            } else {
                println!("{} -> {} {}", log.time0(), log.time1().as_deref().unwrap_or(""), log.get_note());
            }
        }
    }

    /* Returns &str of the title printed out for */
    pub fn log_tracker(&mut self, title: Option<&str>) -> Result<String, TrackerError> {
        let t: (&str, bool) = match (title, self.curr.as_deref()) {
            (Some(t), _) => Ok((t, false)),
            (_, Some(t)) => Ok((t, true)),
            _ => Err(TrackerError::NoneSelected),
        }?;
        
        if let Some(tracker) = self.trackers.get(t.0) {
            self.print_all_logs(tracker);
            Ok(t.0.to_string())
        } else if t.1 {
            self.curr = None;
            Err(TrackerError::NoneSelected)
        } else {
            Err(TrackerError::DoesNotExist)
        }
    }

    pub fn list_all_trackers(&self) {
        println!("{}=== Tracky ==={}", color::Fg(color::Cyan), color::Fg(color::Reset));

        if self.trackers.len() == 0 {
            println!("{}No Trackers Created{}", color::Fg(color::Red), color::Fg(color::Reset));
            return;
        }

        let mut keys: Vec<&str> = self.trackers
            .keys()
            .map(|s| s.as_str())
            .collect();
        keys.sort();

        let curr_title: &str = self.curr.as_deref().unwrap_or("");
        for title in keys {
            println!("{}{}", if title == curr_title {"> "} else {"  "}, title);
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Tracker {
    title: String,
    logs: Vec<Log>,
}

#[allow(dead_code)]
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

    fn last_log(&self) -> Option<&Log> {
        let len = self.logs.len();
        if len > 0 {
            Some(&self.logs[len - 1])
        } else {
            None
        }
    }

    pub fn start_up(&mut self, notes: Option<&str>) -> Result<(), TrackerError> {
        if let Some(log) = self.last_log() {
            if log.is_running() {
                return Err(TrackerError::AlreadyRunning);
            }
        }
        self.logs.push(Log::new(notes.map(|s| s.to_string())));
        Ok(())
    }

    pub fn ouput_last_3_logs(&self) {
        let last_three_logs: &[Log] = &self.logs[self.logs.len() - 3 ..];
        for log in last_three_logs {
            if log.is_running() {
                println!("... {} {}", log.duration(), log.get_note());
            } else {
                println!("{} -> {} {}", log.time0(), log.time1().unwrap(), log.get_note());
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Log {
    start_time: DateTime<Local>,
    end_time: Option<DateTime<Local>>,
    notes: Option<String>,
}

#[allow(dead_code)]
impl Log {
    pub fn new(notes: Option<String>) -> Self {
        Self {
            start_time: Local::now(),
            end_time: None,
            notes,
        }
    }

    pub fn get_note(&self) -> &str {
        self.notes.as_deref().unwrap_or("")
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
        self.end_time.map(|t|
            format!(
                "{}:{:02}:{:02}", 
                t.hour(),
                t.minute(),
                t.second()
            )
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
