use amethyst::{
    core::{Transform, timing::Time},
    ecs::prelude::*,
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
    renderer::{Camera}
};
use std::f32::consts::PI;
use amethyst::core::math::{Unit, Vector3};

pub struct Movable {
    pub speed_forward: f32,
    pub speed_backwards: f32,
    pub speed_sideways: f32,
    pub velocity_forward: f32,
    pub velocity_sideways: f32,
    pub rotation: f32,
}


impl Component for Movable {
    type Storage = DenseVecStorage<Self>;
}

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Movable>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>
    );

    fn run(&mut self, (mut movables, mut transforms, input, time): Self::SystemData) {
        for (movable, transform) in (&mut movables, &mut transforms).join() {
            let x = transform.translation().x;
            let y = transform.translation().y;
            let angle = movable.rotation;
            transform.set_rotation_2d(angle);
            
            let distance_forward = movable.velocity_forward * time.delta_seconds();
            transform.append_translation_along(Unit::new_normalize(Vector3::new(0.0, 1.0, 0.0)), distance_forward);

            let distance_sideways = movable.velocity_sideways * time.delta_seconds();
            transform.append_translation_along(Unit::new_normalize(Vector3::new(1.0, 0.0, 0.0)), distance_sideways);

        }
    }
}