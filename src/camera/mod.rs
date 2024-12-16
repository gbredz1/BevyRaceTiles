pub(crate) mod ui;

use bevy::prelude::*;

use crate::entities::car::{car_driving, CarStates};
use crate::player::Player;

const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const SCALE_MAX: f32 = 3.0;
const SCALE_MIN: f32 = 1.8;

pub(crate) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ui::UIPlugin);

        app.add_systems(Startup, setup)
            .insert_resource(ClearColor(BACKGROUND_COLOR));

        app.add_systems(
            Update,
            (
                camera_follow
                    .after(TransformSystem::TransformPropagate)
                    .after(car_driving),
                camera_scale.after(car_driving),
            ),
        );
    }
}

#[derive(Component)]
pub(crate) struct GameCamera;

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d::default(), GameCamera));
}

fn camera_follow(
    transform_query: Query<&GlobalTransform, With<Player>>,
    mut camera_query: Query<&mut Transform, With<GameCamera>>,
) {
    let Ok(mut camera) = camera_query.get_single_mut() else {
        return;
    };
    let Ok(player_transform) = transform_query.get_single() else {
        return;
    };

    let player_transform = player_transform.compute_transform();
    camera.translation.x = player_transform.translation.x;
    camera.translation.y = player_transform.translation.y;
}

fn camera_scale(
    time: Res<Time>,
    states_query: Query<&CarStates, With<Player>>,
    mut camera_query: Query<
        (&mut Transform, &mut OrthographicProjection),
        (With<GameCamera>, Without<Player>),
    >,
) {
    let Ok(mut camera) = camera_query.get_single_mut() else {
        return;
    };

    let player_states = match states_query.get_single() {
        Ok(x) => x,
        Err(_) => return,
    };

    let car_speed_percent = player_states.speed.abs() / player_states.max_speed;

    let scale = if car_speed_percent < 0.33 {
        SCALE_MIN - camera.1.scale
    } else if car_speed_percent < 0.66 {
        SCALE_MAX / 2.0 - camera.1.scale
    } else {
        SCALE_MAX - camera.1.scale
    };
    camera.1.scale += scale * time.delta_secs();

    camera.1.scale = camera.1.scale.clamp(SCALE_MIN, SCALE_MAX);
}
