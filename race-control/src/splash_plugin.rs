use bevy::prelude::*;
use crate::common::{AppState, despawn_screen};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
        app
            // When entering the state, spawn everything needed for this screen
            .add_system(splash_setup.in_schedule(OnEnter(AppState::Splash)))
            // While in this state, run the `countdown` system
            .add_system(countdown.in_set(OnUpdate(AppState::Splash)))
            // When exiting the state, despawn everything that was spawned for this screen
            .add_system(despawn_screen::<OnSplashScreen>.in_schedule(OnExit(AppState::Splash)));
    }
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnSplashScreen;

// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource)]
pub struct Countdown {
    pub main_timer: Timer,
}

impl Countdown {
    pub fn new() -> Self {
        Self {
            main_timer: Timer::from_seconds(3.0, TimerMode::Once),
        }
    }
}

impl Default for Countdown {
    fn default() -> Self {
        Self::new()
    }
}

/// This system controls ticking the timer within the countdown resource and
/// handling its state.
fn countdown(time: Res<Time>, mut countdown: ResMut<Countdown>, mut game_state: ResMut<NextState<AppState>>) {
    countdown.main_timer.tick(time.delta());

    // The API encourages this kind of timer state checking (if you're only checking for one value)
    // Additionally, `finished()` would accomplish the same thing as `just_finished` due to the
    // timer being repeating, however this makes more sense visually.
    if countdown.main_timer.tick(time.delta()).just_finished() {
        if !countdown.main_timer.finished() {
            // Print the percent complete the main timer is.
            info!(
                "Timer is {:0.0}% complete!",
                countdown.main_timer.percent() * 100.0
            );
            game_state.set(AppState::CheckIn);
        } else {
            // The timer has finished so we pause the percent output timer
            countdown.main_timer.pause();
            info!("Paused percent trigger timer");
        }
    }
}

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("branding/hlo.png");
    // Display the logo
    commands.spawn((
        ImageBundle {
            style: Style {
                // This will center the logo
                margin: UiRect::all(Val::Auto),
                // This will set the logo to be 200px wide, and auto adjust its height
                size: Size::new(Val::Px(200.0), Val::Auto),
                ..default()
            },
            image: UiImage::from(icon),
            ..default()
        },
        OnSplashScreen,
    ));
    // Insert the timer as a resource
    commands.insert_resource(Countdown::default());
}

