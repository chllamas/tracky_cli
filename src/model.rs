use std::collections::HashMap; 
use chrono::{DateTime, Timelike, Local};
use serde::{Serialize, Deserialize};
use termion::color;

#[allow(unused)]
pub enum TrackerError {
    NoneSelected,
    DoesNotExist(String),
    AlreadyExists(String),
    AlreadyRunning,
    IsNotRunning,
    NoLogs,
}

#[derive(Serialize, Deserialize)]
pub struct App {
    trackers: HashMap<String, Tracker>,
    curr: Option<String>,
}

// refPackage, isCurrent
struct TrackerPackage(String, bool);

impl App {
    fn decide_title(&self, title: Option<&str>) -> Result<TrackerPackage, TrackerError> {
        match (title, self.curr.as_deref()) {
            (Some(t), _) => Ok(TrackerPackage(t.to_string(), false)),
            (_, Some(t)) => Ok(TrackerPackage(t.to_string(), true)),
            _ => Err(TrackerError::NoneSelected),
        }
    }

    fn get_tracker(&self, title: Option<&str>) -> Result<&Tracker, TrackerError> {
        let t: TrackerPackage = self.decide_title(title)?;

        if let Some(tracker) = self.trackers.get(&t.0) {
            Ok(tracker)
        } else if t.1 {
            Err(TrackerError::NoneSelected)
        } else {
            Err(TrackerError::DoesNotExist(t.0.to_string()))
        }
    }

    fn get_tracker_mut(&mut self, title: Option<&str>) -> Result<&mut Tracker, TrackerError> {
        let t: TrackerPackage = self.decide_title(title)?;

        if let Some(tracker) = self.trackers.get_mut(&t.0) {
            Ok(tracker)
        } else if t.1 {
            Err(TrackerError::NoneSelected)
        } else {
            Err(TrackerError::DoesNotExist(t.0.to_string()))
        }
    }

    pub fn new() -> Self {
        Self {
            trackers: HashMap::new(),
            curr: None
        }
    }

    pub fn output_state_of_tracker(&self, title: Option<&str>) -> Result<String, TrackerError> {
        let tracker: &Tracker = self.get_tracker(title)?;
        let mut ret: Vec<String> = vec![];

        ret.push(format!(
                "{}: {}{}{}",
                tracker.get_title(),
                color::Fg(color::LightCyan),
                if tracker.is_running() {"running"} else {"idle"},
                color::Fg(color::Reset)
        ));

        ret.push(tracker.get_last_3_logs());

        Ok(ret.join("\n"))
    }

    pub fn new_tracker<'a>(&'a mut self, title: &'a str) -> Result<String, TrackerError> {
        if !self.trackers.contains_key(title) {
            self.trackers.insert(String::from(title), Tracker::new(title));
            if self.curr.is_none() {
                self.curr = Some(String::from(title));
            }
            Ok(title.to_string())
        } else {
            Err(TrackerError::AlreadyExists(title.to_string()))
        }
    }

    pub fn del_tracker(&mut self, title: Option<&str>) -> Result<String, TrackerError> {
        let target: String = self.decide_title(title).map(|t| t.0)?;

        self.trackers
            .remove(&target)
            .map(|t| t.get_title().to_string())
            .ok_or(TrackerError::DoesNotExist(title.unwrap_or("").to_string()))
    }

    pub fn run_tracker(&mut self, title: Option<&str>, notes: Option<&str>) -> Result<String, TrackerError> {
        let tracker: &mut Tracker = self.get_tracker_mut(title)?;

        tracker
            .up(notes)
            .map(|()| "Running...".to_string()) 
    }

    pub fn end_tracker(&mut self, title: Option<&str>) -> Result<String, TrackerError> {
        let tracker: &mut Tracker = self.get_tracker_mut(title)?;

        tracker.down().map(|n| format!("Stopping... {}", n))
    }

    pub fn swp_tracker<'a>(&'a mut self, title: &'a str) -> Result<&'a str, TrackerError> {
        if self.trackers.contains_key(title) {
            self.curr = Some(String::from(title));
            Ok(title)
        } else {
            Err(TrackerError::DoesNotExist(title.to_string()))
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
    pub fn log_tracker(&self, title: Option<&str>) -> Result<String, TrackerError> {
        let tracker: &Tracker = self.get_tracker(title)?;

        self.print_all_logs(tracker);
        Ok(title.unwrap().to_string())
    }

    pub fn list_all_trackers(&self) {
        println!("{}=== Tracky ==={}", color::Fg(color::Cyan), color::Fg(color::Reset));

        if self.trackers.is_empty() {
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

    pub fn is_running(&self) -> bool {
        if let Some(log) = self.last_log() {
            log.is_running()
        } else {
            false
        }
    }

    fn last_log(&self) -> Option<&Log> {
        let len = self.logs.len();
        if len > 0 {
            Some(&self.logs[len - 1])
        } else {
            None
        }
    }

    fn last_log_mut(&mut self) -> Option<&mut Log> {
        let len = self.logs.len();
        if len > 0 {
            Some(&mut self.logs[len - 1])
        } else {
            None
        }
    }

    pub fn up(&mut self, notes: Option<&str>) -> Result<(), TrackerError> {
        if let Some(log) = self.last_log() {
            if log.is_running() {
                return Err(TrackerError::AlreadyRunning);
            }
        }
        self.logs.push(Log::new(notes.map(|s| s.to_string())));
        Ok(())
    }

    pub fn down(&mut self) -> Result<String, TrackerError> {
        match self.last_log_mut() {
            Some(log) => {
                if log.is_running() {
                    log.stop();
                    Ok(log.get_note().to_string())
                } else {
                    Err(TrackerError::IsNotRunning)
                }
            },
            None => Err(TrackerError::NoLogs),
        }
    }

    pub fn get_last_3_logs(&self) -> String {
        let mut ret: Vec<String> = vec![];
        let last_three_logs: &[Log] = if self.logs.len() >= 3 {&self.logs[self.logs.len() - 3 ..]} else {&self.logs[..]};
        for log in last_three_logs.iter().rev() {
            if log.is_running() {
                ret.push(format!("{}{} {}{}", color::Fg(color::LightCyan), log.duration(), log.get_note(), color::Fg(color::Reset)));
            } else {
                ret.push(format!("{} -> {} {}", log.time0(), log.time1().unwrap(), log.get_note()));
            }
        }
        ret.join("\n")
    }

    pub fn ouput_last_3_logs(&self) {
        println!("{}", self.get_last_3_logs());
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
        self.notes.as_deref().unwrap_or("<untitled>")
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



#[cfg(test)]
mod tests {
    use super::App;

    // what do we test?
    // printing ls in order
    // accurately getting status indicators
    // accurately adding a new item
    // accurately removing item 
    // accurately updating start and end 
    // accurately logging all time tracks 
}
