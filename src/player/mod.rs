use bevy::prelude::*;

use crate::entities::car::CarControls;

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keyboard_input);
    }
}

#[derive(Component)]
pub struct Player;

pub fn keyboard_input(
    button_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut CarControls, With<Player>>,
) {
    let mut controls = match query.get_single_mut() {
        Ok(x) => x,
        Err(_) => return,
    };

    if button_input.pressed(KeyCode::ArrowUp) {
        controls.gaz = 1.0;
    } else if button_input.pressed(KeyCode::ArrowDown) {
        controls.gaz = -1.0;
    } else {
        controls.gaz = 0.0;
    }

    if button_input.pressed(KeyCode::ArrowLeft) {
        controls.steering = 1.0;
    } else if button_input.pressed(KeyCode::ArrowRight) {
        controls.steering = -1.0;
    } else {
        controls.steering = 0.0;
    }

    if button_input.pressed(KeyCode::Space) {
        controls.hand_brake = 1.0;
    } else {
        controls.hand_brake = 0.0;
    }
}
