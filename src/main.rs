mod display;
mod system_info;


fn main() {
    display::display_logo();
    let system_info = system_info::get_system_info_struct();
    // Choose your display method
    display::display_info_simple(&system_info); 
    // OR
    display::display_info_tabular(&system_info);
    display::display_disk_info(&system_info);
}