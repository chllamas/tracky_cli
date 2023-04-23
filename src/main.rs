mod db;
mod model;

use clap::{ Args, Parser, Subcommand };
use model::App;

use crate::model::TrackerError;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(short_flag='n')]
    #[command(about="Create a new tracker log")]
    New(RequiredTitle),

    #[command(alias="run", short_flag='r')]
    #[command(about="Start timer")]
    Start(OptionalTitleAndNote),

    #[command(alias="end")]
    #[command(about="Stop timer")]
    Stop(OptionalTitle),

    #[command(about="Switch contexts to another timer")]
    Switch(RequiredTitle),

    #[command(alias="del", short_flag='d')]
    #[command(about="Delete the current timer")]
    Delete(OptionalTitle),

    #[command(about="Displays all logs in tracker")]
    Logs(OptionalTitle),

    #[command(about="Display the status of the tracker, if running")]
    Status(OptionalTitle),

    #[command(alias="ls", short_flag='l')]
    #[command(about="Displays all trackers")]
    List,
}

#[derive(Args)]
struct OptionalTitle {
    title: Option<String>,
}

#[derive(Args)]
struct OptionalTitleAndNote {
    title: Option<String>,
    note: Option<String>,
}

#[derive(Args)]
struct RequiredTitle {
    title: String,
}

fn default_error_handler(e: TrackerError) {
    match e {
        TrackerError::NoneSelected => println!("No tracker is selected!"),
        TrackerError::AlreadyExists => println!("Tracker of same name already exists!"),
        TrackerError::
}

fn main() -> std::io::Result<()> {
    use db::{ load_data, save_data };

    let args = Cli::parse();
    let mut data: App = load_data();

    match &args.command {
        /* must handle errors and print out stuff */
        Commands::New(request) => {
            match data.new_tracker(&request.title) {
                Ok(title) => println!("Inserted {}", title),
                Err(TrackerError::AlreadyExists) => println!("{} already exists!", &request.title),
                _ => unreachable!(),
            };
        },
        Commands::Status(opt_request) => {
            match data.output_state_of_tracker(opt_request.title.as_deref()) {
                Ok(output) => println!(output),
                Err(TrackerError::NoneSelected) => println!("No tracker selected"),

            }
        },
        Commands::Delete(opt_request) => { data.del_tracker(opt_request.title.as_deref()); },
        Commands::Start(opt_request) => { data.run_tracker(opt_request.title.as_deref(), opt_request.note.as_deref()); },
        Commands::Stop(opt_request) => { data.end_tracker(opt_request.title.as_deref()); },
        Commands::Switch(request) => { data.swp_tracker(&request.title); },
        Commands::Logs(opt_request) => { data.log_tracker(opt_request.title.as_deref()); },
        Commands::List => { data.list_all_trackers(); },
    };

    save_data(data)
}
