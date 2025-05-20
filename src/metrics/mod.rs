use sysinfo::System;

pub struct CpuMetrics {
    pub usage: f32,
}

pub fn collect_cpu_metrics(sys: &mut System) -> CpuMetrics {
    sys.refresh_cpu_usage(); // ✅ Refresh CPU usage only
    let cpu_usage = sys.global_cpu_usage(); // ✅ Get global CPU usage
    CpuMetrics {
        usage: cpu_usage,
    }
}



