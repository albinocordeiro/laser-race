use bevy::prelude::{Color, Commands, Component, DespawnRecursiveExt, Entity, Query, Resource, States, With};

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone)]
pub struct PlayerName(pub String);

pub struct ColorScheme{
    pub bg: Color,
    pub main: Color,
    pub bold: Color,
    pub light: Color,
    pub accent: Color,
}

impl ColorScheme {
    pub fn new() -> ColorScheme {
        ColorScheme {
            bg: Color::rgb_u8(8, 51, 89),
            main: Color::rgb_u8(191, 147, 15),
            bold: Color::rgb_u8(191, 130, 15),
            light: Color::rgb_u8(217, 200, 147),
            accent: Color::rgb_u8(166, 47, 20),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    Splash,
    CheckIn,
    Launch,
    Play,
    GameOver,
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}