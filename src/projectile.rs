use amethyst::{
    prelude::*,
    shrev::EventChannel,
    assets::{Handle},
    core::{
        transform::{Transform},
        math::{Vector3},
    },
    ecs::prelude::{
        LazyUpdate,
        World,
        Component,
        DenseVecStorage,
        Entities,
        ReadExpect,
        System,
        Write,
        Entity,
        ReadStorage,
        WriteStorage,
        Join,
    },
    renderer::{
        SpriteSheet,
        SpriteRender,
    },
};

use crate::velocity::Velocity;
use crate::enemy::Enemy;
// use crate::bounding_box::BoundingBox;

pub struct CollisionEvent {
    entity_a: Entity,
    entity_b: Entity,
}

pub struct Projectile {
    // TODO: make this a vec
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

pub struct ProjectileSystem;

impl<'s> System<'s> for ProjectileSystem {
    type SystemData = (
        // Write<'s, EventChannel<CollisionEvent>>,
        ReadStorage<'s, Projectile>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Enemy>,
        Entities<'s>,
    );

    fn run(&mut self, data: Self::SystemData) {
        // let (channel) = data;
        let (projectiles, transforms, mut enemies, entities) = data;
        for (projectile_entity, projectile, projectile_transform) in (&entities, &projectiles, &transforms).join() {
            for (enemy, enemy_transform) in (&mut enemies, &transforms).join() {
                let distance_vector = (projectile_transform.translation() - enemy_transform.translation());
                let distance_sq = (distance_vector.x * distance_vector.x) + (distance_vector.y * distance_vector.y);
                if (distance_sq < 100.0) {
                    match projectile.effect {
                        ProjectileEffect::Damage(damage) => enemy.health -= damage,
                        _ => {},
                    }
                    entities.delete(projectile_entity);
                }
            }
        }
    }
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

    let projectile = Projectile {
        effect: ProjectileEffect::Damage(10),
    };

    let entity = entities.create();
    lazy_update.insert(entity, projectile_sprite);
    lazy_update.insert(entity, transform);
    lazy_update.insert(entity, velocity);
    lazy_update.insert(entity, projectile);
}
