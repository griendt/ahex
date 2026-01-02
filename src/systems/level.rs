use bevy::prelude::*;

use crate::{
    components::{
        goal::Goal,
        level::{LevelCompleteTextMarker, LevelEntityMarker, LevelParser},
    },
    resources::levels::LevelResource,
};

pub fn build_level(
    mut commands: Commands,
    levels: Res<LevelResource>,
    asset_server: Res<AssetServer>,
) {
    levels
        .get_level()
        .render_level(&mut commands, &asset_server);
}

pub fn restart_level(
    mut commands: Commands,
    levels: Res<LevelResource>,
    entities: Query<(&LevelEntityMarker, Entity), Without<LevelParser>>,
    asset_server: Res<AssetServer>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Backspace) {
        for entity in entities {
            commands.entity(entity.1).despawn();
        }

        levels
            .get_level()
            .render_level(&mut commands, &asset_server);
    }
}

pub fn go_to_next_level(
    mut commands: Commands,
    mut levels: ResMut<LevelResource>,
    goals: Query<&Goal>,
    entities: Query<(&LevelEntityMarker, Entity)>,
    asset_server: Res<AssetServer>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    // TODO: replace with more generic check for whether forwarding to next level is allowed
    if !goals.is_empty() {
        return;
    }

    if keys.just_pressed(KeyCode::Enter) {
        // Destroy current level
        for entity in entities {
            commands.entity(entity.1).despawn();
        }

        // Render new level
        levels.current_level_number += 1;
        levels
            .get_level()
            .render_level(&mut commands, &asset_server);
    }
}

pub fn show_level_complete(
    mut commands: Commands,
    goals: Query<&Goal>,
    level_complete_marker: Option<Single<&LevelCompleteTextMarker>>,
    asset_server: Res<AssetServer>,
) {
    if !goals.is_empty() {
        return;
    }

    if level_complete_marker.is_some() {
        return;
    }

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                overflow: Overflow::visible(),
                max_width: Val::Px(0.0),
                left: Val::Percent(50.0),
                top: Val::Percent(70.0),
                ..default()
            },
            LevelEntityMarker,
        ))
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
                LevelEntityMarker,
            ));
        });
}
