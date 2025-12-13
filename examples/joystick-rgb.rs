//! Joystick-Controlled RGB LED
//! 
//! This example uses a joystick to control the color of an RGB LED.
//! Moving the joystick in different directions changes the LED color.
//! Center position turns the LED off.
//!
//! ## Color Map
//! - Center: Off (LED off)
//! - Up: Blue
//! - Down: Yellow
//! - Left: Red
//! - Right: Cyan
//! - Up-Left: Magenta
//! - Up-Right: White
//! - Down-Left: Green
//! - Down-Right: Purple
//!
//! ## Hardware Connections
//! - **RGB LED Module** (has V, R, G, B pins):
//!   - V (VCC) → 5V on Arduino (common anode)
//!   - R (Red) → Pin D11
//!   - G (Green) → Pin D12
//!   - B (Blue) → Pin D13
//!
//! - **Joystick Module** (has G, V, X, Y, B pins):
//!   - G (GND) → GND on Arduino
//!   - V (VCC) → 5V on Arduino
//!   - X → A0 (Analog input for X-axis)
//!   - Y → A1 (Analog input for Y-axis)
//!   - B (Button) → Not used in this example
//!
//! ## Usage
//! Flash to Arduino: `cargo run --example joystick-rgb`
//! Move the joystick to see different colors on the RGB LED!

#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::adc;

// Joystick calibration
const CENTER_VALUE: u16 = 512;
const THRESHOLD: u16 = 300; // Larger deadzone around center for "off" state

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    
    // RGB LED pins
    let mut red = pins.d11.into_output();
    let mut green = pins.d12.into_output();
    let mut blue = pins.d13.into_output();
    
    // Initialize ADC for joystick
    let mut adc = adc::Adc::new(dp.ADC, Default::default());
    
    // Set up analog pins for joystick
    let x_axis = pins.a0.into_analog_input(&mut adc);
    let y_axis = pins.a1.into_analog_input(&mut adc);
    
    // Start with LED off
    red.set_low();
    green.set_low();
    blue.set_low();

    loop {
        // Read joystick position
        let x_raw = x_axis.analog_read(&mut adc);
        let y_raw = y_axis.analog_read(&mut adc);
        
        // Determine joystick direction
        // Note: Y-axis is inverted on some modules (Up is high value)
        let x_left = x_raw < CENTER_VALUE - THRESHOLD;
        let x_right = x_raw > CENTER_VALUE + THRESHOLD;
        let y_up = y_raw > CENTER_VALUE + THRESHOLD;   // Swapped for hardware correction
        let y_down = y_raw < CENTER_VALUE - THRESHOLD; // Swapped for hardware correction
        let center = !x_left && !x_right && !y_up && !y_down;
        
        // Set RGB LED color based on joystick position
        // Note: Common Anode LED means LOW is ON, HIGH is OFF
        if center {
            // Center: Off
            red.set_high();
            green.set_high();
            blue.set_high();
        } else if x_left && y_up {
            // Up-Left: Magenta (Red + Blue)
            red.set_low();
            green.set_high();
            blue.set_low();
        } else if x_right && y_up {
            // Up-Right: White (All On)
            red.set_low();
            green.set_low();
            blue.set_low();
        } else if x_left && y_down {
            // Down-Left: Green
            red.set_high();
            green.set_low();
            blue.set_high();
        } else if x_right && y_down {
            // Down-Right: Purple (Blue only)
            red.set_high();
            green.set_high();
            blue.set_low();
        } else if x_left {
            // Left: Red
            red.set_low();
            green.set_high();
            blue.set_high();
        } else if x_right {
            // Right: Cyan (Green + Blue)
            red.set_high();
            green.set_low();
            blue.set_low();
        } else if y_up {
            // Up: Blue
            red.set_high();
            green.set_high();
            blue.set_low();
        } else if y_down {
            // Down: Yellow (Red + Green)
            red.set_low();
            green.set_low();
            blue.set_high();
        }
        
        // Small delay for stability
        arduino_hal::delay_ms(50);
    }
}

// FIX: Satisfies linker requirement for bare-metal exit
#[no_mangle]
pub extern "C" fn exit(_code: i32) -> ! {
    loop {}
}
