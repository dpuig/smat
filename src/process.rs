use sysinfo::System;
use prettytable::{row, Table};


pub fn get_process() -> Table {
    let mut sys = System::new();
    let mut table = Table::new();
    table.add_row(row!["PID", "Process", "CPU %", "Memory %", "Disk Usage"]);

    sys.refresh_all();
    for (pid, process) in sys.processes() {
        let cpu_usage = process.cpu_usage();
        let memory_usage = process.memory() as f64 / (1024.0 * 1024.0);
        let memory_usage = format!("{:.2}", memory_usage);
        let memory_disk_usage = process.disk_usage();
        table.add_row(row![pid, process.name(), cpu_usage, memory_usage, format!("{:?}", memory_disk_usage)]);
    }
    table
}