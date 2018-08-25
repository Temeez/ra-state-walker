// https://github.com/amethyst/amethyst/blob/e99885926057e37e62dd27e88797a14e739ad136/examples/sprites/sprite_sheet_loader.rs

use amethyst::renderer::{Sprite, SpriteSheet, TextureCoordinates};

use sprite;

pub fn load(texture_id: u64, definition: &sprite::SpriteSheetDefinition) -> SpriteSheet {
    let mut sprites = Vec::with_capacity(definition.row_count * definition.column_count);
    let (offset_w, offset_h) = offset_distances(&definition);
    let (image_w, image_h) = (
        offset_w * definition.column_count as f32,
        offset_h * definition.row_count as f32
    );

    for row in 0..definition.row_count {
        for col in 0..definition.column_count {
            let offset_x = offset_w * col as f32;
            let offset_y = offset_h * row as f32;
            let sprite = create_sprite(
                image_w,
                image_h,
                definition.sprite_w,
                definition.sprite_h,
                offset_x,
                offset_y
            );

            let sprite_number = row * definition.column_count + col;
            println!("{}: Sprite: {:?}", sprite_number, &sprite);

            sprites.push(sprite);
        }
    }

    SpriteSheet {
        texture_id,
        sprites
    }
}

fn offset_distances(definition: &sprite::SpriteSheetDefinition) -> (f32, f32) {
    if definition.has_border {
        (definition.sprite_w +1.0, definition.sprite_h +1.0)
    } else {
        (definition.sprite_w, definition.sprite_h)
    }
}

fn create_sprite(
    image_w: f32,
    image_h: f32,
    sprite_w: f32,
    sprite_h: f32,
    pixel_left: f32,
    pixel_top: f32
) -> Sprite {
    let pixel_right = pixel_left + sprite_w;
    let pixel_bottom = pixel_top + sprite_h;

    let left = pixel_left / image_w;
    let right = pixel_right / image_w;
    let top = 1.0 - pixel_top / image_h;
    let bottom = 1.0 - pixel_bottom / image_h;

    let tex_coords = TextureCoordinates {
        left, right, bottom, top
    };

    Sprite {
        width: sprite_w,
        height: sprite_h,
        offsets: [sprite_w / 2.0, sprite_h / 2.0],
        tex_coords
    }
}