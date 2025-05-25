
# monitor-rs

  

`monitor-rs` is a terminal-based system resource monitoring tool written in Rust. It provides real-time tracking of CPU, memory, disk, and network usage using a rich TUI (Text User Interface) and includes an alerting system based on customizable thresholds.

---
## 📊 Features


-  **Live TUI Dashboard**: Visualizes system resource usage in a terminal using `ratatui`.

-  **Metrics Collection**: Monitors CPU, memory, disk I/O, and network throughput.

-  **Alerting System**: Detects threshold breaches and logs alerts to a file.

-  **Modular Architecture**: Easy to extend or modify collectors, UI widgets, and alert rules.

-  **Cross-platform**: Works on Linux and macOS (Windows support coming soon).

---

## 📦 Installation


### Prerequisites


- [Rust](https://www.rust-lang.org/tools/install) (1.70+)

- Cargo package manager (comes with Rust)

### Clone and Build

```bash

git  clone  https://github.com/Tinega-Devops/monitor-rs.git

cd  monitor-rs

cargo  build  --release

````

---

## 🚀 Usage

```bash

cargo run

```
  
* Press `q` to quit the dashboard.

---

## 🧱 Project Structure

```plaintext
monitor-rs/
├── src/
│   ├── main.rs              # App entry point
│   ├── metrics/             # System collectors and data snapshot
│   │   ├── cpu.rs
│   │   ├── disk.rs
│   │   ├── memory.rs
│   │   ├── network.rs
│   │   └── snapshot.rs
│   ├── alerting/            # Alerting rules and evaluation logic
│   │   ├── handler.rs
│   │   └── rules.rs
│   └── ui/                  # Terminal UI components
│       ├── dashboard.rs
│       ├── cpu_widget.rs
│       ├── memory_widget.rs
│       ├── disk_widget.rs
│       ├── net_widget.rs
│       └── theme.rs
├── Cargo.toml               # Dependencies and package config
└── alerts.log               # File where triggered alerts are logged

```
---

## 🔔 Alerting System

* Alerts are based on rules defined in `alerting/rules.rs`.

* If a metric exceeds a defined threshold, an alert is logged in `alerts.log`.

* Example:

```

[ALERT] High CPU Usage triggered at 2025-05-25 16:30:22. Threshold: 85.0

```

To change thresholds or add new rules, edit the `default_rules()` function.

---

## 📄 License


This project is licensed under the MIT License. See `LICENSE` for details.

---
## 🤝 Contributing

1. Fork the repo

2. Create a new branch (`git checkout -b feature-name`)

3. Make your changes

4. Commit (`git commit -m "Add feature"`)

5. Push (`git push origin feature-name`)

6. Open a Pull Request 🚀

---

## 📬 Contact

Created and maintained by [Tinega Onchari](https://github.com/Tinega-Devops)