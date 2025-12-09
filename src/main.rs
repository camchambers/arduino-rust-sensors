#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    loop {
        // This main.rs serves as a placeholder.
        // Use: cargo run --example <name>
        // Available examples: traffic-light, beeper, etc.
    }
}

// FIX: Satisfies linker requirement for bare-metal exit
#[no_mangle]
pub extern "C" fn exit(_code: i32) -> ! {
    loop {}
}
