use anyhow::Result;
use bevy::prelude::*;

use crate::analog_thresholds::AnalogDetectThresholds;
use crate::checkin_plugin::CheckinPlugin;
use crate::common::AppState;
use crate::splash_plugin::SplashPlugin;


#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Deref)]
struct DevicePath(String);

pub fn run(thresholds: AnalogDetectThresholds, device_path: &str) -> Result<()> {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(thresholds)
        .insert_resource(DevicePath(device_path.to_string()))
        .add_startup_system(setup)
        .add_state::<AppState>()
        .add_plugin(SplashPlugin)
        .add_plugin(CheckinPlugin)
        .run();

    Ok(())
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

