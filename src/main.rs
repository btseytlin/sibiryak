use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
};
use crate::game::Game;

mod game;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let assets_dir = app_root.join("assets");

    let binding_path = app_root.join("config").join("input.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
                            .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                    .with_clear([1.0, 1.0, 1.0, 1.0]),
            )
            .with_plugin(RenderFlat2D::default())
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::MovementSystem, "movement_system", &[])
        .with(systems::CursorSystem, "cursor_system", &[]);

    let mut game = Application::new(assets_dir, Game::default(), game_data)?;
    game.run();
    Ok(())
}