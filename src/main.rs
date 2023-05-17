#![no_std]                              // Unlink from the standard library, ain't nobody got time for that!
#![no_main]                             // Overriding the usual entry point of the Rust runtime

use core::panic::PanicInfo;

#[panic_handler]                        // Function to be called on a panic
fn panic(_info: &PanicInfo) -> ! {      // PanicInfo param holds the line of the panic and optional error message
    loop {}                             // For now just loop indef
}

#[no_mangle]                            // Remove the main method as it doesn't make sense to have 
pub extern "C" fn _start() -> ! {       // one without the underlying runtime to call it.
    loop {}                             // We are overriding the OS entry point with our own.
}
