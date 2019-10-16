use std::collections::HashMap;

use amethyst::{
    assets::{AssetStorage, Loader},
    prelude::*,
    renderer::{sprite::SpriteSheetHandle, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone)]
pub struct SpriteSheetMap {
    sprite_sheets: HashMap<AssetType, SpriteSheetHandle>,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum AssetType {
    Floor,
    JumpingJelly,
    SlideySlime,
}

/** The strings correspond to the names of the `png` and `ron` asset files */
const SPRITE_SHEET_MAPPING: [(AssetType, &str); 3] = [
    (AssetType::Floor, "floor_tiles"),
    (AssetType::SlideySlime, "slidey_slime"),
    (AssetType::JumpingJelly, "jumping_jelly"),
];

impl SpriteSheetMap {
    pub fn new(world: &mut World) -> Self {
        let mut map = HashMap::new();
        for sprite_sheet_tuple in SPRITE_SHEET_MAPPING.iter() {
            map.insert(
                sprite_sheet_tuple.0,
                load_sprite_sheet(world, sprite_sheet_tuple.1),
            );
        }
        SpriteSheetMap { sprite_sheets: map }
    }

    pub fn get(&self, asset_type: AssetType) -> Option<&SpriteSheetHandle> {
        self.sprite_sheets.get(&asset_type)
    }
}

pub fn load_sprite_sheet(world: &mut World, asset_name: &str) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("{}.png", asset_name),
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let spritesheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("{}.ron", asset_name),
        SpriteSheetFormat(texture_handle),
        (),
        &spritesheet_store,
    )
}
