use bevy::{
    prelude::*,
    image::{ImagePlugin, ImageArrayLayout, ImageLoaderSettings},
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(
                ImagePlugin::default_nearest()
            )
        )
        .add_systems(Startup, (setup_camera, setup_character, setup_tilemap).chain())
        .add_systems(Update, move_character)
        .run();
}

fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2d);    
}

#[derive(Component)]
struct Character {
    speed: f32,
    facing: i32,
}

fn setup_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let sprite_image_handle = asset_server.load("textures/rpg/chars/hat-guy/hat-guy.png");

    commands.spawn((
        Character {
            speed: 256.0,
            facing: 0,
        }, 
        Sprite::from_image(sprite_image_handle),
        Transform::from_xyz(0., 0., 0.)
            .with_scale(Vec3::splat(6.0))
    ));
}

fn setup_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let chunk_size = UVec2::new(16, 8);
    let tile_display_size = UVec2::new(64, 64);

    let mut tile_data: Vec<Option<TileData>> = vec![None; chunk_size.element_product() as usize];

    for y in 0..chunk_size.y {
        for x in 0..chunk_size.x {
            let tile_index = (x + y * chunk_size.x) as usize;

            if y < chunk_size.y / 2 - 2 {
                tile_data[tile_index] = Some(TileData::from_tileset_index(1));
            } else if y < chunk_size.y / 2 - 1 {
                tile_data[tile_index] = Some(TileData::from_tileset_index(0));
            } else {
                tile_data[tile_index] = None;
            }
        }
    }

    commands.spawn((
        TilemapChunk {
            chunk_size,
            tile_display_size,
            tileset: asset_server.load_with_settings(
                "textures/array_texture.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.array_layout = Some(ImageArrayLayout::RowCount { rows: 4 });
                },
            ),
            ..default()
        },
        TilemapChunkTileData(tile_data),
    ));
}

fn move_character(
    time: Res<Time>, 
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Sprite, &mut Character)>
) {
    for (mut transform, mut sprite, mut character) in &mut query {
        let mut input_value = 0.0;

        if keyboard_input.pressed(KeyCode::KeyD) {
            input_value += 1.0;
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            input_value -= 1.0;
        }
        
        let displacement = input_value * character.speed * time.delta_secs();

        transform.translation.x += displacement;

        if input_value > 0.0 {
            character.facing = 1;
            sprite.flip_x = false;
        } else if input_value < 0.0 {
            character.facing = -1;
            sprite.flip_x = true;
        }
    }
}
