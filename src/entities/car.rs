use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub(crate) struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sprite_init)
            .add_systems(Update, (car_update, car_driving));
    }
}

#[derive(Bundle)]
pub struct CarBundle {
    #[bundle()]
    pub sprite: Sprite,
    pub body: RigidBody,
    pub velocity: Velocity,
    pub color: CarColor,
    pub mass: ColliderMassProperties,
    pub damping: Damping,
    pub state: CarStates,
    pub controls: CarControls,
}

impl Default for CarBundle {
    fn default() -> Self {
        Self {
            sprite: Default::default(),
            body: RigidBody::Dynamic,
            velocity: Velocity::zero(),
            color: CarColor::Red,
            mass: ColliderMassProperties::Density(50.0),
            damping: Damping::default(),
            state: CarStates::default(),
            controls: CarControls::default(),
        }
    }
}

impl CarBundle {
    pub fn new(color: CarColor) -> Self {
        Self { color, ..default() }
    }
}

#[derive(Component)]
pub(crate) enum CarColor {
    Black,
    Blue,
    Green,
    Red,
    Yellow,
}
fn sprite_init(
    asset_server: Res<AssetServer>,
    mut query: Query<(&CarColor, &mut Sprite)>,
) {
    for (color, mut sprite) in &mut query {
        sprite.image = asset_server.load(format!(
            "map/tilesheets/entities/car_{}_1.png",
            match color {
                CarColor::Black => "black",
                CarColor::Blue => "blue",
                CarColor::Green => "green",
                CarColor::Red => "red",
                CarColor::Yellow => "yellow",
            }
        ));
    }
}

#[derive(Component)]
pub(crate) struct CarControls {
    pub gaz: f32,
    pub steering: f32,
    pub hand_brake: f32,
}

impl Default for CarControls {
    fn default() -> Self {
        Self {
            gaz: 0.0,
            steering: 0.0,
            hand_brake: 0.0,
        }
    }
}

#[derive(Component)]
pub(crate) struct CarStates {
    pub speed: f32,
    pub on_road: bool,
    pub turn_factor: f32,
    pub gaz_factor: f32,
    pub drift_factor: f32,
    pub max_speed: f32,
    pub damping_add: f32,
    pub brake: bool,
}

impl Default for CarStates {
    fn default() -> Self {
        Self {
            speed: 0.0,
            on_road: false,
            drift_factor: 0.95,
            gaz_factor: 800.0,
            turn_factor: 120.0,
            max_speed: 1000.0,
            damping_add: 0.0,
            brake: false,
        }
    }
}

fn car_update(mut cars: Query<&mut CarStates>) {
    for mut car in &mut cars {
        if car.on_road {
            car.max_speed = 1000.0;
            car.drift_factor = 0.95;
            car.damping_add = 0.0;
        } else {
            car.max_speed = 800.0;
            car.drift_factor = 0.99;
            car.damping_add = 2.0;
        }
    }
}

pub(crate) fn car_driving(
    time: Res<Time>,
    mut query: Query<(
        &CarControls,
        &Transform,
        &mut Velocity,
        &mut Damping,
        &mut CarStates,
    )>,
) {
    for (controls, transform, mut velocity, mut damping, mut car) in &mut query
    {
        let mut drift_factor = car.drift_factor;

        // Acceleration operations
        // Apply acceleration force in the vehicle's direction
        velocity.linvel += controls.gaz
            * car.gaz_factor
            * transform.up().truncate()
            * time.delta_secs();

        // Limit maximum vehicle speed
        velocity.linvel = velocity.linvel.clamp_length_max(car.max_speed);

        // Engine brake
        // If no acceleration command is active, apply engine braking
        if controls.gaz == 0.0 {
            damping.linear_damping = 0.5;
        } else {
            damping.linear_damping = 0.0;
        }
        // Add additional damping based on the surface type
        damping.linear_damping += car.damping_add;

        // Forward and backward braking
        // Detect braking when input is opposite to a movement direction
        let brake = (controls.gaz < 0.0 && car.speed > 0.0)
            || (controls.gaz > 0.0 && car.speed < 0.0);
        if brake {
            damping.linear_damping = controls.gaz.abs() * 4.0;
        }
        car.brake = true;

        // Hand brake
        // Increase damping and drift factor to simulate stronger braking
        if controls.hand_brake > 0.0 {
            damping.linear_damping = controls.hand_brake * 5.0;
            drift_factor = 0.99;
        }

        // Calculate current vehicle speed
        car.speed = velocity.linvel.dot(transform.up().truncate());

        // Steering operations
        // Minimum speed factor to prevent rotation while stationary
        let min_speed_factor = if car.speed.abs() > 25.0 { 1.0 } else { 0.0 };
        // Reverse factor for backward movement
        let reverse_factor = if car.speed > 0.0 { 1.0 } else { -1.0 };
        velocity.angvel = controls.steering
            * car.turn_factor
            * reverse_factor
            * min_speed_factor
            * time.delta_secs();

        // Lateral velocity calculation
        // Decompose velocity into a forward/backward component
        let forward_velocity = transform.up().truncate()
            * velocity.linvel.dot(transform.up().truncate());

        // Decompose velocity into lateral component
        let right_velocity = transform.right().truncate()
            * velocity.linvel.dot(transform.right().truncate());

        // Adjust lateral velocity according to the desired drift factor
        velocity.linvel = forward_velocity + right_velocity * drift_factor;

        // Round to 0.01
        // If velocity is very low, consider it zero to prevent sliding
        if velocity.linvel.length() < 0.01 {
            velocity.linvel = Vec2::ZERO;
        }
    }
}
