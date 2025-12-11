//! RGB LED Color Cycler
//! 
//! This example cycles through different colors on an RGB LED module.
//! Press a button to change between colors: Red, Green, Blue, Yellow, Cyan, Magenta, White, Rainbow Fade.
//!
//! ## Hardware Connections
//! - **RGB LED Module** (has V, R, G, B pins):
//!   - V (VCC) → 5V on Arduino (common anode) or GND (common cathode - check your module)
//!   - G (Green) → Pin D13
//!   - R (Red) → Pin D12
//!   - B (Blue) → Pin D11
//! - **Button Module**:
//!   - G (GND) → GND on Arduino
//!   - V (VCC) → Not connected (using internal pull-up)
//!   - S (Signal) → Pin D2
//!
//! ## Usage
//! Flash to Arduino: `cargo run --example rgb-led`
//! Press the button to cycle through different colors.

#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // RGB LED pins
    let mut green = pins.d13.into_output();
    let mut red = pins.d12.into_output();
    let mut blue = pins.d11.into_output();
    
    // Button pin with pull-up resistor
    let button = pins.d2.into_pull_up_input();

    // Track current color and button state
    let mut current_color = 0;
    let mut last_button_state = button.is_high();

    loop {
        // Check for button press (pull-up means LOW when pressed)
        let button_state = button.is_high();
        if !button_state && last_button_state {
            // Button was just pressed - cycle to next color
            current_color = (current_color + 1) % 8;
            
            // Debounce delay
            arduino_hal::delay_ms(300);
        }
        last_button_state = button_state;

        // Set the current color
        match current_color {
            0 => {
                // Rainbow fade - cycle through colors smoothly
                // Blue -> Cyan -> Green -> Yellow -> Red -> Magenta -> Blue
                for _ in 0..10 {
                    // Blue to Cyan (fade in green)
                    red.set_low(); green.set_high(); blue.set_high();
                    arduino_hal::delay_ms(400);
                    
                    // Check for button press
                    if !button.is_high() {
                        break;
                    }
                    
                    // Cyan to Green (fade out blue)
                    red.set_low(); green.set_high(); blue.set_low();
                    arduino_hal::delay_ms(400);
                    
                    // Check for button press
                    if !button.is_high() {
                        break;
                    }
                    
                    // Green to Yellow (fade in red)
                    red.set_high(); green.set_high(); blue.set_low();
                    arduino_hal::delay_ms(400);
                    
                    // Check for button press
                    if !button.is_high() {
                        break;
                    }
                    
                    // Yellow to Red (fade out green)
                    red.set_high(); green.set_low(); blue.set_low();
                    arduino_hal::delay_ms(400);
                    
                    // Check for button press
                    if !button.is_high() {
                        break;
                    }
                    
                    // Red to Magenta (fade in blue)
                    red.set_high(); green.set_low(); blue.set_high();
                    arduino_hal::delay_ms(400);
                    
                    // Check for button press
                    if !button.is_high() {
                        break;
                    }
                    
                    // Magenta to Blue (fade out red)
                    red.set_low(); green.set_low(); blue.set_high();
                    arduino_hal::delay_ms(400);
                    
                    // Check for button press
                    if !button.is_high() {
                        break;
                    }
                }
            }
            1 => {
                // Red
                red.set_high();
                green.set_low();
                blue.set_low();
            }
            2 => {
                // Green
                red.set_low();
                green.set_high();
                blue.set_low();
            }
            3 => {
                // Blue
                red.set_low();
                green.set_low();
                blue.set_high();
            }
            4 => {
                // Yellow (Red + Green)
                red.set_high();
                green.set_high();
                blue.set_low();
            }
            5 => {
                // Cyan (Green + Blue)
                red.set_low();
                green.set_high();
                blue.set_high();
            }
            6 => {
                // Magenta (Red + Blue)
                red.set_high();
                green.set_low();
                blue.set_high();
            }
            7 => {
                // White (All on)
                red.set_high();
                green.set_high();
                blue.set_high();
            }
            _ => {}
        }

        // Small delay to check button frequently
        arduino_hal::delay_ms(50);
    }
}

// FIX: Satisfies linker requirement for bare-metal exit
#[no_mangle]
pub extern "C" fn exit(_code: i32) -> ! {
    loop {}
}
