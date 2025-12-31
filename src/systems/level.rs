use bevy::prelude::*;

use crate::components::{
    goal::Goal,
    level::{LevelCompleteTextMarker, LevelEntityMarker, LevelParser},
};

pub fn build_level(
    mut commands: Commands,
    level: Single<&LevelParser>,
    asset_server: Res<AssetServer>,
) {
    level.render_level(&mut commands, &asset_server);
}

pub fn restart_level(
    mut commands: Commands,
    level: Single<&LevelParser>,
    entities: Query<(&LevelEntityMarker, Entity), Without<LevelParser>>,
    asset_server: Res<AssetServer>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Backspace) {
        for entity in entities {
            commands.entity(entity.1).despawn();
        }

        level.render_level(&mut commands, &asset_server);
    }
}

pub fn show_level_complete(
    mut commands: Commands,
    goals: Query<&Goal>,
    asset_server: Res<AssetServer>,
) {
    if !goals.is_empty() {
        return;
    }

    commands
        .spawn((Node {
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            overflow: Overflow::visible(),
            max_width: Val::Px(0.0),
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            ..default()
        },))
        .with_children(|builder| {
            builder.spawn((
                LevelCompleteTextMarker,
                Text::new("Level Complete!"),
                TextFont {
                    font: asset_server.load("fonts/main.ttf"),
                    font_size: 60.0,
                    ..default()
                },
                TextShadow::default(),
                TextLayout::new_with_justify(Justify::Center).with_no_wrap(),
                TextColor::from(LinearRgba::rgb(1.0, 1.0, 0.0)),
            ));
        });
}
