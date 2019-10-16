#[macro_use]
extern crate specs_derive;

mod enemy;
mod projectile;
mod spawner;
mod sprite;
mod tile_map;
mod tower;
mod velocity;

use std::time::Duration;

use amethyst::{
    animation::{
        get_animation_set, AnimationBundle, AnimationCommand, AnimationControlSet, AnimationSet,
        AnimationSetPrefab, EndControl,
    },
    assets::{
        AssetStorage, Handle, Loader, PrefabData, PrefabLoader, PrefabLoaderSystemDesc,
        ProgressCounter, RonFormat,
    },
    core::{
        ecs::Entity,
        frame_limiter::FrameRateLimitStrategy,
        math::Vector3,
        transform::{Transform, TransformBundle},
    },
    derive::PrefabData,
    ecs::{Entities, Join, ReadStorage, WriteStorage},
    error::Error,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        sprite::{prefab::SpriteScenePrefab, SpriteRender, SpriteSheet},
        types::DefaultBackend,
        Camera, RenderingBundle,
    },
    ui::{get_default_font, Anchor, FontAsset, RenderUi, UiBundle, UiImage, UiText, UiTransform},
    utils::application_root_dir,
};
use rand::Rng;
use serde::Deserialize;

use crate::{
    enemy::EnemySystem,
    projectile::ProjectileSystem,
    spawner::{create_spawner, SpawnerSystem},
    sprite::{AssetType, SpriteSheetMap},
    tile_map::{generate_map, TileMap, TileType},
    tower::{create_tower, TowerSystem},
    velocity::VelocitySystem,
};

const MIN_PATH_LENGTH: usize = 80;

#[derive(Default)]
struct GameplayState {
    // A progress tracker to check that assets are loaded
    pub progress_counter: Option<ProgressCounter>,
}

// Loading data for one entity
#[derive(Debug, Clone, Deserialize, PrefabData)]
struct JumpingJellyPrefab {
    // Information for rendering a scene with sprites
    sprite_scene: SpriteScenePrefab,
    // Аll animations that can be run on the entity
    animation_set: AnimationSetPrefab<AssetType, SpriteRender>,
}

impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        /* Generate map */
        let sprite_sheet_map = SpriteSheetMap::new(world);
        let floor_tiles = sprite_sheet_map.get(AssetType::Floor).unwrap();

        let x_tile_count = (SCREEN_WIDTH / 16.0) as i32;
        let y_tile_count = (SCREEN_HEIGHT / 16.0) as i32;

        // Reroll the map until the path is at least 80 tiles long
        let (tile_map, enemy_path) = {
            let mut tile_map_tuple = generate_map(x_tile_count, y_tile_count);
            while tile_map_tuple.1.path.len() < MIN_PATH_LENGTH {
                tile_map_tuple = generate_map(x_tile_count, y_tile_count);
            }
            tile_map_tuple
        };

        /* Enemy Spawner */
        let spawner_position = Vector3::new(
            (enemy_path.starting_coord.0 as f32) * 16.0 + 8.0,
            (enemy_path.starting_coord.1 as f32) * 16.0 + 8.0,
            0.0,
        );
        create_spawner(world, spawner_position);

        let mut rng = rand::thread_rng();
        for _ in 0..16 {
            let x = rng.gen::<i32>().abs() % x_tile_count;
            let y = rng.gen::<i32>().abs() % y_tile_count;
            println!("{}, {}", x, y);
            if tile_map.get((x, y)) != Some(TileType::Rock) {
                let tower_pos = Vector3::new((x as f32) * 16.0 + 8.0, (y as f32) * 16.0 + 8.0, 0.0);
                create_tower(world, floor_tiles.clone(), tower_pos);
            }
        }

        /* Initialize animated entities */
        // Create new progress counter
        self.progress_counter = Some(Default::default());
        // Starts asset loading
        let jumping_jelly_prefab = world.exec(|loader: PrefabLoader<'_, JumpingJellyPrefab>| {
            loader.load(
                "prefabs/jumping_jelly.ron",
                RonFormat,
                self.progress_counter.as_mut().unwrap(),
            )
        });

        init_floor_tiles(world, floor_tiles.clone(), &tile_map);
        init_ui(world);
        init_camera(world);

        world.insert(tile_map);
        world.insert(sprite_sheet_map);
        world.create_entity().with(jumping_jelly_prefab).build();
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // Checks if we are still loading data
        if let Some(ref progress_counter) = self.progress_counter {
            // Checks progress
            if progress_counter.is_complete() {
                let StateData { world, .. } = data;
                // Execute a pass similar to a system
                world.exec(
                    |(entities, animation_sets, mut control_sets): (
                        Entities,
                        ReadStorage<AnimationSet<AssetType, SpriteRender>>,
                        WriteStorage<AnimationControlSet<AssetType, SpriteRender>>,
                    )| {
                        // For each entity that has AnimationSet
                        for (entity, animation_set) in (&entities, &animation_sets).join() {
                            // Creates a new AnimationControlSet for the entity
                            let control_set = get_animation_set(&mut control_sets, entity).unwrap();
                            // Adds the `JumpingJelly` animation to AnimationControlSet and loops infinitely
                            control_set.add_animation(
                                AssetType::JumpingJelly,
                                &animation_set.get(&AssetType::JumpingJelly).unwrap(),
                                EndControl::Loop(None),
                                1.0,
                                AnimationCommand::Start,
                            );
                        }
                    },
                );
                // All data loaded
                self.progress_counter = None;
            }
        }
        Trans::None
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
        .with_system_desc(
            PrefabLoaderSystemDesc::<JumpingJellyPrefab>::default(),
            "scene_loader",
            &[],
        )
        .with_bundle(AnimationBundle::<AssetType, SpriteRender>::new(
            "sprite_animation_control",
            "sprite_sampler_interpolation",
        ))?
        .with_bundle(rendering_bundle)?
        .with_bundle(
            TransformBundle::new()
                .with_dep(&["sprite_animation_control", "sprite_sampler_interpolation"]),
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(input_bundle)?
        .with(VelocitySystem, "velocity_system", &[])
        .with(TowerSystem, "tower_system", &[])
        .with(EnemySystem, "enemy_system", &[])
        .with(ProjectileSystem, "projectile_system", &[])
        .with(SpawnerSystem, "spawner_system", &[]);

    let mut game = Application::build("assets/", GameplayState::default())?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            60,
        )
        .build(game_data)?;
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

fn init_floor_tiles(world: &mut World, sprite_sheet: Handle<SpriteSheet>, tile_map: &TileMap) {
    for x in 0..tile_map.width {
        for y in 0..tile_map.height {
            let tile = tile_map
                .get((x, y))
                .expect(format!("No tile at location ({}, {})", x, y).as_str());
            let sprite_number = match tile {
                TileType::Grass => 4,
                TileType::Rock => 2,
            };
            let sprite_render = SpriteRender {
                sprite_sheet: sprite_sheet.clone(),
                sprite_number: sprite_number,
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
