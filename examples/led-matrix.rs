//! 8x8 LED Matrix Display with HT16K33 Driver
//! 
//! This example controls an 8x8 LED matrix using the HT16K33 I2C LED driver chip.
//! It displays a heart shape and a smiley face, alternating every 3 seconds.
//!
//! ## Hardware Connections
//! - **8x8 LED Matrix Module with HT16K33**:
//!   - VCC → 5V on Arduino
//!   - GND → GND on Arduino
//!   - SDA → A4 (SDA) on Arduino Uno
//!   - SCL → A5 (SCL) on Arduino Uno
//!
//! ## I2C Address
//! - Default: 0x70 (can be changed with solder jumpers on some modules)
//!
//! ## Usage
//! Flash to Arduino: `cargo run --example led-matrix`

#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::prelude::*;
use arduino_hal::I2c;

// HT16K33 Register definitions
const HT16K33_ADDR: u8 = 0x70;
const HT16K33_CMD_OSCILLATOR_ON: u8 = 0x21;
const HT16K33_CMD_DISPLAY_ON: u8 = 0x81; // Display ON, no blinking
const HT16K33_CMD_BRIGHTNESS: u8 = 0xE0; // Brightness command (0xE0-0xEF)

// Display buffer address starts at 0x00
const HT16K33_DISPLAY_RAM: u8 = 0x00;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    
    // Initialize I2C
    let mut i2c = I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(), // SDA
        pins.a5.into_pull_up_input(), // SCL
        50000, // 50kHz I2C clock (conservative for reliability)
    );

    // Initialize HT16K33
    // Turn on oscillator
    let _ = i2c.write(HT16K33_ADDR, &[HT16K33_CMD_OSCILLATOR_ON]);
    arduino_hal::delay_ms(10);
    
    // Turn on display (no blinking)
    let _ = i2c.write(HT16K33_ADDR, &[HT16K33_CMD_DISPLAY_ON]);
    arduino_hal::delay_ms(10);
    
    // Set brightness to maximum (0xEF = brightest)
    let _ = i2c.write(HT16K33_ADDR, &[HT16K33_CMD_BRIGHTNESS | 0x0F]);
    arduino_hal::delay_ms(10);

    let mut show_heart = true;

    loop {
        // Define patterns for heart and smiley face
        // Each row is represented as a u8 (we only use 8 bits for 8x8 matrix)
        let row0_val: u8;
        let row1_val: u8;
        let row2_val: u8;
        let row3_val: u8;
        let row4_val: u8;
        let row5_val: u8;
        let row6_val: u8;
        let row7_val: u8;
        
        if show_heart {
            // Heart pattern
            // 0b01100110
            // 0b11111111
            // 0b11111111
            // 0b11111111
            // 0b01111110
            // 0b00111100
            // 0b00011000
            // 0b00000000
            row0_val = 0b01100110;
            row1_val = 0b11111111;
            row2_val = 0b11111111;
            row3_val = 0b11111111;
            row4_val = 0b01111110;
            row5_val = 0b00111100;
            row6_val = 0b00011000;
            row7_val = 0b00000000;
        } else {
            // Smiley face pattern
            // 0b00111100
            // 0b01000010
            // 0b10100101
            // 0b10000001
            // 0b10100101
            // 0b10011001
            // 0b01000010
            // 0b00111100
            row0_val = 0b00111100;
            row1_val = 0b01000010;
            row2_val = 0b10100101;
            row3_val = 0b10000001;
            row4_val = 0b10100101;
            row5_val = 0b10011001;
            row6_val = 0b01000010;
            row7_val = 0b00111100;
        }
        
        // Write display buffer to HT16K33
        // Build and send the complete data buffer
        // We need to send: address, then 16 bytes (8 rows x 2 bytes each)
        // For 8x8 matrix, high byte is 0 for each row
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
        
        // Display for 3 seconds
        arduino_hal::delay_ms(3000);
        
        // Toggle between heart and smiley
        show_heart = !show_heart;
    }
}
