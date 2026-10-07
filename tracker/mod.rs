pub mod model;
pub mod store;

use model::MistakeLog;

pub fn record_mistake(attempted: String, suggested: Option<String>) -> u32 {
    let mut log = store::load();
    log.record(attempted, suggested);
    let count = log.total_count;

    store::save(&log).ok();
    count
}

pub fn get_count() -> u32 {
    store::load().total_count
}
