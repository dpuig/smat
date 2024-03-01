use sysinfo::System;
use prettytable::{row, Table};

pub fn get_memory_usage() -> Table {
    let mut sys = System::new();
    let mut table = Table::new();
    table.add_row(row!["Memory MB", "Usage %"]);

    sys.refresh_all(); // Refreshing all system information.
    let memory = sys.used_memory();
    let total_memory = sys.total_memory();
    let memory_usage = format!("{:.2}%", (memory as f64 / total_memory as f64) * 100.0);
    let total_memory_in_gb = total_memory as f64 / (1024.0 * 1024.0);
    let total_memory_in_gb = format!("{:.2}", total_memory_in_gb);
    table.add_row(row![total_memory_in_gb, memory_usage]);

    table
}
