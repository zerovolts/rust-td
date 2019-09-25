use amethyst::{
    assets::Handle,
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{Component, DenseVecStorage, Entities, Join, System, WriteStorage},
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
};

use crate::velocity::Velocity;

#[derive(Component, Default)]
pub struct Enemy {
    pub health: i32,
}

pub struct EnemySystem;

impl<'s> System<'s> for EnemySystem {
    type SystemData = (Entities<'s>, WriteStorage<'s, Enemy>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut enemies) = data;
        for (entity, enemy) in (&entities, &mut enemies).join() {
            if enemy.health <= 0 {
                let _ = entities.delete(entity);
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

    let velocity = Velocity::new(Vector3::new(0.25, 0.0, 0.0));

    let enemy = Enemy { health: 100 };

    world
        .create_entity()
        .with(enemy_sprite)
        .with(transform)
        .with(velocity)
        .with(enemy)
        .build();
}
