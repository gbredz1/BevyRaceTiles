use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::car::CarStates;
use super::car_wheels::CarWheel;

pub struct RoadPlugin;
impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_car_on_road);
    }
}

#[derive(Component)]
pub struct Road;

fn update_car_on_road(
    rapier_context: ReadDefaultRapierContext,
    mut cars: Query<(&mut CarStates, &Children)>,
    wheels: Query<&CarWheel>,
    roads: Query<(&mut Road, Entity)>,
) {
    for (mut car, children) in cars.iter_mut() {
        car.on_road = false;

        for &child in children.iter() {
            if wheels.get(child).is_err() {
                continue; // not a wheel
            }

            for (_, road_entity) in roads.iter() {
                if rapier_context.intersection_pair(child, road_entity)
                    == Some(true)
                {
                    car.on_road = true;
                }
            }
        }
    }
}
