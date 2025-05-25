use crate::metrics::snapshot::MetricSnapshot;

pub struct AlertRule {
    pub name: &'static str,
    pub threshold: f64,
    pub check: fn(&MetricSnapshot) -> bool,
}

pub fn default_rules() -> Vec<AlertRule> {
    vec![
        AlertRule {
            name: "High CPU Usage",
            threshold: 5.0,
            check: |snap| snap.cpu_usage > 5.0,
        },
        AlertRule {
            name: "High Memory Usage",
            threshold: 70.0,
            check: |snap| {
                if snap.total_memory == 0 {
                    false
                } else {
                    (snap.used_memory as f64 / snap.total_memory as f64) * 100.0 > 70.0
                }
            },
        },
        // Add more rules if needed
    ]
}
