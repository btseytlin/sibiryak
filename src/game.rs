use amethyst::{
    prelude::*,
    ecs::prelude::*,
    assets::{AssetStorage, Loader, Handle},
    renderer::{Camera, Texture, SpriteRender, SpriteSheet, SpriteSheetFormat, ImageFormat},
    core::{transform::Transform, timing::Time},
    window::ScreenDimensions,
};

use crate::systems::{
    movement::{Movable},
    animation::{AnimationId, Animation, AnimationState, AnimationsResource}
};

use amethyst::core::math::{Vector3};

#[derive(Default)]
pub struct Game {
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {        
        let world = data.world;

        let time = *(world.try_fetch::<Time>().unwrap());
        let animations_resource = AnimationsResource::new();
        world.insert(animations_resource);

        self.sprite_sheet_handle.replace(load_spritesheet("player_spritesheet", world));
        init_player(world, self.sprite_sheet_handle.clone().unwrap(), time);

        let wall_handle = load_spritesheet("wall", world);
        init_wall(world, &wall_handle);

        let background_handle = load_spritesheet("background", world);
        init_background_sprite(world, &background_handle);

        init_camera(world);

        let cursor_handle = load_spritesheet("crosshair", world);
        init_cursor(world, cursor_handle);

        println!("{:?}", "game started");
    }
}

pub struct Player;

impl Player {
    fn new() -> Player {
        Player {}
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

pub struct Cursor;

impl Component for Cursor {
    type Storage = DenseVecStorage<Self>;
}


fn init_player(world: &mut World, 
    sprite_sheet_handle: Handle<SpriteSheet>,
    time: Time,
    ) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(250.0, 250.0, 0.0);

    let movement = Movable {
        speed_forward: 200.0,
        speed_backwards: 80.0,
        speed_sideways: 100.0,
        velocity_forward: 0.0,
        velocity_sideways: 0.0,
        rotation: 0.0,
    };

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    let player = Player::new();

    let animation_state = AnimationState::new(AnimationId::PlayerIdle, AnimationsResource::get(AnimationId::PlayerIdle), time.absolute_time_seconds());

    world.create_entity()
        .with(transform)
        .with(sprite_render)
        .with(movement)
        .with(animation_state)
        .with(player)
        .named("player")
        .build()
}

fn init_cursor(world: &mut World, cursor_handle: Handle<SpriteSheet>) -> Entity {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(width / 2.0, height / 2.0, 0.0);
    transform.set_scale(Vector3::new(0.5, 0.5, 0.5));

    let sprite_render = SpriteRender {
        sprite_sheet: cursor_handle,
        sprite_number: 0,
    };

    world.create_entity()
        .with(transform)
        .with(sprite_render)
        .with(Cursor {})
        .build()
}

fn init_camera(world: &mut World) -> Entity {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    let mut transform = Transform::default();
    transform.set_translation_z(1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(width, height))
        .with(transform)
        .named("camera")
        .build()
}

fn init_wall(world: &mut World, sprite_sheet: &Handle<SpriteSheet>) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_z(0.0);
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };
    world
        .create_entity()
        .with(transform)
        .with(sprite)
        .named("wall")
        .build()
}


fn init_background_sprite(world: &mut World, sprite_sheet: &Handle<SpriteSheet>) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_z(-10.0);
    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };
    world
        .create_entity()
        .with(transform)
        .with(sprite)
        .named("background")
        .build()
}


fn load_spritesheet(name: &str, world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("texture/{}.png", name),
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("texture/{}.ron", name),
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}