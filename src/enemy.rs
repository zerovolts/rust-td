use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{
        Component, DenseVecStorage, Entities, Join, LazyUpdate, ReadExpect, System, Write,
        WriteStorage,
    },
    renderer::SpriteRender,
    ui::UiText,
};

use crate::{
    sprite::{AssetType, SpriteSheetMap},
    velocity::Velocity,
    BuildingMaterials, GameUi,
};

#[derive(Component)]
pub struct Enemy {
    pub health: i32,
    pub value: i32,
    pub enemy_type: EnemyType,
}

#[derive(Clone, Copy)]
pub enum EnemyType {
    JumpingJelly,
    SlideySlime,
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

pub fn create_enemy(
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
    sprite_sheet_map: SpriteSheetMap,
    origin: Vector3<f32>,
    enemy_type: EnemyType,
) {
    let asset_type = match enemy_type {
        EnemyType::JumpingJelly => AssetType::JumpingJelly,
        EnemyType::SlideySlime => AssetType::SlideySlime,
    };
    let enemy_sprite = SpriteRender {
        sprite_sheet: sprite_sheet_map.get(asset_type).unwrap().clone(),
        sprite_number: 0,
    };

    let mut transform = Transform::default();
    transform.set_translation(origin);

    let velocity = Velocity::new(Vector3::new(0.25, 0.0, 0.0));

    let enemy = Enemy {
        health: 100,
        value: 10,
        enemy_type,
    };

    let entity = entities.create();
    lazy_update.insert(entity, enemy_sprite);
    lazy_update.insert(entity, transform);
    lazy_update.insert(entity, velocity);
    lazy_update.insert(entity, enemy);
}
