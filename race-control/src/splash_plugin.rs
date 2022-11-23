use bevy::app::App;
use bevy::prelude::{AssetServer, Commands, default, Component, Resource, Deref, DerefMut, ImageBundle, Plugin, Res, ResMut, Size, State, Style, SystemSet, Time, Timer, TimerMode, UiImage, UiRect, Val};
use crate::common::{AppState, despawn_screen};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
        app
            // When entering the state, spawn everything needed for this screen
            .add_system_set(SystemSet::on_enter(AppState::Splash).with_system(splash_setup))
            // While in this state, run the `countdown` system
            .add_system_set(SystemSet::on_update(AppState::Splash).with_system(countdown))
            // When exiting the state, despawn everything that was spawned for this screen
            .add_system_set(SystemSet::on_exit(AppState::Splash).with_system(despawn_screen::<OnSplashScreen>),
            );
    }
}

// Tag component used to tag entities added on the splash screen
#[derive(Component)]
struct OnSplashScreen;

// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

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
            image: UiImage(icon),
            ..default()
        },
        OnSplashScreen,
    ));
    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}

// Tick the timer, and change state when finished
fn countdown(
    mut game_state: ResMut<State<AppState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(AppState::CheckIn).unwrap();
    }
}