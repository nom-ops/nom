use std::fs;
use std::path::PathBuf;
use super::model::MistakeLog;

fn get_data_path() -> PathBuf {
    let data_dir = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from(".")).join("nom");

    fs::create_dir_all(&data_dir).ok();
    data_dir.join("mistakes.json")
}

pub fn load() -> MistakeLog {
    let path = get_data_path();
    if !path.exists() {
        return MistakeLog::default();
    }
    
    let content = fs::read_to_string(&path).unwrap_or_default();
    serde_json::from_str(&content).unwrap_or_default()
}

pub fn save(log: &MistakeLog) -> std::io::Result<()> {
    let path = get_data_path();
    let json = serde_json::to_string_pretty(log)?;

    fs::write(path, json)
}
