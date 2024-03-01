mod cpu;
mod memory;
mod disk;
mod network;
mod process;
use cpu::get_cpu_usage;
use memory::get_memory_usage;
use disk::get_disk_usage;
use process::get_process;

fn main() {
    let cpu_usage_table = get_cpu_usage();
    cpu_usage_table.printstd();
    let memory_usage_table = get_memory_usage();
    memory_usage_table.printstd();
    let disk_usage_table = get_disk_usage();
    disk_usage_table.printstd();
    let network_usage_table = network::get_network_usage();
    network_usage_table.printstd();
    let process_table = get_process();
    process_table.printstd();

}