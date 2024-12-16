mod camera;
#[cfg(feature = "dev_mode")]
mod dev_tools;
mod entities;
mod level;
mod player;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        default_plugins,
        level::LevelPlugin,
        player::PlayerPlugin,
        camera::CameraPlugin,
        entities::EntitiesPlugin,
    ));

    #[cfg(feature = "dev_mode")]
    app.add_plugins(dev_tools::DevToolsPlugin);

    app.run();
}

fn default_plugins(app: &mut App) {
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(
        WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Race with somes tiles".into(),
                resolution: (1280., 700.).into(),
                ..default()
            }),
            ..default()
        },
    ));
}
