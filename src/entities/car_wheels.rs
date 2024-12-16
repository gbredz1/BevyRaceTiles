use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub(crate) const WHEEL_FRONT: Vec2 = Vec2 { x: 0.0, y: 45.0 };
pub(crate) const WHEEL_BACK: Vec2 = Vec2 { x: 0.0, y: -45.0 };

#[derive(Component)]
pub struct CarWheel;

#[derive(Bundle)]
pub struct WheelBundle {
    #[bundle()]
    pub collider: Collider,
    pub location: Transform,
    pub sensor: Sensor,
    pub wheel: CarWheel,
}

impl Default for WheelBundle {
    fn default() -> Self {
        Self {
            collider: Collider::capsule_x(25.0, 10.0),
            location: Transform::from_xyz(0.0, 45.0, 0.0),
            sensor: Sensor,
            wheel: CarWheel,
        }
    }
}

impl WheelBundle {
    pub fn new(position: Vec2) -> Self {
        Self {
            location: Transform::from_xyz(position.x, position.y, 0.0),
            ..default()
        }
    }
}
