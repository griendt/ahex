use bevy::{
    asset::{AssetServer, Assets},
    camera::Camera3d,
    core_pipeline::tonemapping::Tonemapping,
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, Query, ResMut},
    },
    light::NotShadowReceiver,
    math::{Vec2, Vec3, Vec4},
    mesh::{Mesh, MeshBuilder, SphereKind, SphereMeshBuilder},
    post_process::bloom::{Bloom, BloomCompositeMode, BloomPrefilter},
    transform::components::Transform,
    utils::default,
};
use bevy_hanabi::{
    AccelModifier, Attribute, ColorOverLifetimeModifier, EffectAsset, Gradient, Module,
    SetAttributeModifier, SetPositionSphereModifier, SetVelocitySphereModifier, ShapeDimension,
    SpawnerSettings,
};
use bevy_water::WaterTile;

use crate::{components::camera::CameraAngle, resources::effects::GlobalEffects};

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        CameraAngle {
            rotation_speed: 3.0,
            ..default()
        },
        Transform::from_xyz(0.0, 16.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
        Tonemapping::TonyMcMapface,
        Bloom {
            intensity: 0.05,
            low_frequency_boost: 0.95,
            low_frequency_boost_curvature: 0.5,
            high_pass_frequency: 0.5,
            prefilter: BloomPrefilter {
                threshold: 0.0,
                threshold_softness: 1.0,
            },
            composite_mode: BloomCompositeMode::Additive,
            max_mip_dimension: 512,
            scale: Vec2::ONE,
        },
    ));
}

pub fn setup_effects(
    mut effects: ResMut<GlobalEffects>,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: ResMut<AssetServer>,
) {
    // Define a color gradient from red to transparent black
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1., 1., 0., 1.));
    gradient.add_key(1.0, Vec4::new(0., 0., 0., 0.));

    // Create a new expression module
    let mut module = Module::default();

    // On spawn, randomly initialize the position of the particle
    // to be over the surface of a sphere of radius 2 units.
    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(0.5),
        dimension: ShapeDimension::Surface,
    };

    // Also initialize a radial initial velocity to 6 units/sec
    // away from the (same) sphere center.
    let init_vel = SetVelocitySphereModifier {
        center: module.lit(Vec3::ZERO),
        speed: module.lit(4.),
    };

    // Initialize the total lifetime of the particle, that is
    // the time for which it's simulated and rendered. This modifier
    // is almost always required, otherwise the particles won't show.
    let lifetime = module.lit(5.); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_size_attr = SetAttributeModifier {
        attribute: Attribute::SIZE,
        value: module.lit(0.1),
    };

    // Every frame, add a gravity-like acceleration downward
    let accel = module.lit(Vec3::new(0., -2., 0.));
    let update_accel = AccelModifier::new(accel);

    let mesh = meshes.add(SphereMeshBuilder::new(0.2, SphereKind::Ico { subdivisions: 1 }).build());

    // Create the effect asset
    let effect = EffectAsset::new(
        // Maximum number of particles alive at a time
        32768,
        // Spawn at a rate of 5 particles per second
        SpawnerSettings::once(1000.0.into()),
        // Move the expression module into the asset
        module,
    )
    .with_name("GoalCollectEffect")
    .init(init_pos)
    .init(init_vel)
    .init(init_size_attr)
    .init(init_lifetime)
    .update(update_accel)
    .mesh(mesh)
    // Render the particles with a color gradient over their
    // lifetime. This maps the gradient key 0 to the particle spawn
    // time, and the gradient key 1 to the particle death (10s).
    .render(ColorOverLifetimeModifier {
        gradient,
        ..default()
    });

    // Insert into the asset system and save a handle for later use
    effects.goal_explosion_effect = Some(assets.add(effect));
}

pub fn enable_water_shadows(
    mut commands: Commands,
    query: Query<(&WaterTile, Entity), With<NotShadowReceiver>>,
) {
    for (_water_tile, entity) in query {
        commands.entity(entity).remove::<NotShadowReceiver>();
    }
}
