use sysinfo::System;

pub struct MemoryCollector {
    sys: System,
}

impl MemoryCollector {
    pub fn new() -> Self {
        let mut sys = System::new();
        sys.refresh_memory();
        MemoryCollector { sys }
    }

    pub fn collect_total(&mut self) -> u64 {
        self.sys.refresh_memory();
        self.sys.total_memory()
    }

    pub fn collect_used(&mut self) -> u64 {
        self.sys.refresh_memory();
        self.sys.used_memory()
    }
}
