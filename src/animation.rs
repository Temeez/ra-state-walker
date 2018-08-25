use amethyst::core::cgmath::{Matrix4, Vector3};
use amethyst::core::transform::{GlobalTransform};
use amethyst::assets::{Handle, Loader};
use amethyst::ecs::prelude::{Entity};
use amethyst::prelude::*;
use amethyst::animation::{
    Animation, InterpolationFunction, Sampler, SpriteRenderChannel, SpriteRenderPrimitive,
};
use amethyst::renderer::{
    Camera, Projection, ScreenDimensions, SpriteRender
};

pub fn blue_walker(world: &mut World, sprite_sheet_id: u64) -> Handle<Animation<SpriteRender>> {
    let sprite_indices = (0..6)
        .into_iter()
        .map(|n| SpriteRenderPrimitive::SpriteIndex(n))
        .collect::<Vec<SpriteRenderPrimitive>>();

    let sprite_index_sampler = {
        Sampler {
            input: vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6],
            function: InterpolationFunction::Step,
            output: sprite_indices
        }
    };

    let sprite_sheet_sampler = Sampler {
        input: vec![0.0, 2.3],
        function: InterpolationFunction::Step,
        output: vec![SpriteRenderPrimitive::SpriteSheet(sprite_sheet_id)]
    };

    let loader = world.write_resource::<Loader>();
    let sampler_animation_handle = 
        loader.load_from_data(sprite_index_sampler, (), &world.read_resource());
    let sprite_sheet_sampler_animation_handle = 
        loader.load_from_data(sprite_sheet_sampler, (), &world.read_resource());

    let animation = Animation {
        nodes: vec![
            (
                0,
                SpriteRenderChannel::SpriteSheet,
                sprite_sheet_sampler_animation_handle
            ),
            (
                0,
                SpriteRenderChannel::SpriteIndex,
                sampler_animation_handle
            )
        ]
    };

    loader.load_from_data(animation, (), &world.read_resource())
}

pub fn initialize_camera(world: &mut World) -> Entity {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0, width, height, 0.0
        )))
        .with(GlobalTransform(Matrix4::from_translation(
            Vector3::new(0.0, 0.0, 1.0).into()
        )))
        .build()
}