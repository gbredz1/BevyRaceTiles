use std::collections::HashMap;

use bevy::color::palettes::css::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_rapier2d::prelude::*;
use tiled::{Map, PropertyValue};

use crate::entities::car::{CarBundle, CarColor};
use crate::entities::car_wheels::{WheelBundle, WHEEL_BACK, WHEEL_FRONT};
use crate::entities::checkpoint::Checkpoint;
use crate::entities::road::Road;
use crate::player::Player;

pub(crate) const ROAD_GROUP: Group = Group::GROUP_1;
pub(crate) const WALL_GROUP: Group = Group::GROUP_2;
pub(crate) const CHECK_GROUP: Group = Group::GROUP_3;
pub(crate) const CAR_GROUP: Group = Group::GROUP_4;

pub(crate) struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TiledPhysicsPlugin::<MyCustomRapierPhysicsBackend>::default(),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        ));

        app.add_systems(Startup, setup);
    }
}

fn setup(mut rapier_config: Query<&mut RapierConfiguration>) {
    let Ok(mut rapier_config) = rapier_config.get_single_mut() else {
        return;
    };

    rapier_config.gravity = Vec2::ZERO;
}

#[derive(Default)]
struct MyCustomRapierPhysicsBackend(TiledPhysicsRapierBackend);

impl TiledPhysicsBackend for MyCustomRapierPhysicsBackend {
    fn spawn_collider(
        &self,
        commands: &mut Commands,
        map: &Map,
        collider_source: &TiledColliderSource,
    ) -> Option<TiledColliderSpawnInfos> {
        let collider = self.0.spawn_collider(commands, map, collider_source);
        let collider = match collider {
            Some(c) => c,
            None => return None,
        };

        let properties = get_properties(collider_source, map)?;

        match properties.get("group") {
            // WALL
            Some(&PropertyValue::IntValue(0)) => {
                commands.entity(collider.entity).insert((
                    ColliderDebugColor(GRAY.into()),
                    CollisionGroups::new(WALL_GROUP, CAR_GROUP),
                ));
            }
            // ROAD
            Some(&PropertyValue::IntValue(1)) => {
                commands.entity(collider.entity).insert((
                    Sensor,
                    ColliderDebugColor(GREEN_YELLOW.into()),
                    CollisionGroups::new(ROAD_GROUP, CAR_GROUP),
                    Road,
                ));
            }
            // CHECKPOINT
            Some(&PropertyValue::IntValue(2)) => {
                let value = get_checkpoint_value(&properties)
                    .expect("Checkpoint with no value properties");

                commands.entity(collider.entity).insert((
                    Sensor,
                    ColliderDebugColor(BLUE.into()),
                    CollisionGroups::new(CHECK_GROUP, CAR_GROUP),
                    ActiveEvents::COLLISION_EVENTS,
                    Checkpoint(value),
                ));
            }
            // CAR
            Some(&PropertyValue::IntValue(3)) => {
                let car_color = get_car_color(&properties)?;

                commands
                    .entity(collider.entity)
                    .insert((
                        ColliderDebugColor(BLACK.into()),
                        CollisionGroups::new(
                            CAR_GROUP,
                            CAR_GROUP | WALL_GROUP | CHECK_GROUP | ROAD_GROUP,
                        ),
                        CarBundle::new(car_color),
                    ))
                    .with_children(|c| {
                        // front wheels
                        c.spawn((
                            WheelBundle::new(WHEEL_FRONT),
                            ColliderDebugColor(BLUE.into()),
                        ));
                        c.spawn((
                            WheelBundle::new(WHEEL_BACK),
                            ColliderDebugColor(RED.into()),
                        ));
                    });

                if let Some(_) = get_player(&properties) {
                    commands.entity(collider.entity).insert(Player).insert(
                        Name::new("PlayerControlledObject (Rapier physics)"),
                    );
                }
            }
            Some(_) => {
                warn!("unknow group: {:?}", collider_source);
            }
            None => {
                warn!("no group: {:?}", collider_source);
            }
        }

        Some(collider)
    }
}

fn get_properties(
    collider_source: &TiledColliderSource,
    map: &Map,
) -> Option<HashMap<String, PropertyValue>> {
    let tile = collider_source.tile(map);
    let object = collider_source.object(map);
    let object_data = (match collider_source.ty {
        TiledColliderSourceType::Tile {
            layer_id: _,
            x: _,
            y: _,
            object_id,
        } => tile
            .as_ref()
            .and_then(|tile| tile.collision.as_ref())
            .map(|collision| collision.object_data())
            .and_then(|objects| objects.get(object_id)),
        TiledColliderSourceType::Object {
            layer_id: _,
            object_id: _,
        } => object.as_deref(),
    })?;

    let mut properties = object_data.properties.clone();
    properties.extend(collider_source.layer(map).unwrap().properties.clone());

    Some(properties)
}

fn get_car_color(
    properties: &HashMap<String, PropertyValue>,
) -> Option<CarColor> {
    let color_name = match properties.get("color").as_ref() {
        Some(&PropertyValue::StringValue(color_name)) => color_name,
        None | Some(_) => return None,
    };

    match color_name.as_str() {
        "red" => Some(CarColor::Red),
        "yellow" => Some(CarColor::Yellow),
        "green" => Some(CarColor::Green),
        "blue" => Some(CarColor::Blue),
        "black" => Some(CarColor::Black),
        _ => None,
    }
}

fn get_player(properties: &HashMap<String, PropertyValue>) -> Option<u8> {
    match properties.get("player") {
        Some(&PropertyValue::IntValue(i)) => Some(i as u8),
        None | Some(_) => None,
    }
}

fn get_checkpoint_value(
    properties: &HashMap<String, PropertyValue>,
) -> Option<usize> {
    match properties.get("value") {
        Some(&PropertyValue::IntValue(i)) => Some(i as usize),
        None | Some(_) => None,
    }
}
