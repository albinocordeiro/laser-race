use std::path::PathBuf;
use std::fs;

use anyhow::{bail, Context, Result};
use clap::Parser;
use log::{info, warn};
use serialport::SerialPort;
use serde_derive::{Deserialize};
use toml;
use crate::command_line::{Cli, get_sensor_calibration_data, get_serial_port_connection};

mod analog_thresholds;
mod command_line;
mod mocks;

#[tokio::main]
async fn main() -> Result<()> {
    println!("
        ░█▀▄░█▀█░█▀▀░█▀▀░░░█▀▀░█▀█░█▀█░▀█▀░█▀▄░█▀█░█░░
        ░█▀▄░█▀█░█░░░█▀▀░░░█░░░█░█░█░█░░█░░█▀▄░█░█░█░░
        ░▀░▀░▀░▀░▀▀▀░▀▀▀░░░▀▀▀░▀▀▀░▀░▀░░▀░░▀░▀░▀▀▀░▀▀▀
    ");

    let cli = Cli::parse();

    if cli.test {
        info!("Running in test mode");
    }

    let calibration_file = cli.calibration_file.to_str()
        .context("A calibration file path must be provided. Follow instructions from the help menu (> race-control --help)")?;
    let thresholds = get_sensor_calibration_data(calibration_file, cli.test)?;

    let device_path = cli.device_path.to_str()
        .context("A device path must be provided. Follow instructions from the help menu (> race-control --help)")?;
    let serial_port = get_serial_port_connection(device_path, cli.test)?;

    let main_loop = MainLoop::new(thresholds, serial_port);
    main_loop.run().context("Main loop interrupted with error")?;
    
    info!("Done...");
    Ok(())
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}