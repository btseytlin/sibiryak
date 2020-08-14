use amethyst::{
    prelude::*,
    ecs::prelude::*,
    assets::{AssetStorage, Loader, Handle},
    renderer::{Camera, Texture, SpriteRender, SpriteSheet, SpriteSheetFormat, ImageFormat},
    core::{transform::Transform, timing::Time},
};

#[derive(Default)]
pub struct Game {
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {        
        let world = data.world;

        // TODO remove
        world.register::<Player>();
        world.register::<Cursor>();

        self.sprite_sheet_handle.replace(load_spritesheet("player_spritesheet", world));
        init_player(world, self.sprite_sheet_handle.clone().unwrap());


        //let cursor_handle = load_spritesheet("crosshair", world);
        //init_cursor(world, cursor_handle);

        init_camera(world);

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


fn init_player(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(250.0, 250.0, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    world.create_entity()
        .with(transform)
        .with(sprite_render)
        .with(Player::new())
        .build();
}

fn init_cursor(world: &mut World, cursor_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(250.0, 250.0, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: cursor_handle,
        sprite_number: 0,
    };

    world.create_entity()
        .with(transform)
        .with(sprite_render)
        .with(Cursor {})
        .build();
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(250.0, 250.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(500.0, 500.0))
        .with(transform)
        .build();
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