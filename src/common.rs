use std::time::{SystemTime, UNIX_EPOCH};

pub fn now() -> u64 {
    let now = SystemTime::now();
    return now.duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
}