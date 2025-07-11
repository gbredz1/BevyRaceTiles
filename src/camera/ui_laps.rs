use crate::entities::car_checkpoint::CheckpointTracker;
use crate::level::LevelProperties;
use crate::player::Player;
use bevy::color::palettes::css::*;
use bevy::prelude::*;

pub(crate) struct UILapsPlugin;

impl Plugin for UILapsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(SpawnScene, update_total)
            .add_systems(Update, update);
    }
}

#[derive(Component)]
struct TotalLapsText;

#[derive(Component)]
struct LapsText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let font_bold = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 24.,
        ..default()
    };

    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                top: Val::Px(10.),
                left: Val::Px(10.),
                padding: UiRect::axes(Val::Px(8.), Val::Px(5.)),
                ..default()
            },
            BackgroundColor(BLACK.with_alpha(0.3).into()),
        ))
        .with_children(|builder| {
            builder.spawn((
                Text::new("LAP"),
                text_font
                    .clone()
                    .with_font_size(26.)
                    .with_font(font_bold.clone()),
                TextColor(WHITE.into()),
            ));
            builder
                .spawn((
                    Text::default(),
                    text_font.clone(),
                    TextColor(WHITE.into()),
                ))
                .with_child((TextSpan::default(), text_font.clone(), LapsText))
                .with_child((TextSpan::new("/"), text_font.clone()))
                .with_child((
                    TextSpan::default(),
                    text_font.clone(),
                    TotalLapsText,
                ));
        });
}

fn update_total(
    mut query: Query<&mut TextSpan, With<TotalLapsText>>,
    level_properties: ResMut<LevelProperties>,
) {
    for mut text in &mut query {
        **text = format!("{}", level_properties.laps);
    }
}
fn update(
    mut query: Query<&mut TextSpan, With<LapsText>>,
    trackers: Query<&CheckpointTracker, With<Player>>,
) {
    let tracker = match trackers.get_single() {
        Ok(x) => x,
        Err(_) => return,
    };

    let current = tracker.laps_completed;
    for mut text in &mut query {
        **text = format!("{current}");
    }
}
