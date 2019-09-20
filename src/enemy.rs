use amethyst::{
    prelude::*,
    assets::{AssetStorage, Loader, Handle},
    core::{
        transform::{Transform, TransformBundle},
        math::{Vector3},
    },
    ecs::prelude::{
        System,
        Join,
        Entity,
        Component,
        DenseVecStorage,
    },
    renderer::{
        Camera,
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
        SpriteSheet,
        SpriteSheetFormat,
        ImageFormat,
        SpriteRender,
        Texture,
    },
};

use crate::velocity::{Velocity};

#[derive(Default)]
pub struct Enemy;

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}

pub fn create_enemy(world: &mut World, sprite_sheet: Handle<SpriteSheet>, origin: Vector3<f32>) {
    let enemy_sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 2,
    };

    let mut transform = Transform::default();
    transform.set_translation(origin);

    let velocity = Velocity {
        vector: Vector3::new(0.25, 0.0, 0.0),
    };

    world.create_entity()
        .with(enemy_sprite)
        .with(transform)
        .with(velocity)
        .with(Enemy)
        .build();
}