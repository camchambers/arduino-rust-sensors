//! Buzzer Pattern Player
//! 
//! This example plays different musical patterns on a piezo buzzer.
//! Press a button to cycle through patterns: Happy Birthday, Star Wars, and Super Mario.
//!
//! ## Hardware Connections
//! - **Buzzer Module (Active/Passive)**:
//!   - G (GND) → GND on Arduino
//!   - V (VCC) → 5V on Arduino  
//!   - S (Signal) → Pin D8
//! - **Button Module**:
//!   - G (GND) → GND on Arduino
//!   - V (VCC) → Not connected (using internal pull-up)
//!   - S (Signal) → Pin D2
//!
//! ## Usage
//! Flash to Arduino: `cargo run --example buzzer`
//! Press the button to cycle through different musical patterns.

#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Pin 8 (PB0) for Buzzer
    let mut buzzer = pins.d8.into_output();

    // Pin 2 (PD2) for Button - using internal pull-up
    let button = pins.d2.into_pull_up_input();

    // Track current pattern and button state
    let mut current_pattern = 0;
    let mut last_button_state = button.is_high();
    let mut pattern_playing = false;

    loop {
        // Check for button press (pull-up means LOW when pressed)
        let button_state = button.is_high();
        if !button_state && last_button_state {
            // Button was just pressed
            current_pattern = (current_pattern + 1) % 3;
            pattern_playing = false; // Reset to start new pattern
            // Debounce delay
            arduino_hal::delay_ms(300);
        }
        last_button_state = button_state;

        // Play the current pattern once, then wait
        if !pattern_playing {
            match current_pattern {
                0 => pattern_happy_birthday(&mut buzzer),
                1 => pattern_star_wars(&mut buzzer),
                2 => pattern_mario(&mut buzzer),
                _ => {}
            }
            pattern_playing = true;
        }

        // Small delay to check button frequently
        arduino_hal::delay_ms(50);
    }
}

// Pattern 0: Happy Birthday
fn pattern_happy_birthday(
    buzzer: &mut arduino_hal::port::Pin<arduino_hal::port::mode::Output, arduino_hal::hal::port::PB0>
) {
    // Happy birthday to you
    play_tone(buzzer, 262, 250);  // C
    play_tone(buzzer, 262, 250);  // C
    play_tone(buzzer, 294, 500);  // D
    play_tone(buzzer, 262, 500);  // C
    play_tone(buzzer, 349, 500);  // F
    play_tone(buzzer, 330, 1000); // E
    arduino_hal::delay_ms(500);
}

// Pattern 1: Star Wars Imperial March
fn pattern_star_wars(
    buzzer: &mut arduino_hal::port::Pin<arduino_hal::port::mode::Output, arduino_hal::hal::port::PB0>
) {
    play_tone(buzzer, 392, 500);  // G
    play_tone(buzzer, 392, 500);  // G
    play_tone(buzzer, 392, 500);  // G
    play_tone(buzzer, 311, 350);  // Eb
    play_tone(buzzer, 466, 150);  // Bb
    play_tone(buzzer, 392, 500);  // G
    play_tone(buzzer, 311, 350);  // Eb
    play_tone(buzzer, 466, 150);  // Bb
    play_tone(buzzer, 392, 1000); // G
    arduino_hal::delay_ms(500);
}

// Pattern 2: Super Mario Bros
fn pattern_mario(
    buzzer: &mut arduino_hal::port::Pin<arduino_hal::port::mode::Output, arduino_hal::hal::port::PB0>
) {
    play_tone(buzzer, 659, 150);  // E
    play_tone(buzzer, 659, 150);  // E
    arduino_hal::delay_ms(150);
    play_tone(buzzer, 659, 150);  // E
    arduino_hal::delay_ms(150);
    play_tone(buzzer, 523, 150);  // C
    play_tone(buzzer, 659, 150);  // E
    arduino_hal::delay_ms(150);
    play_tone(buzzer, 784, 150);  // G
    arduino_hal::delay_ms(450);
    play_tone(buzzer, 392, 150);  // G (lower)
    arduino_hal::delay_ms(500);
}

// --- The Sound Engine ---
fn play_tone(
    pin: &mut arduino_hal::port::Pin<arduino_hal::port::mode::Output, arduino_hal::hal::port::PB0>, 
    freq_hz: u32, 
    duration_ms: u32
) {
    if freq_hz == 0 {
        // FIX: Removed "as u16" here as well
        arduino_hal::delay_ms(duration_ms);
        return;
    }

    let delay_us = 1_000_000 / freq_hz / 2;
    let cycles = (duration_ms * 1000) / (delay_us * 2);

    for _ in 0..cycles {
        pin.set_high();
        arduino_hal::delay_us(delay_us);
        pin.set_low();
        arduino_hal::delay_us(delay_us);
    }
}

#[no_mangle]
pub extern "C" fn exit(_code: i32) -> ! {
    loop {}
}