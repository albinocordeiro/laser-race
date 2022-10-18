use std::path::PathBuf;

use clap::Parser;

/// Cli option definition
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path for the laser sensor signal calibration file: SensorCalibration.toml
    /// File format example in the this repo
    #[arg(short, long , value_name="CALIBRATION_FILE", default_value="SensorCalibration.toml")]
    calibration_file: PathBuf,

    /// Arduino device path (ex.: /dev/ttyACM0)
    #[arg(short, long, value_name="ARDUINO_0_DEVICE_PATH", default_value="/dev/ttyACM0")]
    ard0_path: PathBuf,

    /// Debug level
    #[arg(short, long , action = clap::ArgAction::Count)]
    debug: u8,
}

fn main() {
    println!("
        ░█▀▄░█▀█░█▀▀░█▀▀░░░█▀▀░█▀█░█▀█░▀█▀░█▀▄░█▀█░█░░
        ░█▀▄░█▀█░█░░░█▀▀░░░█░░░█░█░█░█░░█░░█▀▄░█░█░█░░
        ░▀░▀░▀░▀░▀▀▀░▀▀▀░░░▀▀▀░▀▀▀░▀░▀░░▀░░▀░▀░▀▀▀░▀▀▀
    ");

    let cli = Cli::parse();
}



#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
