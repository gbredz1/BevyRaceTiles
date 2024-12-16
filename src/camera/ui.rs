use bevy::color::palettes::css::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

use crate::entities::car::{CarControls, CarStates};
use crate::player::Player;

pub(crate) struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(
            Update,
            (update_controls, update_speed, update_road, update_fps),
        );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 18.0,
        ..default()
    };

    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                left: Val::Px(12.0),
                bottom: Val::Px(10.0),
                min_width: Val::Px(250.0),
                padding: UiRect::axes(Val::Px(8.0), Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(BLACK.with_alpha(0.3).into()),
        ))
        .with_children(|builder| {
            builder
                .spawn((Text::new("Speed: "), text_font.clone()))
                .with_child((
                    TextSpan::default(),
                    text_font.clone(),
                    TextColor(GOLD.into()),
                    SpeedText,
                ));
            builder
                .spawn((Text::new("Fps: "), text_font.clone()))
                .with_child((
                    TextSpan::default(),
                    text_font.clone(),
                    TextColor(GOLD.into()),
                    FpsText,
                ));
            builder
                .spawn((Text::new("Road: "), text_font.clone()))
                .with_child((
                    TextSpan::default(),
                    text_font.clone(),
                    TextColor(GOLD.into()),
                    RoadText,
                ));
            builder
                .spawn((Text::new("Controls: "), text_font.clone()))
                .with_child((
                    TextSpan::default(),
                    text_font.clone(),
                    TextColor(GOLD.into()),
                    ControlsText,
                ));
        });
}

#[derive(Component)]
struct ControlsText;

fn update_controls(
    mut text_query: Query<&mut TextSpan, With<ControlsText>>,
    controls_query: Query<&CarControls, With<Player>>,
) {
    for mut text in &mut text_query {
        if let Ok(controls) = controls_query.get_single() {
            **text = format!(
                "gaz={0:.2} brake={1:.2} steering={2:.2}",
                controls.gaz, controls.hand_brake, controls.steering
            );
        }
    }
}

#[derive(Component)]
struct RoadText;

fn update_road(
    mut text_query: Query<&mut TextSpan, With<RoadText>>,
    cars: Query<&CarStates, With<Player>>,
) {
    for mut text in &mut text_query {
        if let Ok(car) = cars.get_single() {
            **text = format!("{}", car.on_road);
        }
    }
}

#[derive(Component)]
struct SpeedText;

fn update_speed(
    states_query: Query<&CarStates, With<Player>>,
    mut query: Query<&mut TextSpan, With<SpeedText>>,
) {
    let player_states: &CarStates = match states_query.get_single() {
        Ok(x) => x,
        Err(_) => return,
    };

    let speed = player_states.speed;
    for mut text in &mut query {
        **text = format!("{speed:.2}");
    }
}

#[derive(Component)]
struct FpsText;

fn update_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut TextSpan, With<FpsText>>,
) {
    for mut text in &mut query {
        let fps = match diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            Some(f) => f,
            None => return,
        };

        if let Some(value) = fps.smoothed() {
            **text = format!("{value:.2}");
        }
    }
}
