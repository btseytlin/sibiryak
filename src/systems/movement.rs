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

use amethyst::core::math::{Unit, Vector3};

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
        Read<'s, Time>
    );

    fn run(&mut self, (mut players, mut movables, mut action_state, mut transforms, input, time): Self::SystemData) {
        for (movable, transform) in (&mut movables, &mut transforms).join() {
            let x = transform.translation().x;
            let y = transform.translation().y;
            let angle = movable.rotation;
            transform.set_rotation_2d(angle);

            let distance_forward = movable.velocity_forward * movable.speed_forward * time.delta_seconds();

            transform.append_translation_along(Unit::new_normalize(Vector3::new(0.0, 1.0, 0.0)), distance_forward);

            let distance_left = movable.velocity_left * movable.speed_left * time.delta_seconds();

            transform.append_translation_along(Unit::new_normalize(Vector3::new(1.0, 0.0, 0.0)), distance_left);

        }
    }
}