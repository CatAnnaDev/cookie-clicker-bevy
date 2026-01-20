pub fn format_number(num: u64) -> String {
    if num < 1_000 {
        num.to_string()
    } else if num < 1_000_000 {
        format!("{:.4}K", num as f64 / 1_000.0)
    } else if num < 1_000_000_000 {
        format!("{:.4}M", num as f64 / 1_000_000.0)
    } else if num < 1_000_000_000_000 {
        format!("{:.4}B", num as f64 / 1_000_000_000.0)
    } else if num < 1_000_000_000_000_000 {
        format!("{:.4}T", num as f64 / 1_000_000_000_000.0)
    } else {
        format!("{:.4}Q", num as f64 / 1_000_000_000_000_000.0)
    }
}

pub fn pseudo_random() -> f32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos % 1000) as f32 / 1000.0
}