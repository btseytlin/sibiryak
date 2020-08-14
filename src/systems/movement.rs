use amethyst::{
    core::{Transform, SystemDesc, timing::Time, math::Vector3},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
};
use std::f32::consts::PI;

pub struct MovementSystem;

use crate::game::{Player};

pub const PLAYER_SPEED_X: f32 = 100.0;
pub const PLAYER_SPEED_Y: f32 = 100.0;

fn get_angle_to_face(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let triangle_oposite_side = (x1 - x2).abs();
    let triangle_close_side = (y1 - y2).abs();
    let distance = (triangle_oposite_side.powf(2.0) + triangle_close_side.powf(2.0)).sqrt();

    if x1 >= x2 && y1 <= y2 { 
        (triangle_oposite_side / distance).asin() 
    } else if x1 < x2 && y1 < y2 {
        2.0*PI - (triangle_oposite_side / distance).asin()
    } else if x1 > x2 && y1 > y2 {
        PI - (triangle_oposite_side / distance).asin() 
    } else {
        PI + (triangle_oposite_side / distance).asin()
    }
}

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(&mut self, (mut players, mut transforms, input, time, screen_dimensions): Self::SystemData) {
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
                let mouse_y = screen_dimensions.height() - mouse_y;
                let player_x = transform.translation().x;
                let player_y = transform.translation().y;
                
                let angle = get_angle_to_face(player_x, player_y, mouse_x, mouse_y);

                transform.set_rotation_2d(angle);
            }
         }
    }
}