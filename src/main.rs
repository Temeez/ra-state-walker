extern crate amethyst;

mod animation;
mod sprite;
mod sprite_sheet_loader;

use std::time::Duration;

use amethyst::core::cgmath::{Point3, Transform as CgTransform, Vector3};
use amethyst::core::transform::{GlobalTransform, Transform, TransformBundle};
use amethyst::assets::{AssetStorage, Loader};
use amethyst::ecs::prelude::Entity;
use amethyst::prelude::*;
use amethyst::input::{is_close_requested, is_key_down, InputBundle};
use amethyst::ui::{UiBundle, DrawUi, Anchor, TtfFormat, UiText, UiTransform};
use amethyst::animation::{
    get_animation_set, AnimationBundle, AnimationCommand, AnimationControl, ControlState,
    EndControl
};
use amethyst::renderer::{
    ColorMask, DisplayConfig, DrawSprite, Event, MaterialTextureSet, Pipeline,
    RenderBundle, ScreenDimensions, SpriteRender, SpriteSheet, SpriteSheetHandle, SpriteSheetSet,
    Stage, VirtualKeyCode, ALPHA
};

mod components;
mod systems;
mod pauser;

use components::*;
use pauser::{CustomGameData, CustomGameDataBuilder};
use sprite::SpriteSheetDefinition;

#[derive(Debug)]
struct GameplayState {
    player: Option<Entity>
}

#[derive(Debug)]
struct PausedState;

pub struct GameStateText {
    pub text: Entity
}

impl GameplayState {
    fn new() -> GameplayState {
        GameplayState {
            player: None
        }
    }

    fn toggle_player_sprite_animation(&mut self, world: &mut World) {
        let mut animation_contorl_set_storage = world.write_storage();
        let player_entity = self.player.unwrap();
        let animation_set = 
            get_animation_set::<u32, SpriteRender>(&mut animation_contorl_set_storage, player_entity)
                .unwrap();
        animation_set.toggle(0);
    }

    fn draw_sprites_animated(
        &mut self,
        world: &mut World,
        common_transform: &Transform,
        sprite_sheet_handle: SpriteSheetHandle,
        sprite_sheet_index: u64,
        sprite_count: usize,
        sprite_w: f32,
        sprite_h: f32
    ) {
        let blue_walker_animation = animation::blue_walker(world, sprite_sheet_index);

        let mut sprite_transform = Transform::default();
        sprite_transform.translation = Vector3::new(sprite_w, sprite_h * 2.5, 0.0);

        CgTransform::<Point3<f32>>::concat_self(&mut sprite_transform, &common_transform);

        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_handle.clone(),
            sprite_number: sprite_count,
            flip_horizontal: false,
            flip_vertical: false
        };

        println!("sprite_render: `{:?}`", sprite_render);

        let player_entity = world
            .create_entity()
            .with(sprite_render)
            .with(sprite_transform)
            .with(GlobalTransform::default())
            .with(components::PlayerComponent::default())
            .build();

        let animation = blue_walker_animation.clone();
        let mut animation_contorl_set_storage = world.write_storage();
        let animation_set = 
            get_animation_set::<u32, SpriteRender>(&mut animation_contorl_set_storage, player_entity)
                .unwrap();

        let animation_id = 0;

        let animation_contorl = AnimationControl::new(
            animation,
            EndControl::Loop(None),
            ControlState::Deferred(Duration::from_millis(200)),
            AnimationCommand::Start,
            1.0
        );

        animation_set.insert(animation_id, animation_contorl);

        self.player = Some(player_entity);
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b>> for GameplayState {
    fn on_start(&mut self, data: StateData<CustomGameData>) {
        // Pull the `World` instance from the `StateData`
        let StateData { world, .. } = data;

        world.register::<components::PlayerComponent>();

        animation::initialize_camera(world);

        let (sprite_sheet_handle, sprite_sheet_index, sprite_count, sprite_w, sprite_h) = 
            load_sprite_sheet(world);

        let sprite_offset_x = sprite_count as f32 * sprite_w / 2.0;
        let sprite_offset_y = sprite_h;
        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };
        let mut common_transform = Transform::default();
        common_transform.translation = Vector3::new(
            width / 2.0 - sprite_offset_x,
            height / 2.0 - sprite_offset_y,
            0.0
        );

        self.draw_sprites_animated(
            world,
            &common_transform,
            sprite_sheet_handle,
            sprite_sheet_index,
            sprite_count,
            sprite_w,
            sprite_h
        );

        // Initialing UI things
        // Load the font into world resources
        let font = world.read_resource::<Loader>().load(
            "fonts/Aroania.ttf",
            TtfFormat,
            Default::default(),
            (),
            &world.read_resource()
        );

        // Create an invisible area for the UI text
        let text_state_transform = UiTransform::new(
            "state_text:".to_string(),
            Anchor::Middle,
            0.0, 0.0, 1.0,
            230.0, 50.0,
            0
        );

        // UI text that anchors to top left corner of the
        // `text_state_transform` UI area
        let text_state = world
            .create_entity()
            .with(text_state_transform)
            .with(UiText::new(
                font.clone(),
                "GAMEPLAY".to_string(),
                [0.086, 0.078, 0.235, 1.0],
                60.0
            ))
            .build();

        world.add_resource(GameStateText { text: text_state });
    }

