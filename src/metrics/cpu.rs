use sysinfo::System;
pub struct CpuCollector {
    sys: System,
}

impl CpuCollector {
    pub fn new() -> Self {
        let mut sys = System::new();
        sys.refresh_cpu_all();
        CpuCollector { sys }
    }

    pub fn collect(&mut self) -> f32 {
        self.sys.refresh_cpu_all();
        self.sys.global_cpu_usage()
    }
}
