#[macro_use]
extern crate specs_derive;

mod enemy;
mod projectile;
mod sprite;
mod tower;
mod velocity;

use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{
        ecs::Entity,
        math::Vector3,
        transform::{Transform, TransformBundle},
    },
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        sprite::{SpriteRender, SpriteSheet},
        types::DefaultBackend,
        Camera, RenderingBundle,
    },
    ui::{get_default_font, Anchor, FontAsset, RenderUi, UiBundle, UiImage, UiText, UiTransform},
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

        init_floor_tiles(world, sprite_sheet.clone());
        init_ui(world);
        init_camera(world);

        world.insert(sprite_sheet_map);
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(RenderFlat2D::default())
        .with_plugin(RenderUi::default())
        .with_plugin(
            RenderToWindow::from_config_path(display_config_path)
                .with_clear([0.34, 0.36, 0.52, 1.0]),
        );

    let input_bundle = InputBundle::<StringBindings>::new();

    let game_data = GameDataBuilder::default()
        .with_bundle(rendering_bundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(input_bundle)?
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

#[derive(Default)]
pub struct BuildingMaterials {
    coins: i32,
}

pub struct GameUi {
    pub coin_display: Entity,
}

fn init_ui(world: &mut World) {
    let font_handle = {
        let loader = world.read_resource::<Loader>();
        let font_store = world.read_resource::<AssetStorage<FontAsset>>();
        get_default_font(&loader, &font_store)
    };

    let coin_display_transform = UiTransform::new(
        "coin_display_transform".to_string(),
        Anchor::TopLeft,
        Anchor::TopLeft,
        0.,
        0.,
        1.,
        200.,
        50.,
    );

    let coin_display_text = UiText::new(
        font_handle.clone(),
        "Coins: 0".to_string(),
        [1., 1., 1., 1.],
        20.,
    );

    let coin_display_background = UiImage::SolidColor([0.2, 0.2, 0.2, 0.5]);

    let coin_display = world
        .create_entity()
        .with(coin_display_transform)
        .with(coin_display_text)
        .with(coin_display_background)
        .build();

    world.insert(GameUi { coin_display });
}

fn init_floor_tiles(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let x_tile_count = (SCREEN_WIDTH / 16.0) as i32;
    let y_tile_count = (SCREEN_HEIGHT / 16.0) as i32;
    for x in 0..x_tile_count {
        for y in 0..y_tile_count {
            let sprite_render = SpriteRender {
                sprite_sheet: sprite_sheet.clone(),
                sprite_number: 4,
            };

            let mut transform = Transform::default();
            transform.set_translation_xyz(((x as f32) * 16.) + 8., ((y as f32) * 16.) + 8., -1.);

            world
                .create_entity()
                .with(transform)
                .with(sprite_render)
                .build();
        }
    }
}
