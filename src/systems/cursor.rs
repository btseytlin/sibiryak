use amethyst::{
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};

pub struct CursorSystem;

use crate::game::{Cursor};

pub const CURSOR_SPEED_X: f32 = 2.0;
pub const CURSOR_SPEED_Y: f32 = 2.0;

impl<'s> System<'s> for CursorSystem {
    type SystemData = (
        WriteStorage<'s, Cursor>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut cursors, mut transforms, input): Self::SystemData) {
         for (cursor, transform) in (&mut cursors, &mut transforms).join() {
            if let Some((mouse_x, mouse_y)) = input.mouse_position() {
                println!("{:?} {:?}", mouse_x, mouse_y);
                transform.set_translation_x(mouse_x.min(0.0).max(500.0));
                transform.set_translation_y(mouse_y.min(0.0).max(500.0));
            }
         }
    }
}