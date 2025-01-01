use super::car::CarControls;
use bevy::prelude::*;

pub(crate) struct CarBrakeLightsPlugin;

impl Plugin for CarBrakeLightsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (put_on_cars, update));
    }
}

#[derive(Component)]
pub(crate) struct BrakeLight;

#[derive(Component)]
pub(crate) struct BrakeLightCreated;

fn put_on_cars(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<Entity, (Without<BrakeLightCreated>, Added<CarControls>)>,
) {
    for entity in &query {
        commands.entity(entity).insert(BrakeLightCreated);

        let (x, y, z) = (26., -56., -1.);
        let mesh = Mesh2d(meshes.add(RegularPolygon::new(8., 6)));
        let mesh_material =
            MeshMaterial2d(materials.add(Color::srgba(200., 0., 0., 1.)));

        let child = commands
            .spawn((
                BrakeLight,
                mesh.clone(),
                mesh_material.clone(),
                Transform::from_translation(Vec3::new(-x, y, z)),
                Visibility::Inherited,
            ))
            .id();
        commands.entity(entity).add_child(child);

        let child = commands
            .spawn((
                BrakeLight,
                mesh.clone(),
                mesh_material.clone(),
                Transform::from_translation(Vec3::new(x, y, z)),
                Visibility::Inherited,
            ))
            .id();
        commands.entity(entity).add_child(child);
    }
}

fn update(
    mut query: Query<(&mut Visibility, &Parent), With<BrakeLight>>,
    controls_query: Query<&CarControls, Changed<CarControls>>,
) {
    for (mut visibility, parent) in &mut query {
        let Ok(controls) = controls_query.get(parent.get()) else {
            return;
        };

        *visibility = match controls.hand_brake {
            0.0 => Visibility::Hidden,
            _ => Visibility::Inherited,
        };
    }
}
