use sysinfo::{System, Networks};
use prettytable::{row, Table};

pub fn get_network_usage() -> Table {

    let mut sys = System::new();
    let mut table = Table::new();
    table.add_row(row!["Interface", "Data received B", "Data transmitted B"]);

    sys.refresh_all();
    let networks = Networks::new_with_refreshed_list();
    for (interface_name, data) in &networks {
        let data_received = data.received() as f64 / (1024.0 * 1024.0);
        let data_received = format!("{:.2}", data_received);
        let data_transmitted = data.transmitted() as f64 / (1024.0 * 1024.0);
        let data_transmitted = format!("{:.2}", data_transmitted);
        table.add_row(row![interface_name, data_received, data_transmitted]);
    }
    table
}