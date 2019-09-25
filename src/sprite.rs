use std::collections::HashMap;

use amethyst::{
    assets::{AssetStorage, Loader},
    prelude::*,
    renderer::{sprite::SpriteSheetHandle, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
};

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub enum AssetType {
    Floor,
}

#[derive(Default)]
pub struct SpriteSheetMap {
    sprite_sheets: HashMap<AssetType, SpriteSheetHandle>,
}

impl SpriteSheetMap {
    pub fn new(world: &mut World) -> Self {
        let mut map = HashMap::new();
        map.insert(AssetType::Floor, load_spritesheet(world));
        SpriteSheetMap { sprite_sheets: map }
    }

    pub fn get(&self, asset_type: AssetType) -> Option<&SpriteSheetHandle> {
        self.sprite_sheets.get(&asset_type)
    }
}

pub fn load_spritesheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "floor_tiles.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let spritesheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "floor_tiles.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &spritesheet_store,
    )
}
