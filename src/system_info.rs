use sysinfo::{
    Disks, System,
};

// Define a struct to hold all relevant system information
pub struct SystemInfo {
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub hostname: String,
    pub cpu_model: String,
    pub ram_total: u64,
    pub ram_used: u64,
    pub uptime: String,
    pub disks: Vec<DiskInfo>, 
}

// Struct to store information about individual disks
pub struct DiskInfo {
    pub name: String,
    pub file_system: String,
    pub total_space: u64,
    pub available_space: u64,
}

// Our main function to collect all the data
fn get_system_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Collect the information
    SystemInfo {
        os_name: sysinfo::System::name().unwrap_or_else(|| "Unknown".to_string()),
        os_version: sysinfo::System::os_version().unwrap_or_else(|| "Unknown".to_string()),
        kernel_version: sysinfo::System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
        hostname: sysinfo::System::host_name().unwrap_or_else(|| "Unknown".to_string()),
        cpu_model: sys.global_cpu_info().brand().to_string(),
        ram_total: sys.total_memory(),
        ram_used: sys.used_memory(),
        uptime: format!("{} days", sysinfo::System::uptime() / 86400), // Approximate uptime
        disks: get_disk_info(&sys),
    }
}

// Helper function to get disk information
fn get_disk_info(_sys: &System) -> Vec<DiskInfo> {
    let disks = Disks::new_with_refreshed_list();
    disks
        .iter()
        .map(|disk| DiskInfo {
            name: disk.mount_point().to_str().unwrap_or("Unknown").to_string(),
            file_system: disk.file_system().to_str().unwrap_or("Unknown").to_string(),
            total_space: disk.total_space(),
            available_space: disk.available_space(),
        })
        .collect()
}

// Export the function so it's usable in main.rs
pub fn get_system_info_struct() -> SystemInfo {
    get_system_info()
}
