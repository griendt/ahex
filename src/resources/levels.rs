use bevy::ecs::resource::Resource;
use include_dir::{Dir, include_dir};
use serde::Deserialize;

use crate::components::level::Level;

static LEVEL_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/levels");

#[derive(Deserialize, Default, Debug)]
pub enum LevelState {
    #[default]
    WaitingForPlayerInput,
    ProcessingPlayerInput,
    ProcessingLevelEffects,
}

#[derive(Resource, Default)]
pub struct LevelResource {
    pub current_level_identifier: String,
    pub level_state: LevelState,
}

impl LevelResource {
    pub fn get_level(&self) -> Level {
        let level = LEVEL_DIR
            .get_file(format!("{}.toml", self.current_level_identifier).as_str())
            .expect("Level number not found")
            .contents_utf8()
            .unwrap();

        toml::from_str(level).unwrap()
    }
}
