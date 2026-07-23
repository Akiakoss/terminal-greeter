use colored::*;
use users::{get_user_by_uid, get_current_uid};
use systemstat::{System, Platform};

fn main() {
    let sys = System::new();

    let user = get_user_by_uid(get_current_uid()).unwrap();
    println!("Hello, {}!", user.name().to_string_lossy().cyan());

    match sys.uptime() {
        Ok(uptime) => {
            let total_secs = uptime.as_secs();
            let hours = total_secs / 3600;
            let minutes = (total_secs % 3600) / 60;
            let seconds = total_secs % 60;

            let mut uptime_peices = Vec::new();
            if hours > 0 {
                uptime_peices.push(pluralize(hours, "hour"));
            }
            if minutes > 0 {
                uptime_peices.push(pluralize(minutes, "minute"));
            }
            if seconds > 0 || uptime_peices.is_empty() {
                uptime_peices.push(pluralize(seconds, "second"));
            }

            let uptime_str = match uptime_peices.len() {
                1 => uptime_peices[0].clone(),
                2 => format!("{} and {}", uptime_peices[0], uptime_peices[1]),
                _ => format!("{}, {} and {}", uptime_peices[0], uptime_peices[1], uptime_peices[2]),
            };

            println!("The system has been running for {}.", uptime_str.cyan());
        },
        Err(x) => println!("Uptime: error: {}", x)
    }
}

fn pluralize(n: u64, unit: &str) -> String {
    if n == 1 {
        format!("{} {}", n, unit)
    } else {
        format!("{} {}s", n, unit)
    }
}