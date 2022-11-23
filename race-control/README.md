# race-control

[![Crates.io](https://img.shields.io/crates/v/race-control.svg)](https://crates.io/crates/race-control)
[![Docs.rs](https://docs.rs/race-control/badge.svg)](https://docs.rs/race-control)
[![CI](https://github.com/albinocordeiro/race-control/workflows/CI/badge.svg)](https://github.com/albinocordeiro/race-control/actions)
[![Coverage Status](https://coveralls.io/repos/github/albinocordeiro/race-control/badge.svg?branch=main)](https://coveralls.io/github/albinocordeiro/race-control?branch=main)

## Installation

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install race-control`

## Design
### Color scheme
https://colordesigner.io/#083359-BF930F-BF820F-D9C893-A62F14

### Game Modes

* **CheckIn**
    * Show sensor statuses or just a SYSTEMS UP message if sensors seem all right, that is,
      they are all activated on the sensor-high state. If there is a sensor on low state, return
      message stating which sensor is low which gives the admin the option to just yank out the bad sensor/laser pair.
    * If current state is SYSTEMS UP, a _Enter player name_ input prompt should be displayed.
    * If a player name is entered transition to **Launch** mode.
* **Launch**
    * Start monitoring serial port (Arduino) for the _start signal_ (specific digital arduino GPIO). If start signal on
      the serial port
      is detected or _L_ keyboard key is pressed then transition to **Play** mode.
    * If _Esc_ keyboard key is pressed then transition back to **CheckIn** mode.
* **Play**
    * Start monitoring serial port for laser sensor levels and detect hits based on thresholds in the calibration.toml
      file.
      Also monitor for the _end signal_ or _D_ keyboard key.
    * If _end signal_ or _D_ key is detected then, save player name, current time, hits, score transition to **
      GameOver** mode.
    * Display player name, timer and hit count
    * If _Esc_ is detected transition to **CheckIn** mode.
* **GameOver**
    * Display _Game Over_ message
    * Player name and score
    * Leader board
    * Monitor for _Esc_, when detected transition to **CheckIn** mode.

**Note:** at any point Ctrl+C will terminate the program

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
