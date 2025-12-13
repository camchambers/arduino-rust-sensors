//! Joystick Controlled Dot on 8x8 LED Matrix
//! 
//! This example uses a joystick to control a dot on an 8x8 LED matrix.
//! Move the joystick to move the dot around the display.
//! Press the button to toggle between drawing mode (leaves a trail) and normal mode (single dot).
//!
//! ## Hardware Connections
//! - **8x8 LED Matrix Module with HT16K33**:
//!   - VCC → 5V on Arduino
//!   - GND → GND on Arduino
//!   - SDA → A4 (SDA) on Arduino Uno
//!   - SCL → A5 (SCL) on Arduino Uno
//!
//! - **Joystick Module**:
//!   - G (GND) → GND on Arduino
//!   - V (VCC) → 5V on Arduino
//!   - X → A0 (Analog input for X-axis)
//!   - Y → A1 (Analog input for Y-axis)
//!   - B (Button) → D2
//!
//! ## Usage
//! Flash to Arduino: `cargo run --example joystick-dot`
//! Move the joystick to control the dot position on the LED matrix.
//! Press the button to toggle drawing mode - when enabled, the dot leaves a trail!

#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::prelude::*;
use arduino_hal::I2c;
use arduino_hal::adc;

// HT16K33 Register definitions
const HT16K33_ADDR: u8 = 0x70;
const HT16K33_CMD_OSCILLATOR_ON: u8 = 0x21;
const HT16K33_CMD_DISPLAY_ON: u8 = 0x81;
const HT16K33_CMD_BRIGHTNESS: u8 = 0xE0;
const HT16K33_DISPLAY_RAM: u8 = 0x00;

// Joystick calibration
// Typical joystick values: center ~512, min ~0, max ~1023
const CENTER_THRESHOLD: u16 = 300; // Larger deadzone
const CENTER_VALUE: u16 = 512;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    
    // Initialize I2C for LED matrix
    let mut i2c = I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(), // SDA
        pins.a5.into_pull_up_input(), // SCL
        50000,
    );

    // Initialize ADC for joystick
    let mut adc = adc::Adc::new(dp.ADC, Default::default());
    
    // Set up analog pins for joystick
    let x_axis = pins.a0.into_analog_input(&mut adc);
    let y_axis = pins.a1.into_analog_input(&mut adc);
    
    // Set up button pin
    let button = pins.d2.into_pull_up_input();

    // Initialize HT16K33
    let _ = i2c.write(HT16K33_ADDR, &[HT16K33_CMD_OSCILLATOR_ON]);
    arduino_hal::delay_ms(10);
    
    let _ = i2c.write(HT16K33_ADDR, &[HT16K33_CMD_DISPLAY_ON]);
    arduino_hal::delay_ms(10);
    
    let _ = i2c.write(HT16K33_ADDR, &[HT16K33_CMD_BRIGHTNESS | 0x0F]);
    arduino_hal::delay_ms(10);

    // Initial dot position (center of 8x8 matrix)
    let mut dot_x: u8 = 4;
    let mut dot_y: u8 = 4;
    
    // Drawing mode state
    let mut drawing_mode = false;
    let mut canvas: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0]; // Persistent canvas for drawing
    
    // Button debouncing
    let mut last_button_state = true; // Pull-up means HIGH when not pressed
    
    // Movement timing
    let mut delay_counter: u8 = 0;
    const MOVE_DELAY: u8 = 10; // Movement speed control

    loop {
        // Read joystick position
        let x_raw = x_axis.analog_read(&mut adc);
        let y_raw = y_axis.analog_read(&mut adc);
        
        // Check button for mode toggle (LOW when pressed due to pull-up)
        let button_state = button.is_high();
        if !button_state && last_button_state {
            // Button just pressed - toggle drawing mode
            drawing_mode = !drawing_mode;
            
            // If entering drawing mode, clear the canvas
            if drawing_mode {
                canvas = [0, 0, 0, 0, 0, 0, 0, 0];
            }
            
            // Debounce delay
            arduino_hal::delay_ms(200);
        }
        last_button_state = button_state;
        
        // Update position based on joystick (only every MOVE_DELAY iterations)
        if delay_counter >= MOVE_DELAY {
            delay_counter = 0;
            
            // X-axis movement (left/right)
            // Lower values = left, higher values = right
            if x_raw < CENTER_VALUE - CENTER_THRESHOLD {
                // Move left
                if dot_x > 0 {
                    dot_x -= 1;
                }
            } else if x_raw > CENTER_VALUE + CENTER_THRESHOLD {
                // Move right
                if dot_x < 7 {
                    dot_x += 1;
                }
            }
            
            // Y-axis movement (up/down)
            // Lower values = up, higher values = down
            if y_raw < CENTER_VALUE - CENTER_THRESHOLD {
                // Move up (decrease Y)
                if dot_y > 0 {
                    dot_y -= 1;
                }
            } else if y_raw > CENTER_VALUE + CENTER_THRESHOLD {
                // Move down (increase Y)
                if dot_y < 7 {
                    dot_y += 1;
                }
            }
        }
        delay_counter += 1;
        
        // In drawing mode, add current position to canvas
        if drawing_mode {
            canvas[dot_y as usize] |= 1u8 << dot_x;
        }
        
        // Create display pattern
        // In drawing mode: show canvas with current dot
        // In normal mode: show only current dot
        let row0_val: u8 = if drawing_mode { canvas[0] | if dot_y == 0 { 1u8 << dot_x } else { 0 } } else { if dot_y == 0 { 1u8 << dot_x } else { 0 } };
        let row1_val: u8 = if drawing_mode { canvas[1] | if dot_y == 1 { 1u8 << dot_x } else { 0 } } else { if dot_y == 1 { 1u8 << dot_x } else { 0 } };
        let row2_val: u8 = if drawing_mode { canvas[2] | if dot_y == 2 { 1u8 << dot_x } else { 0 } } else { if dot_y == 2 { 1u8 << dot_x } else { 0 } };
        let row3_val: u8 = if drawing_mode { canvas[3] | if dot_y == 3 { 1u8 << dot_x } else { 0 } } else { if dot_y == 3 { 1u8 << dot_x } else { 0 } };
        let row4_val: u8 = if drawing_mode { canvas[4] | if dot_y == 4 { 1u8 << dot_x } else { 0 } } else { if dot_y == 4 { 1u8 << dot_x } else { 0 } };
        let row5_val: u8 = if drawing_mode { canvas[5] | if dot_y == 5 { 1u8 << dot_x } else { 0 } } else { if dot_y == 5 { 1u8 << dot_x } else { 0 } };
        let row6_val: u8 = if drawing_mode { canvas[6] | if dot_y == 6 { 1u8 << dot_x } else { 0 } } else { if dot_y == 6 { 1u8 << dot_x } else { 0 } };
        let row7_val: u8 = if drawing_mode { canvas[7] | if dot_y == 7 { 1u8 << dot_x } else { 0 } } else { if dot_y == 7 { 1u8 << dot_x } else { 0 } };
        
        // Write display buffer to HT16K33
        let write_buf: [u8; 17] = [
            HT16K33_DISPLAY_RAM,
            row0_val, 0,
            row1_val, 0,
            row2_val, 0,
            row3_val, 0,
            row4_val, 0,
            row5_val, 0,
            row6_val, 0,
            row7_val, 0,
        ];
        
        let _ = i2c.write(HT16K33_ADDR, &write_buf);
        
        // Small delay for smooth updates
        arduino_hal::delay_ms(10);
    }
}
