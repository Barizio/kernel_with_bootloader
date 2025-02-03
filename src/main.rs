#![no_std]
#![no_main]


mod writer;
use writer::FrameBufferWriter;


use bootloader_api::config::Mapping;
use x86_64::instructions::hlt;
// use print_macro::print;
// use utility_lib::add;

// Use the entry_point macro to register the entry point function:
//bootloader_api::entry_point!(kernel_main)



pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 100 * 1024; //100KiB
    config
};
bootloader_api::entry_point!(my_entry_point, config = &BOOTLOADER_CONFIG);


mod interrupts;
use interrupts{init_idt, PICS};
fn my_entry_point(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    let frame_buffer_info = boot_info.framebuffer.as_mut().unwrap().info();
    let buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();
    let mut frame_buffer_writer = FrameBufferWriter::new(buffer, frame_buffer_info);
   // let mut writer = FrameBufferWriter::new(framebuffer, info);

    use core::fmt::Write; // Required for writeln!

    // Write multiple lines to test scrolling
    for i in 0..150 {
        writeln!(frame_buffer_writer, "This is line number {}", i).unwrap();
        print!(frame_buffer_writer, "This is printed without\t a \\cnewl\\rine for the bread and egg and the and the and the and the and the and the and the and the and the and the and the and the and the and the and ape {}.\n\n\n", i);
        // print!(writer, "Hello, World!\n");
        // print!(writer, "This is a\ttabbed message.\n");
        // print!(writer, "Here is a custom constant: \\c\n");
       
    }

    // Halt the CPU in an infinite loop
    loop {
        x86_64::instructions::hlt();
    }
}


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
loop{
hlt();
}
}
