
# Rusty OS

Unlinking from stdlib in Rust throws 2 errors we need to deal with.


## Panic Handler Not Found
The panic handler defines the speciifc function that the compiler should
invoke when there is a panic/error in the program

We can create our own custom panic function to be called so the compiler is happy, for now.

## Language Item Required "eh_personality"
Language Items are special functions that the compiler needs internally.

In this case, the eh_personality language item handles stack unwinding, a process
which calls the destructors of all live stack variables if a panic occurs, allowingthe parent calling function or parent thread to handle the panic, as well as frees upthe memory allocated to stack variables.

We can create a custom language item implementation for this but not recommended.

We will disable unwinding and simply abort on a Panic in Cargo.toml



