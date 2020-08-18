use amethyst::{
    renderer::{Camera},
    core::{Transform},
    ecs::prelude::*,
    input::{InputHandler, StringBindings},
    window::ScreenDimensions,
};
use crate::game::{Cursor};

pub struct CursorSystem;

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
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Cursor>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(&mut self, (cameras, mut cursors, mut transforms, input, screen_dimensions): Self::SystemData) {
        let mut camera_pos: Option<(f32, f32)> = None;

        for (_camera, transform) in (&cameras, &mut transforms).join() {
            camera_pos.replace((transform.translation().x, transform.translation().y));
            break;
        }


        if let Some((mouse_x, mouse_y)) = input.mouse_position() {
            if let Some((camera_x, camera_y)) = camera_pos {
                for (_cursor, transform) in (&mut cursors, &mut transforms).join() {
                    //println!("Mouse {:?} {}", mouse_x, mouse_y );

                    let (abs_mouse_x, abs_mouse_y) = get_abs_mouse_position(mouse_x, mouse_y, camera_x, camera_y, screen_dimensions.width(), screen_dimensions.height());

                    transform.set_translation_x(abs_mouse_x);
                    transform.set_translation_y(abs_mouse_y);
                }
            }
        }
    }
}