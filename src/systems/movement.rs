use amethyst::{
    core::{Transform, timing::Time},
    ecs::prelude::*,
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
    renderer::{Camera}
};
use std::f32::consts::PI;

use crate::game::{Player, ActionState, Action};
use crate::systems::cursor::{get_abs_mouse_position};

pub struct Movable {
    pub speed_forward: f32,
    pub speed_left: f32,
    pub velocity_forward: f32,
    pub velocity_left: f32,
    pub rotation: f32,
}


impl Component for Movable {
    type Storage = DenseVecStorage<Self>;
}


pub struct MovementSystem;


fn get_local_angle(angle: f32) -> f32 {
    if angle >= PI/2.0 && angle <= PI {
        PI - angle
    } else if angle > PI && angle <= 1.5 * PI {
        1.5* PI - angle
    } else if angle > 1.5 * PI {
        2.0 * PI - angle
    } else {
        angle
    }
}

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, Movable>,
        WriteStorage<'s, ActionState>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        ReadStorage<'s, Camera>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(&mut self, (mut players, mut movables, mut action_state, mut transforms, input, time, cameras, screen_dimensions): Self::SystemData) {
        let mut player_x: Option<f32> = None;
        let mut player_y: Option<f32> = None;

        for (movable, transform) in (&mut movables, &mut transforms).join() {
            let x = transform.translation().x;
            let y = transform.translation().y;
            let angle = movable.rotation;

            let local_angle = get_local_angle(angle);

            let distance_forward = movable.velocity_forward * movable.speed_forward * time.delta_seconds();

            let mut dx = distance_forward * local_angle.sin();
            let mut dy = distance_forward * local_angle.cos();

            if angle >= PI/2.0 && angle <= 1.5 * PI {
                dy = -dy;
            }

            if angle <= PI {
                dx = -dx;
            }


            println!("x: {}, y: {}, angle: {}, local_angle: {}", x, y, angle, local_angle);

            println!("velocity forward: {}", movable.velocity_forward);

            println!("distance forward: {}", distance_forward);

            println!("dx: {}, dy: {}", dx, dy);


            if dx != 0.0 {
                transform.prepend_translation_x(dx);
            }

            if dy != 0.0 {
                transform.prepend_translation_y(dy);
            }

            transform.set_rotation_2d(angle);

            //movable.velocity_forward = 0.0;
            //movable.velocity_left = 0.0;
        }

        for (player, movable, action_state, transform) in (&mut players, &mut movables, &mut action_state, &mut transforms).join() {
            player_x.replace(transform.translation().x);
            player_y.replace(transform.translation().y);

            // if movement_x_adjusted.abs() > 0.0 || movement_y_adjusted.abs() > 0.0 {
            //     action_state.action = Action::Walk;
            // } else {
            //     action_state.action = Action::Idle;
            // }
        }

        // Todo separate in own system

        if player_x.is_some()  && player_y.is_some()  {
            for (_camera, transform) in (&cameras, &mut transforms).join() {
                transform.set_translation_x(player_x.unwrap());
                transform.set_translation_y(player_y.unwrap());
            }
        }
        
    }
}