
#![no_std]                              // Unlink from the standard library, ain't nobody got time for that!

use core::panic::PanicInfo;

#[panic_handler]                        // Function to be called on a panic
fn panic(_info: &PanicInfo) -> ! {      // PanicInfo param holds the line of the panic and optional error message
    loop {}                             // For now just loop indef
}

fn main() {
}
