use amethyst::{
    core::{Transform, timing::Time},
    ecs::prelude::*,
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
    renderer::{Camera}
};
use std::f32::consts::PI;
use crate::game::Player;
use crate::systems::movement::{Movable};
use crate::systems::cursor::{get_abs_mouse_position};


pub struct InputSystem;

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

impl<'s> System<'s> for InputSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, Movable>,
        ReadStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(&mut self, (mut players, mut movables, transforms, input, screen_dimensions): Self::SystemData) {

        for (player, movable, transform) in (&mut players, &mut movables, &transforms).join() {
            let movement_forward = input.axis_value("movement_forward").unwrap();
            let movement_left = input.axis_value("movement_left").unwrap();

            /*let movement_forward_adjusted = movement_x*movable.velocity_forward*time.delta_seconds();
            let movement_left_adjusted = movement_y*movable.velocity_left*time.delta_seconds();*/

            movable.velocity_forward = movement_forward;
            movable.velocity_left = movement_left;

            if let Some((mouse_x, mouse_y)) = input.mouse_position() {
                let (player_x, player_y) = (transform.translation().x, transform.translation().y);

                let (rel_mouse_x, rel_mouse_y) = get_abs_mouse_position(mouse_x, mouse_y, player_x, player_y, screen_dimensions.width(), screen_dimensions.height());
                
                let angle = get_angle_to_face(player_x, player_y, rel_mouse_x, rel_mouse_y);

                movable.rotation = angle;
            }
        }
        
    }
}