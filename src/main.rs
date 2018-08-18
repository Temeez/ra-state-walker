extern crate amethyst;

use amethyst::core::transform::TransformBundle;
use amethyst::assets::{Loader};
use amethyst::ecs::prelude::{Entity};
use amethyst::prelude::*;
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::ui::{UiBundle, DrawUi, Anchor, TtfFormat, UiText, UiTransform};
use amethyst::renderer::{DisplayConfig, DrawFlat, Event, Pipeline, PosNormTex,
                         RenderBundle, Stage, VirtualKeyCode};

struct GameplayState;
struct PausedState;

pub struct GameStateText {
    pub text: Entity
}

impl<'a, 'b> State<GameData<'a, 'b>> for GameplayState {
    fn on_start(&mut self, data: StateData<GameData>) {
        // Pull the `World` instance from the `StateData`
        let StateData { world, .. } = data;

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

    fn handle_event(&mut self, data: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
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

        // Closes the game when Escape is pressed
        // This doesn't work when in `PausedState`, obviously
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            return Trans::Quit
        }
        Trans::None
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        Trans::None
    }
}

impl<'a, 'b> State<GameData<'a, 'b>> for PausedState {
    fn handle_event(&mut self, data: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
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

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        Trans::None
    }
}

fn main() -> Result<(), amethyst::Error> {
    amethyst::start_logger(Default::default());

    let path = format!(
        "{}/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            // Lets game the background color nice purple-y
            .clear_target([0.408, 0.361, 0.733, 1.0], 1.0)
            // TODO: FO
            .with_pass(DrawFlat::<PosNormTex>::new())
            // Let's draw the UI too!
            .with_pass(DrawUi::new())
    );

    let game_data = GameDataBuilder::default()
        // Registers transform stuffs
        .with_bundle(TransformBundle::new())?
        // Registers UI stuffs
        .with_bundle(UiBundle::<String, String>::new())?
        // Registers rendering stuffs
        .with_bundle(RenderBundle::new(pipe, Some(config)))?;
    let mut game = Application::build("./", GameplayState)?
        .build(game_data)?;
    game.run();
    Ok(())
}