    fn handle_event(&mut self, data: StateData<CustomGameData>, event: Event) -> Trans<CustomGameData<'a, 'b>> {
        // Pull the `World` instance from the `StateData`
        let StateData { world, .. } = data;

        // Check if Pause key is being pressed
        if is_key_down(&event, VirtualKeyCode::Space) {
            // Change the game state text to reflect the state change
            let text_resource = world.read_resource::<GameStateText>();
            let mut ui_text_storage = world.write_storage::<UiText>();

            if let Some(ui_text_storage) = ui_text_storage.get_mut(text_resource.text) {
                ui_text_storage.text = "PAUSED".to_string();
            }

            // Switch to the `PausedState`
            println!("Switching to Pausedstate");
            return Trans::Push(Box::new(PausedState));
        }

        if is_key_down(&event, VirtualKeyCode::I) {
            let mut player_storage = world.write_storage::<PlayerComponent>();
            let player = player_storage.get_mut(self.player.unwrap()).expect("Failed to get components for player entity");
            player.input_state = player.input_state.next();
            println!("Player input state set to: {:?}", player.input_state);
        }

        // Closes the game when Escape is pressed
        // This doesn't work when in `PausedState`, obviously
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            return Trans::Quit
        }
        Trans::None
    }

    fn update(&mut self, data: StateData<CustomGameData>) -> Trans<CustomGameData<'a, 'b>> {
        data.data.update(&data.world, true);
        Trans::None
    }

    fn on_pause(&mut self, data: StateData<CustomGameData>) {
        // Pull the `World` instance from the `StateData`
        let StateData { world, .. } = data;

        // Pause the player sprite animation
        self.toggle_player_sprite_animation(world);
    }

    fn on_resume(&mut self, data: StateData<CustomGameData>) {
        // Pull the `World` instance from the `StateData`
        let StateData { world, .. } = data;

        // Resume the player sprite animation
        self.toggle_player_sprite_animation(world);
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b>> for PausedState {
    fn handle_event(&mut self, data: StateData<CustomGameData>, event: Event) -> Trans<CustomGameData<'a, 'b>> {
        let StateData { world, .. } = data;

        if is_key_down(&event, VirtualKeyCode::Space) {
            // Switch back to the `Gameplay` state when space is pressed.
            let text_resource = world.read_resource::<GameStateText>();
            let mut ui_text_storage = world.write_storage::<UiText>();

            if let Some(ui_text_storage) = ui_text_storage.get_mut(text_resource.text) {
                ui_text_storage.text = "GAMEPLAY".to_string();
            }

            // Switching back to the `GameplayState`
            println!("Switching to GameplayState");
            return Trans::Pop;
        }
        Trans::None
    }

    fn update(&mut self, data: StateData<CustomGameData>) -> Trans<CustomGameData<'a, 'b>> {
        data.data.update(&data.world, false);
        Trans::None
    }
}

fn load_sprite_sheet(world: &mut World) -> (SpriteSheetHandle, u64, usize, f32, f32) {
    let sprite_sheet_index = 0;
    let texture = sprite::load("textures/walkingwhiteball.png", world);
    world.write_resource::<MaterialTextureSet>().insert(sprite_sheet_index, texture);

    let sprite_w = 64.0;
    let sprite_h = 64.0;
    let sprite_sheet_definition = SpriteSheetDefinition::new(sprite_w, sprite_h, 1, 6, false);
    let sprite_sheet = sprite_sheet_loader::load(sprite_sheet_index, &sprite_sheet_definition);
    let sprite_count = sprite_sheet.sprites.len();

    let sprite_sheet_handle = {
        let loader = world.read_resource::<Loader>();
        loader.load_from_data(
            sprite_sheet,
            (),
            &world.read_resource::<AssetStorage<SpriteSheet>>()
        )
    };

    world
        .write_resource::<SpriteSheetSet>()
        .insert(sprite_sheet_index, sprite_sheet_handle.clone());

    (
        sprite_sheet_handle,
        sprite_sheet_index,
        sprite_count,
        sprite_w,
        sprite_h
    )
}

fn main() -> Result<(), amethyst::Error> {
    amethyst::start_logger(Default::default());

    let path = format!(
        "{}/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let config = DisplayConfig::load(&path);

    let key_bindings_path = format!(
        "{}/resources/input.ron",
        env!("CARGO_MANIFEST_DIR")
    );

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            // Lets game the background color nice purple-y
            .clear_target([0.408, 0.361, 0.733, 1.0], 1.0)
            // Let's draw sprites
            .with_pass(DrawSprite::new().with_transparency(ColorMask::all(), ALPHA, None))
            // Let's draw the UI too!
            .with_pass(DrawUi::new())
    );

    let game_data = CustomGameDataBuilder::default()
        .with_base_bundle(AnimationBundle::<u32, SpriteRender>::new(
            "animation_control_system",
            "sampler_interpolation_system"
        ))?
        .with_base_bundle(TransformBundle::new().with_dep(&["animation_control_system", "sampler_interpolation_system"]))?
        .with_base_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_base_bundle(InputBundle::<String, String>::new().with_bindings_from_file(&key_bindings_path)?)?
        .with_base_bundle(UiBundle::<String, String>::new())?
        .with_running_bundle(InputBundle::<String, String>::new().with_bindings_from_file(&key_bindings_path)?)?
        .with_running(systems::MovePlayerSystem::default(), "move_player_system", &["input_system"]);

    let mut game = Application::build("./", GameplayState::new())?
        .build(game_data)?;
    game.run();

    Ok(())
}
