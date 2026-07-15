use std::fs;
use std::thread::available_parallelism;

use colored::*;

fn get_system_cpu_count() -> usize {
    let core_count = available_parallelism().map(|n| n.get()).unwrap_or(1);

    if let Ok(content) = fs::read_to_string("/sys/fs/cgroup/cpu.max") {
        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.len() == 2 {
            if parts[0] != "max" {
                if let (Ok(max), Ok(period)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                    let limit = (max / period).ceil() as usize;
                    return limit.max(1);
                }
            }
        }
    }

    core_count
}

pub fn get_calculated_worker_count() -> usize {
     get_system_cpu_count() * 2
}


pub fn print_init_art() {
    let banner = include_str!("../../assets/hermes_banner.txt");
    println!("{}", banner.bright_green());

}