use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Join, Read, ReadExpect, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::core::cgmath::{Vector2, Vector3, MetricSpace, InnerSpace};
use amethyst::renderer::{ScreenDimensions, MouseButton};
use components::PlayerComponent;
use components::PlayerState;

#[derive(Default)]
pub struct MovePlayerSystem {
    mouse_target_location: Option<(f64, f64)>
}

impl<'s> System<'s> for MovePlayerSystem {
    type SystemData = (
        WriteStorage<'s, PlayerComponent>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, InputHandler<String, String>>,
        ReadExpect<'s, ScreenDimensions>
    );

    fn run(&mut self, (mut players, mut transforms, time, input, screen): Self::SystemData) {
        for (player, transform) in (&mut players, &mut transforms).join() {
            // Handle player movement with keyboard
            if player.uses_keyboard() {
                let mut player_location = transform.translation;
                let mut input_dir = Vector2::new(0.0, 0.0);

                input_dir.x = input.axis_value("horizontal_movement").unwrap();
                input_dir.y = input.axis_value("vertical_movement").unwrap();

                // Actually move the player entity
                transform.translation[0] += player.speed * time.delta_seconds() * input_dir.x as f32;
                transform.translation[1] += player.speed * time.delta_seconds() * input_dir.y as f32;

                // Set player state to `Standing` or `Moving` only once
                // when player entity starts moving or stops moving
                if player_location == transform.translation && player.state != PlayerState::Standing {
                    player.state = PlayerState::Standing;
                    println!("PlayerState set to standing.");
                } else if player_location != transform.translation && player.state != PlayerState::Moving {
                    player.state = PlayerState::Moving;
                    println!("PlayerState set to moving.");
                }
            }

            // Handle player movement with mouse clicks
            if player.uses_mouse() {
                if input.mouse_button_is_down(MouseButton::Left) {
                    // Set new target location for the player entity to move to
                    self.mouse_target_location = input.mouse_position();

                    // Let's do it only once per click
                    if !player.is_moving() {
                        player.state = PlayerState::Moving;
                        println!("PlayerState set to moving.");
                    }
                }

                // Get x and y coordinates from the saved location
                if let Some((pox, poy)) = self.mouse_target_location {
                    // Move the player as long the player state is set as `Moving`
                    if player.is_moving() {
                        // Mouse target location into `Vector3`
                        // Y-axis needs to be "inverted" with the screen height
                        // for some reason ¯\_(ツ)_/¯
                        let mut target_location = Vector3::new(pox as f32, screen.height() - poy as f32, 0.0);
                        let distance = transform.translation.distance(target_location);
                        let direction = (target_location - transform.translation).normalize();

                        // Actually move the player entity
                        transform.translation[0] += player.speed * time.delta_seconds() * direction.x as f32;
                        transform.translation[1] += player.speed * time.delta_seconds() * direction.y as f32;

                        // Distance stops at around 0.5 or so
                        // So we conclude that we're there
                        if distance <= 1.0 {
                            self.mouse_target_location = None;
                            player.state = PlayerState::Standing;
                            println!("PlayerState set to standing.");
                        }
                    }
                }
            }

        }
    }
}