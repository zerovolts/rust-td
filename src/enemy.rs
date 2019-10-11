use amethyst::{
    assets::Handle,
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{
        Component, DenseVecStorage, Entities, Join, ReadExpect, System, Write, WriteStorage,
    },
    prelude::*,
    renderer::{SpriteRender, SpriteSheet},
    ui::UiText,
};

use crate::velocity::Velocity;
use crate::{BuildingMaterials, GameUi};

#[derive(Component, Default)]
pub struct Enemy {
    pub health: i32,
    pub value: i32,
}

pub struct EnemySystem;

impl<'s> System<'s> for EnemySystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Enemy>,
        Write<'s, BuildingMaterials>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, GameUi>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut enemies, mut building_materials, mut ui_text, game_ui) = data;
        for (entity, enemy) in (&entities, &mut enemies).join() {
            if enemy.health <= 0 {
                building_materials.coins += enemy.value;
                if let Some(text) = ui_text.get_mut(game_ui.coin_display) {
                    text.text = format!("Coins: {}", building_materials.coins.to_string());
                }
                let _ = entities.delete(entity);
            }
        }
    }
}

pub fn create_enemy(world: &mut World, sprite_sheet: Handle<SpriteSheet>, origin: Vector3<f32>) {
    let enemy_sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    let mut transform = Transform::default();
    transform.set_translation(origin);

    let velocity = Velocity::new(Vector3::new(0.25, 0.0, 0.0));

    let enemy = Enemy {
        health: 100,
        value: 10,
    };

    world
        .create_entity()
        .with(enemy_sprite)
        .with(transform)
        .with(velocity)
        .with(enemy)
        .build();
}
