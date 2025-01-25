#![no_std]
#![no_main]

use bootloader_api::config::{Mapping, BootloaderConfig};
use bootloader_api::{entry_point, BootInfo};
use x86_64::instructions::hlt;

// Define a custom bootloader configuration
pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic); // Optional custom mapping
    config.kernel_stack_size = 100 * 1024; // 100 KiB stack size
    config
};

// Register the entry point function with the custom configuration
entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

// Define the entry point function
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    loop {
        hlt(); // Prevent unnecessary CPU usage in the loop
    }
}

// Define the panic handler
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt(); // Halt on panic
    }
}
