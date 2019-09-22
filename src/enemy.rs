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
        Entities,
        Component,
        DenseVecStorage,
        WriteStorage,
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
pub struct Enemy {
    pub health: i32,
}

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}

pub struct EnemySystem;

impl<'s> System<'s> for EnemySystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Enemy>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut enemies) = data;
        for (entity, enemy) in (&entities, &mut enemies).join() {
            if enemy.health <= 0 {
                entities.delete(entity);
            }
        }
    }
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

    let enemy = Enemy {
        health: 100,
    };

    world.create_entity()
        .with(enemy_sprite)
        .with(transform)
        .with(velocity)
        .with(enemy)
        .build();
}