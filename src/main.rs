mod metrics;
mod ui;
mod alerting;
mod config;

use sysinfo::System;

fn main() {
    println!("Starting monitor-rs...");

    let mut sys = System::new();

    let cpu = metrics::collect_cpu_metrics(&mut sys);
    println!("CPU Usage: {:.2}%", cpu.usage);
}
