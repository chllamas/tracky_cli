mod db;
mod model;

use std::io::{ Error, ErrorKind };
use clap::{ Args, Parser, Subcommand };
use model::{ App, Log, Tracker };

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(short_flag='n')]
    #[command(about="Create a new tracker log")]
    New(NewTracker),

    #[command(alias="run", short_flag='r')]
    #[command(about="Start timer")]
    Start,

    #[command(alias="end")]
    #[command(about="Stop timer")]
    Stop,

    #[command(short_flag='c')]
    #[command(alias="curr", alias="cur")]
    #[command(about="Current timer")]
    Current,

    #[command(about="Switch contexts to another timer")]
    Switch(SwitchRequest),

    #[command(alias="del", short_flag='d')]
    #[command(about="Delete the current timer")]
    Delete(OptionalTitle),

    #[command(about="Displays all logs in tracker")]
    Logs,

    #[command(about="Display the status of the tracker, if running")]
    Status,

    #[command(alias="ls", short_flag='l')]
    #[command(about="Displays all trackers")]
    List,
}

#[derive(Args)]
struct OptionalTitle {
    title: Option<String>,
}

#[derive(Args)]
struct NewTracker {
    title: String,
}

#[derive(Args)]
struct SwitchRequest {
    to: String,
}

fn get_tracker(data: &mut App) -> std::io::Result<&Tracker> {
    if let Some(title) = data.current.as_ref() {
        match data.trackers.get(title) {
            Some(t) => Ok(t),
            None => { 
                data.current = None;
                Err(Error::new(ErrorKind::NotFound, "Selected tracker no longer exists"))
            },
        }
    } else {
        Err(Error::new(ErrorKind::Other, "No tracker selected to start from"))
    }
}

fn get_tracker_mut(data: &mut App) -> std::io::Result<&mut Tracker> {
    if let Some(title) = data.current.as_ref() {
        match data.trackers.get_mut(title) {
            Some(t) => Ok(t),
            None => { 
                data.current = None;
                Err(Error::new(ErrorKind::NotFound, "Selected tracker no longer exists"))
            },
        }
    } else {
        Err(Error::new(ErrorKind::Other, "No tracker selected to start from"))
    }
}

fn get_last_log(data: &mut App) -> Option<&Log> {
    let t: &Tracker = get_tracker(data).ok()?;
    let size: usize = t.logs.len();
    if size > 0 {
        Some(&t.logs[size - 1])
    } else {
        None
    }
}

fn get_last_log_mut(data: &mut App) -> Option<&mut Log> {
    let t: &mut Tracker = get_tracker_mut(data).ok()?;
    let size: usize = t.logs.len();
    if size > 0 {
        Some(&mut t.logs[size - 1])
    } else {
        None
    }
}

fn main() -> std::io::Result<()> {
    use db::{ load_data, save_data };

    let args = Cli::parse();
    let mut data: App = load_data();

    match &args.command {
        Commands::New(details) => {
            let nt: Tracker = Tracker::new(&details.title);
            data.trackers.insert(details.title.clone(), nt);
            if data.current.is_none() {
                data.current = Some(details.title.clone());
            }
            println!("Created new tracker {}", &details.title);
        },
        Commands::Status => {
            if let Some(last_log) = get_last_log(&mut data) {
                if last_log.is_running() {
                    println!("Running for: {}", last_log.duration());
                } else {
                    println!("Tracker is idle");
                }
            } else {
                println!("No tracker selected");
            }
        },
        Commands::Delete(req) => {
            if let Some(title) = req.title.as_ref().or(data.current.as_ref()) {
                // TODO: currently doesnt check if it even exists
                // TODO: sets data.current to none even if it's not the one we're removing
                println!("Removed {}", title);
                data.trackers.remove(title);
                data.current = None;
            } else {
                println!("No tracker selected");
            }
        },
        Commands::Start => {
            let t: &mut Tracker = get_tracker_mut(&mut data)?;
            t.logs.push(Log::new(None));
        },
        Commands::Stop => {
            if let Some(last_log) = get_last_log_mut(&mut data) {
                if let Some(duration_str) = last_log.stop() {
                    println!("Ended log after {}", duration_str);
                }
            }
        },
        Commands::Current => {
            let t: &Tracker = get_tracker(&mut data)?;
            println!("Current tracker: {}", t.get_title())
        },
        Commands::Switch(req) => {
            match data.trackers.contains_key(&req.to) {
                true => {
                    data.current = Some(req.to.clone());
                    println!("Switched to {}", &req.to);
                },
                _ => println!("{} does not exist", &req.to),
            };
        },
        Commands::Logs => {
            let t: &Tracker = get_tracker(&mut data)?;
            for l in t.logs.iter() {
                println!("{} -> {}", l.time0(), l.time1().unwrap_or(String::new()));
            }
        },
        Commands::List => {
            // it'd be better to have a separate vec stored in struct for keys pre-sorted
            if data.trackers.len() > 0 {
                let mut sorted_keys: Vec<&str> = data.trackers
                    .keys()
                    .map(|k| k.as_str())
                    .collect();
                sorted_keys.sort();
                for k in sorted_keys {
                    println!("{} {}",
                        if data.current.as_ref().map_or("", String::as_str) == k {">"} else {" "},
                        k
                    )
                };
            } else {
                println!("No trackers exist");
            }
        },
    };

    save_data(data)
}
