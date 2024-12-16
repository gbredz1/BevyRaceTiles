pub(crate) mod car;
pub(crate) mod car_wheels;
pub(crate) mod checkpoint;
pub(crate) mod road;

use bevy::prelude::*;

use car::CarPlugin;
use checkpoint::CheckpointPlugin;
use road::RoadPlugin;

pub(crate) struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CarPlugin, RoadPlugin, CheckpointPlugin));
    }
}
