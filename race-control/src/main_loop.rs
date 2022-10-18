use std::sync::Arc;

use anyhow::Result;
use bevy::prelude::*;
use serialport::SerialPort;

use crate::analog_thresholds::AnalogDetectThresholds;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum AppState {
    CheckIn,
    Launch,
    Play,
    GameOver,
}

pub fn run(thresholds: AnalogDetectThresholds, serial_port: Box<dyn SerialPort>) -> Result<()> {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(thresholds)
        .insert_resource(Arc::new(serial_port))
        .add_startup_system(setup)
        .add_state(AppState::CheckIn)
        .run();

    Ok(())
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
