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
};

pub struct Tower {
    cost: i32,
    speed: f32,
    rotation_speed: f32,
    last_fire_time: f64,
}

impl Component for Tower {
    type Storage = DenseVecStorage<Self>;
}

pub struct TowerSystem;

impl<'s> System<'s> for TowerSystem {
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
        ReadExpect<'s, LazyUpdate>,
        Read<'s, SpriteSheetMap>,
        // ReadStorage<'s, Transform>,
        WriteStorage<'s, Tower>,
        Read<'s, Time>,
        Entities<'s>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut sprite_renders, mut transforms, mut velocities, lazy_update, sprite_sheet_map, mut towers, time, entities) = data;
        let sprite_sheet = sprite_sheet_map.get(AssetType::Floor).unwrap();
        for (transform, tower) in (&transforms, &mut towers).join() {
            let current_time = time.absolute_time_seconds();
            if tower.last_fire_time + 1.0 < current_time {
                tower.last_fire_time = current_time;
                create_projectile(&entities, &lazy_update, sprite_sheet.clone(), *transform.translation(), Vector3::new(0.0, 200.0, 0.0), 1.0);
            }
        }
    }
}

pub fn create_tower(world: &mut World, sprite_sheet: Handle<SpriteSheet>, position: Vector3<f32>) {
    let mut transform = Transform::default();
    transform.set_translation(position);

    let tower = Tower {
        cost: 0,
        speed: 1.0,
        rotation_speed: 0.0,
        last_fire_time: 0.0,
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