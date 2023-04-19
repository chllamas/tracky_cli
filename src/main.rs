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
    Switch(SwitchRequest),

    #[command(about = "Delete the current timer")]
    Delete,

    #[command(about = "Displays all logs in tracker")]
    Logs,

    #[command(about = "Displays all trackers")]
    List,
}

#[derive(Args)]
struct NewTracker {
    title: String,
}

#[derive(Args)]
struct SwitchRequest {
    to: String,
}

fn main() -> std::io::Result<()> {
    use db::{ load_data, save_data };
    use model::{ App, Tracker };

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
        Commands::Delete => {

        },
        Commands::Start => {

        },
        Commands::Stop => {

        },
        Commands::Current => {
            if let Some(title) = data.current.as_ref() {
                match data.trackers.get(title) {
                    Some(t) => println!("Current tracker: {}", t.get_title()),
                    _ => { data.current = None },
                }
            } else {
                println!("No tracker selected");
            }
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
