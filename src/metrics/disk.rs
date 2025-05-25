// src/metrics/disk.rs
use sysinfo::System;

pub struct DiskCollector {
    sys: System,
    prev_read: u64,
    prev_write: u64,
}

impl DiskCollector {
    pub fn new() -> Self {
        let mut sys = System::new_all(); // ensures process list is populated
        sys.refresh_all();

        let (read, write) = Self::aggregate(&sys);
        DiskCollector {
            sys,
            prev_read: read,
            prev_write: write,
        }
    }

    fn aggregate(sys: &System) -> (u64, u64) {
        let mut read = 0;
        let mut write = 0;
        for process in sys.processes().values() {
            let usage = process.disk_usage();
            read += usage.total_read_bytes;
            write += usage.total_written_bytes;
        }
        (read, write)
    }

    pub fn collect_read(&mut self) -> u64 {
        self.sys.refresh_all();
        let (read, _) = Self::aggregate(&self.sys);
        let delta = read.saturating_sub(self.prev_read);
        self.prev_read = read;
        delta
    }

    pub fn collect_write(&mut self) -> u64 {
        self.sys.processes();
        let (_, write) = Self::aggregate(&self.sys);
        let delta = write.saturating_sub(self.prev_write);
        self.prev_write = write;
        delta
    }
}
