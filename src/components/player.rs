use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(PartialEq, Clone, Debug)]
pub enum PlayerState {
    Standing,
    Moving
}

#[derive(PartialEq, Clone, Debug)]
pub enum InputState {
    Mouse,
    Keyboard,
    Controller
}

impl InputState {
    // For cycling through the `InputState` values
    pub fn next(&self) -> Self {
        use InputState::*;
        match *self {
            Mouse => Keyboard,
            Keyboard => Controller,
            Controller => Mouse
        }
    }
}

#[derive(Clone, Debug)]
pub struct PlayerComponent {
    pub speed: f32,
    pub state: PlayerState,
    pub input_state: InputState
}

impl Default for PlayerComponent {
    fn default() -> PlayerComponent {
        PlayerComponent {
            speed: 100.0,
            state: PlayerState::Standing,
            input_state: InputState::Mouse
        }
    }
}

impl Component for PlayerComponent {
    type Storage = DenseVecStorage<Self>;
}

impl PlayerComponent {
    // Check if player is moving
    pub fn is_moving(&self) -> bool {
        self.state != PlayerState::Standing
    }

    // Check if player is using mouse input
    pub fn uses_mouse(&self) -> bool {
        self.input_state == InputState::Mouse
    }

    // Check if player is using keyboard input
    pub fn uses_keyboard(&self) -> bool {
        self.input_state == InputState::Keyboard
    }
}