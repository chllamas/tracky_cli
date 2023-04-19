mod db;
mod model;

use clap::{ Args, Parser, Subcommand };

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Create a new tracker log")]
    New(NewTracker),

    #[command(about = "Start timer")]
    Start,

    #[command(about = "Stop timer")]
    Stop,

    #[command(about = "Current timer")]
    Current,

    #[command(about = "Switch contexts to another timer")]
    Switch,

    #[command(about = "Displays all logs in tracker")]
    Logs,

    #[command(about = "Displays all trackers")]
    List,
}

#[derive(Args)]
struct NewTracker {
    name: String,
}

fn main() -> std::io::Result<()> {
    use db::load_data;
    use model::App;

    let args = Cli::parse();
    let data: App = load_data();

    match &args.command {
        Commands::New(details) => {
            println!("Creating new tracker of name {:?}", details.name)
        },
        _ => todo!(),
    };

    Ok(())
}
