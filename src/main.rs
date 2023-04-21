mod db;
mod model;

use clap::{ Args, Parser, Subcommand };
use model::App;

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
    Start(OptionalTitle),

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
struct RequiredTitle {
    title: String,
}

fn main() -> std::io::Result<()> {
    use db::{ load_data, save_data };

    let args = Cli::parse();
    let mut data: App = load_data();

    match &args.command {
        Commands::New(request) => { data.new_tracker(&request.title); },
        Commands::Status => { data.output_state_of_tracker(); },
        Commands::Delete(opt_request) => { data.del_tracker(opt_request.title.as_deref()); },
        Commands::Start(opt_request) => { data.run_tracker(opt_request.title.as_deref()); },
        Commands::Stop(opt_request) => { data.end_tracker(opt_request.title.as_deref()); },
        Commands::Switch(request) => { data.swp_tracker(&request.title); },
        Commands::Logs(opt_request) => { data.log_tracker(opt_request.title.as_deref()); },
        Commands::List => { data.list_all(); },
    };

    save_data(data)
}
