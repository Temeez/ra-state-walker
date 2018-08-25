use amethyst::assets::{AssetStorage, Loader};
use amethyst::prelude::*;
use amethyst::renderer::{PngFormat, Texture, TextureHandle};

// https://github.com/amethyst/amethyst/blob/e99885926057e37e62dd27e88797a14e739ad136/examples/sprites/png_loader.rs
pub fn load<N>(name: N, world: &World) -> TextureHandle
where
    N: Into<String>,
{
    let loader = world.read_resource::<Loader>();
    loader.load(
        name,
        PngFormat,
        Default::default(),
        (),
        &world.read_resource::<AssetStorage<Texture>>()
    )
}

// https://github.com/amethyst/amethyst/blob/e99885926057e37e62dd27e88797a14e739ad136/examples/sprites/sprite.rs
#[derive(Debug)]
pub struct SpriteSheetDefinition {
    pub sprite_w: f32,
    pub sprite_h: f32,
    pub row_count: usize,
    pub column_count: usize,
    pub has_border: bool
}

impl SpriteSheetDefinition {
    pub fn new(
        sprite_w: f32,
        sprite_h: f32,
        row_count: usize,
        column_count: usize,
        has_border: bool
    ) -> Self {
        SpriteSheetDefinition {
            sprite_w,
            sprite_h,
            row_count,
            column_count,
            has_border
        }
    }
}