use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(PartialEq, Clone, Debug)]
pub enum PlayerState {
    Standing,
    Moving
}


#[derive(Clone, Debug)]
pub struct PlayerComponent {
    pub speed: f32,
    pub state: PlayerState
}

impl Default for PlayerComponent {
    fn default() -> PlayerComponent {
        PlayerComponent {
            speed: 100.0,
            state: PlayerState::Standing
        }
    }
}

impl Component for PlayerComponent {
    type Storage = DenseVecStorage<Self>;
}

impl PlayerComponent {
    pub fn is_moving(&self) -> bool {
        self.state != PlayerState::Standing
    }
}