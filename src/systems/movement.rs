use amethyst::{
    core::{Named, Transform, SystemDesc, timing::Time, math::Vector3},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
    renderer::{Camera}
};
use std::f32::consts::PI;

pub struct MovementSystem;

use crate::game::{Player};
use crate::systems::cursor::{get_abs_mouse_position};

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
        ReadStorage<'s, Camera>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(&mut self, (mut players, mut transforms, input, time, cameras, screen_dimensions): Self::SystemData) {
        let mut player_x: Option<f32> = None;
        let mut player_y: Option<f32> = None;
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
                player_x.replace(transform.translation().x);
                player_y.replace(transform.translation().y);

                
                let (rel_mouse_x, rel_mouse_y) = get_abs_mouse_position(mouse_x, mouse_y, player_x.unwrap(), player_y.unwrap(), screen_dimensions.width(), screen_dimensions.height());
                
                let angle = get_angle_to_face(player_x.unwrap(), player_y.unwrap(), rel_mouse_x, rel_mouse_y);

                transform.set_rotation_2d(angle);
            }
            //println!("Player {:?} {}", transform.translation().x, transform.translation().y);
        }
        if player_x.is_some()  && player_y.is_some()  {
            for (camera, transform) in (&cameras, &mut transforms).join() {
                transform.set_translation_x(player_x.unwrap());
                transform.set_translation_y(player_y.unwrap());
                //println!("Camera {:?} {}", transform.translation().x, transform.translation().y);
            }
        }
        
    }
}