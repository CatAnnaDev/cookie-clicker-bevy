pub fn format_number(num: u128) -> String {
    const UNITS: [&str; 15] = [
        "", "K", "M", "B", "T",
        "Qa", "Qi", "Sx", "Sp", "Oc",
        "No", "Dc", "Ud", "Dd", "Td"
    ];

    if num < 1_000 {
        return num.to_string();
    }

    let mut unit = 0;
    let mut value = num as f64;

    while value >= 1_000.0 && unit < UNITS.len() - 1 {
        value /= 1_000.0;
        unit += 1;
    }

    let decimals = if value >= 100.0 {
        1
    } else if value >= 10.0 {
        2
    } else {
        3
    };

    format!("{:.*}{}", decimals, value, UNITS[unit])
}


pub fn random_spawn_time() -> f32 {
    30.0 + pseudo_random() * 390.0
}

pub fn pseudo_random() -> f32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    ((nanos as f32 * 0.000000001) * 997.0) % 1.0
}