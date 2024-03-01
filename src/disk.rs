use sysinfo::{System, Disks};
use prettytable::{row, Table};

pub fn get_disk_usage() -> Table {
    let mut sys = System::new();
    let mut table = Table::new();
    table.add_row(row!["Disk", "Total Space GB", "Available Space GB"]);

    sys.refresh_all();
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        let total_space = disk.total_space() as f64 / (1024.0 * 1024.0 * 1024.0);
        let total_space = format!("{:.2}", total_space);
        let available_space = disk.available_space() as f64 / (1024.0 * 1024.0 * 1024.0);
        let available_space = format!("{:.2}", available_space);
        table.add_row(row![disk.name().to_string_lossy(), total_space, available_space]);
    }
    table
}