use std::path::PathBuf;
use std::fs;

use anyhow::{bail, Context, Result};
use clap::Parser;
use log::{debug, error, info, warn};
use serialport::SerialPort;
use serde_derive::{Deserialize};
use toml;

mod mocks;


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

    /// Test mode flag
    #[arg(short, long)]
    test: bool,
}

#[derive(Deserialize)]
struct AnalogDetectThresholds {
    a0_low: u8,
    a0_high: u8,
    a1_low: u8,
    a1_high: u8,
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
            a0_low: 0,
            a0_high: 0,
            a1_low: 0,
            a1_high: 0,
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

    // Load sensor calibration data
    let thresholds = if let Ok(thresholds_txt) = fs::read_to_string(&cli.calibration_file.as_path()) {
        toml::from_str(&thresholds_txt).context("Failed to parse thresholds from file contents")?
    } else {
        if cli.test {
            warn!("Could not read sensor calibration file: {:?}.", cli.calibration_file.as_path());
            AnalogDetectThresholds::default()
        } else {
            bail!("Could not read sensor calibration file: {:?}.", cli.calibration_file.as_path());
        }
    };

    // Connect to serial port
    let serial_port: Box<dyn SerialPort> = match SerialPort::new(cli.device_path.to_str(), 9600).open() {
        Ok(port) => {
            port
        },
        Err(e) => {
            if cli.test {
                warn!("Could not open serial port: {:?} with error: {:?}", cli.device_path.as_path(), e);
                Box::new(mocks::DummySerialPort::new())
            } else {
                bail!("Could not open serial port: {:?} with error: {:?}", cli.device_path.as_path(), e);
            }
        }
    };
}



#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
