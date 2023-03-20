use bevy::prelude::*;
use crate::common::{AppState, ColorScheme, PlayerName};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum NameInputState {
    NotReadyToSubmit,
    ReadyToSubmit,
}

// Tag component used to tag entities added on the Checkin menu screen
#[derive(Component)]
struct OnCheckinMenuScreen;

pub struct CheckinPlugin;

impl Plugin for CheckinPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<AppState>()
            .insert_resource(PlayerName(String::new()))
            .add_system(setup_checkin_menu.in_schedule(OnEnter(AppState::CheckIn)));
    }
}

fn setup_checkin_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    let colors = ColorScheme::new();

    let node_style = Style {
        margin: UiRect::all(Val::Auto),
        size: Size::new(Val::Px(1280.0), Val::Px(720.0)),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        ..default()
    };

    let font_asset =  asset_server.load("fonts/FiraSans-Bold.ttf");

    let button_label_style: TextStyle = TextStyle {
        font: font_asset.clone(),
        font_size: 30.0,
        color: colors.light,
    };

    let start_button_label: TextBundle = TextBundle::from_section(
        "Start",
        button_label_style,
    );

    let button_bundle_style = Style {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        margin: UiRect::all(Val::Auto),
        size: Size::new(Val::Px(180.0), Val::Px(65.0)),
        ..default()
    };

    let start_button = ButtonBundle {
        style: button_bundle_style,
        background_color: colors.accent.into(),
        ..default()
    };

    let text_prompt_style = TextStyle {
        font: font_asset.clone(),
        font_size: 30.0,
        color: colors.main,
    };
    let name_prompt = TextBundle::from_section("Enter player name: ", text_prompt_style);

    let name_input_style = TextStyle {
        font: font_asset.clone(),
        font_size: 60.0,
        color: colors.bold,
    };

    let name_input_container = TextBundle::from_section("", name_input_style);

    commands
        .spawn((
            NodeBundle {
                style: node_style,
                background_color: colors.bg.into(),
                ..default()
            },
            OnCheckinMenuScreen,
            )
        )
        .with_children(|parent| {
            parent.spawn(
                name_prompt.with_style(
                    Style{
                        margin: UiRect::all(Val::Px(80.0)),
                        ..default()
                    }
                ),
            );

            parent.spawn(
                name_input_container.with_style(
                    Style{
                        margin: UiRect::all(Val::Px(80.0)),
                        ..default()
                    }
                ),
            );

            parent
                .spawn(start_button)
                    .with_children(|parent|{
                        parent.spawn(start_button_label);
                    }
                );
            }
        );
}