#[macro_use]
extern crate specs_derive;

mod enemy;
mod projectile;
mod sprite;
mod tower;
mod velocity;

use amethyst::{
    core::{
        math::Vector3,
        transform::{Transform, TransformBundle},
    },
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        Camera, RenderingBundle,
    },
    utils::application_root_dir,
};

use crate::{
    enemy::{create_enemy, EnemySystem},
    projectile::ProjectileSystem,
    sprite::{AssetType, SpriteSheetMap},
    tower::{create_tower, TowerSystem},
    velocity::VelocitySystem,
};

struct GameplayState;

impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_map = SpriteSheetMap::new(world);
        let sprite_sheet = sprite_sheet_map.get(AssetType::Floor).unwrap();

        for i in 0..6 {
            create_enemy(
                world,
                sprite_sheet.clone(),
                Vector3::new((i as f32) * -32.0, 160.0, 0.0),
            );
        }

        for i in 0..6 {
            let tower_pos = Vector3::new(64.0 + ((i as f32) * 32.0), 128.0, 0.0);
            create_tower(world, sprite_sheet.clone(), tower_pos);
        }

        world.add_resource(sprite_sheet_map);
        init_camera(world);
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with(VelocitySystem, "velocity_system", &[])
        .with(TowerSystem, "tower_system", &[])
        .with(EnemySystem, "enemy_system", &[])
        .with(ProjectileSystem, "projectile_system", &[]);

    let mut game = Application::new("assets/", GameplayState, game_data)?;
    game.run();

    Ok(())
}

pub const SCREEN_WIDTH: f32 = 320.0;
pub const SCREEN_HEIGHT: f32 = 240.0;

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(SCREEN_WIDTH * 0.5, SCREEN_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(SCREEN_WIDTH, SCREEN_HEIGHT))
        .with(transform)
        .build();
}
