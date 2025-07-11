use super::checkpoint::CheckpointEvent;
use crate::entities::car::CarControls;
use crate::level::LevelProperties;
use bevy::prelude::*;
use bevy::utils::HashSet;

pub(crate) struct CarCheckpointPlugin;

impl Plugin for CarCheckpointPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(SpawnScene, put_on_cars)
            .add_systems(Update, update);
    }
}

#[derive(Component, Debug, Default)]
pub(crate) struct CheckpointTracker {
    pub checkpoint_visits: HashSet<usize>,
    pub laps_completed: usize,
    pub total_checkpoints: usize,
}

impl CheckpointTracker {
    fn all_checkpoints_completed(&self) -> bool {
        self.checkpoint_visits.len() == self.total_checkpoints
    }
}

fn put_on_cars(
    mut commands: Commands,
    query: Query<Entity, (Without<CheckpointTracker>, Added<CarControls>)>,
    level_properties: Res<LevelProperties>,
) {
    for entity in &query {
        commands.entity(entity).insert(CheckpointTracker {
            checkpoint_visits: HashSet::new(),
            laps_completed: 0,
            total_checkpoints: level_properties.checkpoints as usize,
        });
    }
}

fn update(
    mut events: EventReader<CheckpointEvent>,
    mut query: Query<&mut CheckpointTracker>,
) {
    for event in &mut events.read() {
        let Ok(mut tracker) = query.get_mut(event.car) else {
            return;
        };

        if event.checkpoint == tracker.checkpoint_visits.len() {
            tracker.checkpoint_visits.insert(event.checkpoint);
        } else if event.checkpoint + 2 == tracker.checkpoint_visits.len() {
            tracker.checkpoint_visits.remove(&(event.checkpoint + 1));
        } else if event.checkpoint == 0 && tracker.all_checkpoints_completed() {
            tracker.laps_completed += 1;
            // todo event for lap complete

            tracker.checkpoint_visits.clear();
            tracker.checkpoint_visits.insert(event.checkpoint);
        }
    }
}
