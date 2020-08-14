use amethyst::{
    core::{Transform, SystemDesc, timing::Time, math::Vector3},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};
use std::f32::consts::PI;

pub struct MovementSystem;

use crate::game::{Player};

pub const PLAYER_SPEED_X: f32 = 100.0;
pub const PLAYER_SPEED_Y: f32 = 100.0;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut players, mut transforms, input, time): Self::SystemData) {
         for (player, transform) in (&mut players, &mut transforms).join() {
            let movement_x = input.axis_value("move_x");
            let movement_y = input.axis_value("move_y");

            if let Some(mv_value) = movement_x {
                transform.prepend_translation_x(mv_value*PLAYER_SPEED_X*time.delta_seconds());
            } 

            if let Some(mv_value) = movement_y {
                transform.prepend_translation_y(mv_value*PLAYER_SPEED_Y*time.delta_seconds());
            } 

            if let Some((mouse_x, mouse_y)) = input.mouse_position() {
                let player_x = transform.translation().x;
                let player_y = transform.translation().y;
                let katet_a = (player_x - mouse_x).abs();
                let katet_b = (player_y - mouse_y).abs();
                let c = (katet_a.powf(2.0) + katet_b.powf(2.0)).sqrt();
                //println!("{:?} {}", mouse_x, mouse_y);
                
                let angle = { 
                    if player_x >= mouse_x && player_y >= mouse_y { 
                        (katet_a / c).asin() 
                    } else if player_x < mouse_x && player_y > mouse_y {
                        2.0*PI - (katet_a / c).asin()
                    } else if player_x > mouse_x && player_y < mouse_y {
                        PI - (katet_a / c).asin() 
                    } else if player_x < mouse_x && player_y < mouse_y {
                        PI + (katet_a / c).asin()
                    } else { 0.0 } 
                };

                println!("{}, {}, {}, {}", katet_a, c,  (katet_a / c).asin() , angle);
                

                transform.set_rotation_2d(angle);
            }
         }
    }
}