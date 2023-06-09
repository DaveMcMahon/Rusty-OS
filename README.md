
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

## Start Lang Item Required
Huh, well whoever said the main function was the entry point of a program wasn't technically correct.

It looks like a runtime system needs to get called beforehand to setup system specific things, and then
invokes the entry point of the Rust runtime denoted by the Start Lang item.

In a binary that links to the stdlib, a C runtime library called crt0 gets initialised and sets up the
environment for a basic C application e.g. Stack Creation.

Finally the Rust runtime entry point is invoked which then calls the main function.

We unliked from the standard library so we don't have access to crt0. A custom implementation of
the Start lang item would still require crt0 to call it, so we can instead overwrite the entry point meaning
we can start the program from a custom entry point without an underlying runtime invoking it.

## [no_mangle] 
This will stop the compiler from giving a unique cryptic name to the custom _start function entry point.
We need to give this function name to the linker as the Linker looks for a function named _start

## Extern C
Interestingly enough, telling the compiler to use the C calling convention as Rust doesn't have one (need to verify this)

The _start function is diverging (never returns) as the entry point will never be called by another function,
rather invoked directly on the OS or bootloader.

## Linker Error
At this point when we run the program we get a linker error. OS's have different executable formats so the
error itself isn't important rather the root cause of the error i.e. expecting the C runtime (crt0).

Rust will attempt to compile for the specific host it's building on and uses the target triple to determine
the set of cpu instructions to use, and always assumes there is an underlying C runtime. We need to compile
to a different target triple so it doesn't expect the C runtime.

## Bare Metal Target
Cross compiling to a target with no underlying OS, and therefore no C runtime expectation, will get rid
of linker errors under normal compilation that uses the hosts target triple.

Successful compilation.

## COMMIT: 905e3f672e04651094cbd972879a396ff5da2270




