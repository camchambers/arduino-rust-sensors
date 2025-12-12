fn main() {
    // For AVR targets, we need to link against avr-libc for certain symbols
    // The 'exit' symbol is required by the AVR startup code
    let target = std::env::var("TARGET").unwrap_or_default();
    
    if target.starts_with("avr") {
        // Tell cargo to pass -lgcc to the linker to get the exit symbol
        println!("cargo:rustc-link-lib=gcc");
    }
}
