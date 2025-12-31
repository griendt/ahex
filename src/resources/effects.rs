use bevy::{asset::Handle, ecs::resource::Resource};
use bevy_hanabi::EffectAsset;

#[derive(Resource, Default)]
pub struct GlobalEffects {
    pub goal_explosion_effect: Option<Handle<EffectAsset>>,
}
