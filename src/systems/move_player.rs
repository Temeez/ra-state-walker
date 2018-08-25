use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Join, Read, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::core::cgmath::Vector2;

use components::PlayerComponent;
use components::PlayerState;

pub struct MovePlayerSystem;

impl<'s> System<'s> for MovePlayerSystem {
    type SystemData = (
        WriteStorage<'s, PlayerComponent>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, InputHandler<String, String>>
    );

    fn run(&mut self, (mut players, mut transforms, time, input): Self::SystemData) {
        for (player, transform) in (&mut players, &mut transforms).join() {
            let mut player_location = transform.translation;
            let mut input_dir = Vector2::new(0.0, 0.0);

            input_dir.x = input.axis_value("horizontal_movement").unwrap();
            input_dir.y = input.axis_value("vertical_movement").unwrap();

            transform.translation[0] += player.speed * time.delta_seconds() * input_dir.x as f32;
            transform.translation[1] += player.speed * time.delta_seconds() * input_dir.y as f32;

            if player_location == transform.translation && player.state != PlayerState::Standing {
                player.state = PlayerState::Standing;
                println!("PlayerState set to standing.");
            } else if player_location != transform.translation && player.state != PlayerState::Moving {
                player.state = PlayerState::Moving;
                println!("PlayerState set to moving.");
            }
        }
    }
}