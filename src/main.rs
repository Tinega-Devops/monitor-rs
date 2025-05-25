// src/main.rs
use chrono::Local;
use metrics::snapshot::MetricSnapshot;
use metrics::{
    cpu::CpuCollector, disk::DiskCollector, memory::MemoryCollector, network::NetworkCollector,
};
use std::sync::mpsc::channel;
use std::time::Duration;

mod metrics;

fn main() {
    println!("Starting monitor-rs...");

    let mut cpu = CpuCollector::new();
    let mut mem = MemoryCollector::new();
    let mut disk = DiskCollector::new();
    let mut net = NetworkCollector::new();

    let (ui_tx, _ui_rx) = channel();
    let (alert_tx, _alert_rx) = channel();

    loop {
        let snapshot = MetricSnapshot {
            timestamp: Local::now(),
            cpu_usage: cpu.collect(),
            total_memory: mem.collect_total(),
            used_memory: mem.collect_used(),
            disk_read: disk.collect_read(),
            disk_write: disk.collect_write(),
            net_rx: net.collect_rx(),
            net_tx: net.collect_tx(),
        };

        println!("{:?}", snapshot); // For now, print to console

        ui_tx.send(snapshot.clone()).unwrap();
        alert_tx.send(snapshot).unwrap();

        std::thread::sleep(Duration::from_millis(1000));
    }
}
