use amethyst::{
    core::{math::Vector3, timing::Time, transform::Transform},
    ecs::prelude::{
        Component, DenseVecStorage, Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage,
        System, WriteStorage,
    },
    prelude::*,
};
use rand::seq::SliceRandom;

use crate::{
    enemy::{create_enemy, EnemyType},
    sprite::SpriteSheetMap,
};

#[derive(Component)]
pub struct Spawner {
    speed: f32,
    last_spawn_time: f64,
}

pub struct SpawnerSystem;

impl<'s> System<'s> for SpawnerSystem {
    type SystemData = (
        WriteStorage<'s, Spawner>,
        ReadStorage<'s, Transform>,
        ReadExpect<'s, LazyUpdate>,
        Read<'s, SpriteSheetMap>,
        Read<'s, Time>,
        Entities<'s>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut spawners, transforms, lazy_update, sprite_sheet_map, time, entities) = data;
        for (transform, spawner) in (&transforms, &mut spawners).join() {
            let current_time = time.absolute_time_seconds();
            if spawner.last_spawn_time + (spawner.speed as f64) < current_time {
                let mut rng = rand::thread_rng();
                let enemy_type = [EnemyType::JumpingJelly, EnemyType::SlideySlime]
                    .choose(&mut rng)
                    .unwrap();
                spawner.last_spawn_time = current_time;
                create_enemy(
                    &entities,
                    &lazy_update,
                    sprite_sheet_map.clone(),
                    *transform.translation(),
                    *enemy_type,
                );
            }
        }
    }
}

pub fn create_spawner(world: &mut World, origin: Vector3<f32>) {
    let spawner = Spawner {
        speed: 2.0,
        last_spawn_time: 0.0,
    };

    let mut transform = Transform::default();
    transform.set_translation(origin);

    world.create_entity().with(spawner).with(transform).build();
}
