use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_ecs_tiled::debug::TiledMapDebugPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::render::RapierDebugRenderPlugin;

pub struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierDebugRenderPlugin::default(),
            TiledMapDebugPlugin::default(),
            WorldInspectorPlugin::default()
                .run_if(input_toggle_active(false, KeyCode::Escape)),
            FrameTimeDiagnosticsPlugin { ..default() },
        ));
    }
}
