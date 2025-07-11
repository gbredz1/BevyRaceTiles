pub(crate) mod physics;

use core::panic;

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use tiled::PropertyValue;

pub(crate) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TilemapPlugin,
            TiledMapPlugin::default(),
            physics::PhysicsPlugin,
        ))
        .add_systems(Startup, setup)
        .init_resource::<LevelProperties>()
        .add_observer(map_created);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle: Handle<TiledMap> = asset_server.load("map/level0.tmx");
    let mut map_entity = commands.spawn(TiledMapHandle(map_handle));

    map_entity.insert((
        TiledMapSettings::default(),
        TilemapRenderSettings::default(),
    ));
}

#[derive(Resource, Default, Debug)]
pub(crate) struct LevelProperties {
    pub checkpoints: u32,
    pub laps: u32,
}

fn map_created(
    trigger: Trigger<TiledMapCreated>,
    map_asset: Res<Assets<TiledMap>>,
    mut level_properties: ResMut<LevelProperties>,
) {
    let map = trigger.event().map(&map_asset);

    level_properties.checkpoints = match map.properties.get("checkpoints") {
        Some(&PropertyValue::IntValue(i)) => i as u32,
        None | Some(_) => {
            panic!("error loading checkpoints properties")
        }
    };

    level_properties.laps = match map.properties.get("laps") {
        Some(&PropertyValue::IntValue(i)) => i as u32,
        None | Some(_) => {
            panic!("error loading checkpoints properties")
        }
    };
}
