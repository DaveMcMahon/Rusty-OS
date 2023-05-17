/*
*   Unlinking from stdlib in Rust throws 2 errors we need to deal with.
*   
*   Error 1: "#[panic_handler]` function required, but not found"
*       - The panic handler defines the speciifc function that the compiler should
*           invoke when there is a panic/error in the program
*       - We can create our own custom panic function to be called so the compiler is happy, for
*           now.
*   Error 2: "language item required, but not found: `eh_personality`"
*       - Language Items are special functions that the compiler needs internally.
*           In this case, the eh_personality language item handles stack unwinding, a process
*           which calls the destructors of all live stack variables if a panic occurs, allowing
*           the parent calling function or parent thread to handle the panic, as well as frees up
*           the memory allocated to stack variables.
*       - We can create a custom language item implementation for this but not recommended. 
*           We will disable unwinding and simply abort on a Panic in Cargo.toml
*
*/
#![no_std]                              // Unlink from the standard library, ain't nobody got time for that!


use core::panic::PanicInfo;

#[panic_handler]                        // Function to be called on a panic
fn panic(_info: &PanicInfo) -> ! {      // PanicInfo param holds the line of the panic and optional error message
    loop {}                             // For now just loop indef
}

fn main() {
}
