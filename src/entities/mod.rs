pub(crate) mod car;
pub(crate) mod car_brakelights;
pub(crate) mod car_checkpoint;
pub(crate) mod car_wheels;
pub(crate) mod checkpoint;
pub(crate) mod road;

use bevy::prelude::*;

use crate::entities::car_checkpoint::CarCheckpointPlugin;
use car::CarPlugin;
use car_brakelights::CarBrakeLightsPlugin;
use checkpoint::CheckpointPlugin;
use road::RoadPlugin;

pub(crate) struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CarPlugin,
            RoadPlugin,
            CheckpointPlugin,
            CarBrakeLightsPlugin,
            CarCheckpointPlugin,
        ));
    }
}
