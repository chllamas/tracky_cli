mod db;
mod model;

use clap::{ Args, Parser, Subcommand };
use model::App;
use termion::color;

use crate::model::TrackerError;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short='t', long=None)]
    #[arg(help="Specify tracker by name")]
    tracker: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(short_flag='n')]
    #[command(about="Create a new tracker log")]
    New(RequiredTitle),

    #[command(alias="run", short_flag='r')]
    #[command(about="Start timer")]
    Start(OptionalNote),

    #[command(alias="end")]
    #[command(about="Stop timer")]
    Stop,

    #[command(about="Switch contexts to another timer")]
    Switch(RequiredTitle),

    #[command(alias="del", short_flag='d')]
    #[command(about="Delete the current timer")]
    Delete,

    #[command(about="Displays all logs in tracker")]
    Logs,

    #[command(about="Display the status of the tracker, if running")]
    Status,

    #[command(alias="ls", short_flag='l')]
    #[command(about="Displays all trackers")]
    List,
}

#[derive(Args)]
struct OptionalNote {
    contents: Option<String>,
}

#[derive(Args)]
struct RequiredTitle {
    title: String,
}

fn default_error_handler(e: TrackerError) {
    match e {
        TrackerError::NoneSelected => println!("No tracker is selected!"),
        TrackerError::AlreadyExists(s) => println!(
            "Tracker {}{}{} already exists!",
            color::Fg(color::Blue),
            s,
            color::Fg(color::Reset)
        ),
        TrackerError::DoesNotExist(s) => println!(
            "Tracker {}{}{} does not exist!",
            color::Fg(color::Blue),
            s,
            color::Fg(color::Reset)
        ),
        TrackerError::AlreadyRunning => println!("Tracker is already running!"),
        TrackerError::IsNotRunning => println!("Tracker is not running!"),
        TrackerError::NoLogs => println!("Tracker has no logs!"),
    }
}

fn main() -> std::io::Result<()> {
    use db::{ load_data, save_data };

    let args = Cli::parse();
    let mut data: App = load_data();

    let target: Option<&str> = args.tracker.as_deref();

    match &args.command {
        /* must handle errors and print out stuff */
        Commands::New(request) => {
            match data.new_tracker(&request.title) {
                Ok(title) => println!("Inserted {}", title),
                Err(e) => default_error_handler(e),
            };
        },
        Commands::Status => {
            match data.output_state_of_tracker(target) {
                Ok(output) => println!("{}", output),
                Err(e) => default_error_handler(e),
            }
        },
        Commands::Delete => { 
            match data.del_tracker(target) {
                Ok(title) => println!("Deleted {}", title),
                Err(e) => default_error_handler(e),
            }
        },
        Commands::Start(opt_note) => { 
            match data.run_tracker(target, opt_note.contents.as_deref()) {
                Ok(output) => println!("{}", output),
                Err(e) => default_error_handler(e),
            }
        },
        Commands::Stop => { 
            match data.end_tracker(target) {
                Ok(output) => println!("{}", output),
                Err(e) => default_error_handler(e),
            }
        },
        Commands::Switch(request) => { 
            match data.swp_tracker(&request.title) {
                Ok(title) => println!("Switched to {}", title),
                Err(e) => default_error_handler(e),
            }
        },
        Commands::Logs => { 
            match data.log_tracker(target) {
                Ok(output) => println!("{}", output),
                Err(e) => default_error_handler(e),
            }
        },
        Commands::List => { data.list_all_trackers(); },
    };

    save_data(data)
}
