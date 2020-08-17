use amethyst::{
    core::{Named, Transform},
    ecs::prelude::*,
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
};

pub struct CursorSystem;

use crate::game::{Cursor};

pub fn get_abs_mouse_position(mouse_x: f32, 
                                   mouse_y: f32, 
                                   camera_x: f32,
                                   camera_y: f32,
                                   screen_width: f32,
                                   screen_height: f32) -> (f32, f32) {
    let mouse_y = screen_height - mouse_y;
    (camera_x - screen_width/2.0 + mouse_x, camera_y - screen_height/2.0 + mouse_y)
}

impl<'s> System<'s> for CursorSystem {
    type SystemData = (
        WriteStorage<'s, Cursor>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, ScreenDimensions>,
        ReadStorage<'s, Named>,
    );

    fn run(&mut self, (mut cursors, mut transforms, input, screen_dimensions, named): Self::SystemData) {
        let mut camera_x: f32 = 0.0;
        let mut camera_y: f32 = 0.0;

        for (name, transform) in (&named, &mut transforms).join() {
            if name.name == "camera" {
                camera_x = transform.translation().x;
                camera_y = transform.translation().y;
                break;
            }
        }

        if let Some((mouse_x, mouse_y)) = input.mouse_position() {
            for (_cursor, transform) in (&mut cursors, &mut transforms).join() {
                //println!("Mouse {:?} {}", mouse_x, mouse_y );

                let (rel_mouse_x, rel_mouse_y) = get_abs_mouse_position(mouse_x, mouse_y, camera_x, camera_y, screen_dimensions.width(), screen_dimensions.height());

                transform.set_translation_x(rel_mouse_x);
                transform.set_translation_y(rel_mouse_y);
            }
        }
    }
}