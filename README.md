Arduino Rust Sensors
======================

Rust project for the _Arduino Uno_ using components from the **KEYESTUDIO 48 Sensors Modules Starter Kit for Arduino**.

## Project Structure
This project uses Cargo's `examples/` directory to organize multiple programs:
- Each file in `examples/` is a separate Arduino program
- Run any example with: `cargo run --example <name>`

## Build Instructions
1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Build an example: `cargo build --example <name>`

3. Flash an example to your board: `cargo run --example <name>`
   
   If `ravedude` fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.

4. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.

## Available Examples
- `buzzer.rs` - Play musical patterns on a buzzer (button cycles through songs)
- `traffic-light.rs` - Animated LED patterns with red, yellow, and green lights
- `rgb-led.rs` - Cycle through colors on an RGB LED (red, green, blue, yellow, cyan, magenta, white)

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude

## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
