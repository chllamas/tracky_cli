use crate::model::App;
use std::path::PathBuf;
use std::fs;

pub fn get_file_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap();
    path.push("tracky_cli");
    // Implement below error handling
    fs::create_dir_all(&path).expect("Could not create path for saving data");
    path.push("data.json");
    path
}

pub fn load_data() -> App {
    let json: String = fs::read_to_string(&get_file_path()).unwrap_or(String::new());
    serde_json::from_str(&json).unwrap_or(App::new())
}

pub fn save_data(data: App) -> std::io::Result<()> {
    let json = serde_json::to_string(&data)?;
    fs::write(&get_file_path(), json)
}
