pub(crate) mod physics;

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub(crate) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TilemapPlugin,
            TiledMapPlugin::default(),
            physics::PhysicsPlugin,
        ))
        .add_systems(Startup, setup);
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
