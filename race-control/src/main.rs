use std::path::PathBuf;
use std::fs::read_to_string;
use std::ptr::read;

use anyhow::{Context, Result};
use clap::Parser;
use log::{debug, error, info, warn};
use serde_derive::{Deserialize};
use toml;

/// Cli option definition
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path for the laser sensor signal calibration file
    /// File format example in the this repo
    #[arg(short, long , value_name="CALIBRATION_FILE", default_value="SensorCalibration.toml")]
    calibration_file: PathBuf,

    /// Arduino device path
    #[arg(short, long, value_name="ARDUINO_0_DEVICE_PATH", default_value="/dev/ttyACM0")]
    ard_0_path: PathBuf,

    /// Debug level
    #[arg(short, long , action = clap::ArgAction::Count)]
    debug: u8,
}

#[derive(Deserialize)]
struct AnalogDetectThresholds {
    A0_low: u8,
    A0_high: u8,
    A1_low: u8,
    A1_high: u8,
    A2_low: u8,
    A2_high: u8,
    A3_low: u8,
    A3_high: u8,
    A4_low: u8,
    A4_high: u8,
    A5_low: u8,
    A5_high: u8,
}

impl Default for AnalogDetectThresholds {
    fn default() -> Self {
        AnalogDetectThresholds {
            A0_low: 0,
            A0_high: 0,
            A1_low: 0,
            A1_high: 0,
            A2_low: 0,
            A2_high: 0,
            A3_low: 0,
            A3_high: 0,
            A4_low: 0,
            A4_high: 0,
            A5_low: 0,
            A5_high: 0,
        }
    }
}
#[tokio::main]
async fn main() -> Result<()> {
    println!("
        ░█▀▄░█▀█░█▀▀░█▀▀░░░█▀▀░█▀█░█▀█░▀█▀░█▀▄░█▀█░█░░
        ░█▀▄░█▀█░█░░░█▀▀░░░█░░░█░█░█░█░░█░░█▀▄░█░█░█░░
        ░▀░▀░▀░▀░▀▀▀░▀▀▀░░░▀▀▀░▀▀▀░▀░▀░░▀░░▀░▀░▀▀▀░▀▀▀
    ");

    let cli = Cli::parse();
    let thresholds = if let Some(thresholds_txt) = read_to_string(&cli.calibration_file) {
        toml::from_str(&thresholds_txt).context("Failed to parse thresholds from file")?
    } else {
        AnalogDetectThresholds::default()
    };



}



#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
