use std::f32::INFINITY;

use amethyst::{
    prelude::*,
    assets::{AssetStorage, Loader, Handle},
    core::{
        timing::Time,
        transform::{Transform, TransformBundle},
        math::{Vector3},
    },
    ecs::prelude::{
        LazyUpdate,
        System,
        Join,
        Read,
        ReadStorage,
        WriteStorage,
        Component,
        DenseVecStorage,
        Entities,
        ReadExpect,
        Entity,
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
    utils::application_root_dir,
};

use crate::{
    velocity::Velocity,
    projectile::create_projectile,
    sprite::{SpriteSheetMap, AssetType},
    enemy::Enemy,
};

pub struct Tower {
    speed: f32,
    range: f32,
    last_fire_time: f64,
    target: Option<Entity>,
}

impl Component for Tower {
    type Storage = DenseVecStorage<Self>;
}

pub struct TowerSystem;

impl<'s> System<'s> for TowerSystem {
    type SystemData = (
        WriteStorage<'s, Tower>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Enemy>,
        ReadExpect<'s, LazyUpdate>,
        Read<'s, SpriteSheetMap>,
        Read<'s, Time>,
        Entities<'s>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut towers,
            transforms,
            enemies,
            lazy_update,
            sprite_sheet_map,
            time,
            entities
        ) = data;
        let sprite_sheet = sprite_sheet_map.get(AssetType::Floor).unwrap();

        for (transform, tower) in (&transforms, &mut towers).join() {
            if let Some(enemy) = tower.target {
                let enemy_transform = transforms.get(enemy).cloned().unwrap();
                // If the target is too far away, stop targeting it
                if !in_range(transform.translation(), enemy_transform.translation(), tower.range) {
                    tower.target = None;
                    continue;
                }
                // Projectile firing
                let current_time = time.absolute_time_seconds();
                if tower.last_fire_time + (tower.speed as f64) < current_time {
                    tower.last_fire_time = current_time;
                    create_projectile(&entities, &lazy_update, sprite_sheet.clone(), *transform.translation(), *enemy_transform.translation(), 1.0);
                }
            }
            else { 
                // Iterate over enemies and set closest one as target
                let mut closest_enemy: Option<Entity> = None;
                let mut closest_len = INFINITY;
                for (entity, enemy, enemy_transform) in (&entities, &enemies, &transforms).join() {
                    let len_sq = len_sq(&(enemy_transform.translation() - transform.translation()));
                    if in_range(transform.translation(), enemy_transform.translation(), tower.range) {
                        if len_sq < closest_len {
                            closest_enemy = Some(entity);
                            closest_len = len_sq;
                        }
                    }
                }
                tower.target = closest_enemy;
            }
        }
    }
}

fn len_sq(v: &Vector3<f32>) -> f32 {
    v.x * v.x + v.y * v.y
}

fn in_range(tower: &Vector3<f32>, enemy: &Vector3<f32>, range: f32) -> bool {
    len_sq(&(enemy - tower)) < (range * range)
}

pub fn create_tower(world: &mut World, sprite_sheet: Handle<SpriteSheet>, position: Vector3<f32>) {
    let mut transform = Transform::default();
    transform.set_translation(position);

    let tower = Tower {
        speed: 1.0,
        range: 100.0,
        last_fire_time: 0.0,
        target: None,
    };

    let grass_sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    world.create_entity()
        .with(grass_sprite.clone())
        .with(tower)
        .with(transform.clone())
        .build();
}