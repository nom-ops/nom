use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MistakeEntry {
    pub timestamp: DateTime<Utc>,
    pub attempted: String,
    pub suggested: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MistakeLog {
    pub total_count: u32,
    pub entries: Vec<MistakeEntry>,
}

impl MistakeLog {
    pub fn record(&mut self, attempted: String, suggested: Option<String>) {
        self.total_count += 1;
        self.entries.push(MistakeEntry {
            timestamp: Utc::now(),
            attempted,
            suggested,
        });
    }
}
