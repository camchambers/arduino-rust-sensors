//! Traffic Light Controller
//! 
//! This example simulates a traffic light using the KEYESTUDIO Traffic Light module.
//! Press a button to cycle through different patterns: standard traffic sequence, 
//! flashing yellow, and police lights.
//!
//! ## Hardware Connections
//! - **Traffic Light Module** (has R, Y, G, GND pins):
//!   - GND → GND on Arduino
//!   - R (Red) → Pin D11
//!   - Y (Yellow) → Pin D12
//!   - G (Green) → Pin D13
//! - **Button Module**:
//!   - G (GND) → GND on Arduino
//!   - V (VCC) → Not connected (using internal pull-up)
//!   - S (Signal) → Pin D2
//!
//! ## Usage
//! Flash to Arduino: `cargo run --example traffic-light`
//! Press the button to cycle through different light patterns.

#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::prelude::*; // Import traits for delays

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // MAPPING: Green=13, Yellow=12, Red=11, Button=2
    let mut green = pins.d13.into_output();
    let mut yellow = pins.d12.into_output();
    let mut red = pins.d11.into_output();
    let button = pins.d2.into_pull_up_input(); // Use pull-up like buzzer.rs

    let mut current_pattern: u8 = 0;
    let mut last_button_state = button.is_high();

    loop {
        // Check for button press (pull-up means LOW when pressed)
        let button_state = button.is_high();
        if !button_state && last_button_state {
            // Button was just pressed - cycle to next pattern
            current_pattern = (current_pattern + 1) % 3;
            
            // Turn off all LEDs when switching patterns
            red.set_low();
            yellow.set_low();
            green.set_low();
            
            // Debounce delay
            arduino_hal::delay_ms(300);
        }
        last_button_state = button_state;

        // Execute the current pattern
        match current_pattern {
            0 => {
                // --- PATTERN 1: The "Knight Rider" Bounce (Chase) ---
                // Red -> Yellow -> Green -> Yellow -> Red
                
                // Down
                red.set_high(); arduino_hal::delay_ms(100); red.set_low();
                yellow.set_high(); arduino_hal::delay_ms(100); yellow.set_low();
                green.set_high(); arduino_hal::delay_ms(100); green.set_low();
                
                // Up
                yellow.set_high(); arduino_hal::delay_ms(100); yellow.set_low();
            }
            1 => {
                // --- PATTERN 2: Police Strobe (Fast Flash) ---
                // Flash Red fast 3 times, then Green fast 3 times
                for _ in 0..3 {
                    red.set_high(); arduino_hal::delay_ms(40); red.set_low(); arduino_hal::delay_ms(40);
                }
                for _ in 0..3 {
                    green.set_high(); arduino_hal::delay_ms(40); green.set_low(); arduino_hal::delay_ms(40);
                }
            }
            2 => {
                // --- PATTERN 3: "Warning" (All ON) ---
                red.set_high(); yellow.set_high(); green.set_high();
                arduino_hal::delay_ms(200);
                red.set_low(); yellow.set_low(); green.set_low();
                arduino_hal::delay_ms(200);
            }
            _ => {}
        }
    }
}

// The "Linker Shim" (Required for bare-metal Rust)
#[no_mangle]
pub extern "C" fn exit(_code: i32) -> ! {
    loop {}
}