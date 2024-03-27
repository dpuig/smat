use system_info::SystemInfo;
use comfy_table::*;
use asciifyer::{convert_to_ascii, Dimension};
use reqwest::blocking;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::system_info;

pub fn display_info_simple(system_info: &SystemInfo) {
    // Print the information
    println!("Hostname: {}", system_info.hostname);
    println!("-------------------");
    println!("OS Name: {}", system_info.os_name);
    println!("OS Version: {}", system_info.os_version);
    println!("Kernel Version: {}", system_info.kernel_version);
    println!("CPU Model: {}", system_info.cpu_model);
    println!("Total RAM: {}", system_info.ram_total);
    println!("Used RAM: {}", system_info.ram_used);
    println!("Uptime: {}", system_info.uptime);

}


pub fn display_info_tabular(system_info: &SystemInfo) {
    let mut table = Table::new();
    table.set_header(vec!["Attribute", "Value"]);
    table.add_row(vec!["OS", &system_info.os_name]);
    table.add_row(vec!["OS Version", &system_info.os_version]);
    table.add_row(vec!["Kernel", &system_info.kernel_version]);
    table.add_row(vec!["Hostname", &system_info.hostname]);
    table.add_row(vec!["CPU Model", &system_info.cpu_model]);
    table.add_row(vec!["Total RAM", &system_info.ram_total.to_string()]);
    table.add_row(vec!["Used RAM", &system_info.ram_used.to_string()]);
    table.add_row(vec!["Uptime", &system_info.uptime]);
    
    println!("{}", table);
}

pub fn display_disk_info(system_info: &SystemInfo) {
    for disk in &system_info.disks {
        println!("Disk Name: {}", disk.name);
        println!("File System: {}", disk.file_system);
        println!("Total Space: {}", disk.total_space);
        println!("Available Space: {}", disk.available_space);
    }
}

pub fn display_logo(name: &str){
    let image_url = "https://www.cultofmac.com/wp-content/uploads/2019/07/logo.jpg"; // Replace with your image URL
    let dimensions = Dimension::new(80, 40); // Adjust the output size as desired

    let response = blocking::get(image_url).unwrap();
    let image_bytes = response.bytes().unwrap();

    let temp_file_path = Path::new("temp_logo.jpg"); // Or choose a suitable name
    let mut file = File::create(temp_file_path).unwrap();
    file.write_all(&image_bytes).unwrap();
    
    let ascii = convert_to_ascii(temp_file_path, Some(dimensions)); // Pass the path
    println!("{}", ascii);
}