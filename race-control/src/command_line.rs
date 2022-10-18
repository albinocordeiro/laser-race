use std::fs;
use std::path::PathBuf;
use anyhow::{bail, Context, Result};
use log::warn;
use serialport::SerialPort;
use crate::{mocks, Parser};
use crate::analog_thresholds::AnalogDetectThresholds;

/// Cli option definition
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path for the laser sensor signal calibration file
    /// File format example in the this repo
    #[arg(short, long , value_name="CALIBRATION_FILE", default_value="SensorCalibration.toml")]
    pub calibration_file: PathBuf,

    /// Arduino device path
    #[arg(short, long, value_name="ARDUINO_0_DEVICE_PATH", default_value="/dev/ttyACM0")]
    pub device_path: PathBuf,

    /// Debug level
    #[arg(short, long , action = clap::ArgAction::Count)]
    pub level_of_debug_messages: u8,

    /// Test mode flag: In this mode the app can be tested without a connected Arduino device.
    #[arg(short, long)]
    pub test: bool,
}

pub fn get_sensor_calibration_data(calibration_file_path: &str, test_mode: bool) -> Result<AnalogDetectThresholds> {
    let thresholds = if let Ok(thresholds_txt) = fs::read_to_string(calibration_file_path) {
        toml::from_str(&thresholds_txt).context("Failed to parse thresholds from file contents")?
    } else {
        if test_mode {
            warn!("Could not read sensor calibration file: {:?}.", calibration_file_path);
            AnalogDetectThresholds::default()
        } else {
            bail!("Could not read sensor calibration file: {:?}.", calibration_file_path);
        }
    };

    Ok(thresholds)
}

pub fn get_serial_port_connection(device_path: &str, test_mode: bool) -> Result<Box<dyn SerialPort>> {
    let serial_port: Box<dyn SerialPort> = match serialport::new(device_path, 9600).open() {
        Ok(port) => {
            port
        },
        Err(e) => {
            if test_mode {
                warn!("Could not open serial port: {:?} with error: {:?}", device_path, e);
                Box::new(mocks::DummySerialPort::new())
            } else {
                bail!("Could not open serial port: {:?} with error: {:?}", device_path, e);
            }
        }
    };

    Ok(serial_port)
}