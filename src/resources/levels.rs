use bevy::ecs::resource::Resource;
use include_dir::{Dir, include_dir};

use crate::components::level::LevelParser;

static LEVEL_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/levels");

#[derive(Resource, Default)]
pub struct LevelResource {
    pub current_level_number: isize,
}

impl LevelResource {
    pub fn get_level(&self) -> LevelParser {
        let level = LEVEL_DIR
            .get_file(format!("{}.toml", self.current_level_number).as_str())
            .expect("Level number not found")
            .contents_utf8()
            .unwrap();

        toml::from_str(level).unwrap()
    }
}
