use crate::model::App;

pub fn load_data() -> App {
    /*
    // get path to our data file
    let mut path = dirs::config_dir().unwrap();
    path.push("tracky_cli");
    path.push("data.json");

    let json: String = fs::read_to_string(&path).unwrap_or(String::new());
    let data: App = serde_json::from_str(&json).unwrap_or(
        App {
            trackers: Vec::new(),
            current: None,
        }
    );
    */
    App {
        trackers: Vec::new(),
        current: None,
    }
}
