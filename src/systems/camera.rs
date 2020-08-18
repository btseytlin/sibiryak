use amethyst::{
    core::Transform,
    ecs::prelude::*,
    renderer::Camera,
};
use crate::game::Player;

pub struct CameraSystem;

impl<'s> System<'s> for CameraSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
    );

    fn run(&mut self, (players, mut transforms, cameras): Self::SystemData) {
        let mut player_x: Option<f32> = None;
        let mut player_y: Option<f32> = None;

        for (player, transform) in (&players, &mut transforms).join() {
            player_x.replace(transform.translation().x);
            player_y.replace(transform.translation().y);
            break;
        }

        if player_x.is_some()  && player_y.is_some()  {
            for (_camera, transform) in (&cameras, &mut transforms).join() {
                transform.set_translation_x(player_x.unwrap());
                transform.set_translation_y(player_y.unwrap());
            }
        }
    }

}