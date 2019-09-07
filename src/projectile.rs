use amethyst::{
    prelude::*,
    assets::{Handle},
    core::{
        transform::{Transform},
        math::{Vector3},
    },
    ecs::prelude::{
        LazyUpdate,
        // Storage,
        World,
        Component,
        DenseVecStorage,
        Entities,
        ReadExpect,
    },
    renderer::{
        SpriteSheet,
        SpriteRender,
    },
};

use crate::velocity::Velocity;

pub struct Projectile {
    effect: ProjectileEffect,
}

impl Component for Projectile {
    type Storage = DenseVecStorage<Self>;

}

enum TimingFunction {
    Linear, // bullet
    EaseIn, // rocket thruster
    EaseOut,
    EaseInOut, // trebuchet
}

enum ProjectileEffect {
    Damage(i32),
    DoT(i32, i32),
    Slow(i32, i32),
    Stun(i32),
    Area(i32),
    Easing(TimingFunction),
}

pub fn create_projectile(
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,

    sprite_sheet: Handle<SpriteSheet>,
    origin: Vector3<f32>,
    target: Vector3<f32>,
    speed: f32
) {
    let projectile_sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 1,
    };

    let mut transform = Transform::default();
    transform.set_translation(origin);

    let mut velocity = Velocity {
        vector: (target - origin).normalize(),
    };

    let entity = entities.create();
    lazy_update.insert(entity, projectile_sprite);
    lazy_update.insert(entity, transform);
    lazy_update.insert(entity, velocity);
}
