use sysinfo::System;
use prettytable::{row, Table};


pub fn get_cpu_usage() -> Table {
    let mut sys = System::new();
    let mut table = Table::new();
    table.add_row(row!["CPU Name", "Usage %"]);

    sys.refresh_all(); // Refreshing all system information.
    for cpu in sys.cpus() {
        let cpu_name = format!("CPU {}", cpu.name());
        let cpu_usage = format!("{:.2}%", cpu.cpu_usage());
        table.add_row(row![cpu_name, cpu_usage]);
    }
    
    table
}
