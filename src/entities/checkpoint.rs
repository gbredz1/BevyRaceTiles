use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::car::CarStates;

pub(crate) struct CheckpointPlugin;

impl Plugin for CheckpointPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, checkpoint);
        app.add_event::<CheckpointEvent>();
    }
}

#[derive(Event, Debug)]
pub(crate) struct CheckpointEvent {
    pub car: Entity,
    pub checkpoint: u8,
}

#[derive(Component, Debug)]
pub(crate) struct Checkpoint(pub u8);

fn checkpoint(
    mut events: EventReader<CollisionEvent>,
    cars: Query<&mut CarStates>,
    checkpoints: Query<&Checkpoint>,
    mut event_writer: EventWriter<CheckpointEvent>,
) {
    for event in &mut events.read() {
        if let CollisionEvent::Started(entity_a, entity_b, _) = event {
            let (car, checkpoint) = match (
                (cars.contains(*entity_a), checkpoints.get(*entity_b)),
                (cars.contains(*entity_b), checkpoints.get(*entity_a)),
            ) {
                ((true, Ok(checkpoint)), _) => (*entity_a, checkpoint),
                (_, (true, Ok(checkpoint))) => (*entity_b, checkpoint),
                (_, _) => continue,
            };

            event_writer.send(CheckpointEvent {
                car: car,
                checkpoint: checkpoint.0,
            });

            info!("checkpoint event: {:?} x {:?}", car, checkpoint);
        }
    }
}
