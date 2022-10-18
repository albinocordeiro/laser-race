use bevy::prelude::*;

enum AppState {
    CheckIn,
    Launch,
    Play,
    GameOver,
}

pub struct MainLoop {
    app: App,
}

impl MainLoop {
    pub fn new(thresholds: AnalogDetectThresholds, serial_port: Box<dyn SerialPort>) -> Self {
        let app = App::new()
            .add_plugins(DefaultPlugins)
            .insert_resource(thresholds)
            .insert_resource(serial_port)
            .add_startup_system(setup)
            .add_state(AppState::CheckIn);

        Self { app }
    }
    pub fn run(&self) {
        self.app.run();
    }
}
