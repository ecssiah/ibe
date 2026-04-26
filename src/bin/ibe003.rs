use std::time::Duration;
use bevy::{
    prelude::*,
    image::{ImagePlugin, ImageArrayLayout, ImageLoaderSettings},
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};

fn main() {
    let plugins = DefaultPlugins.set(ImagePlugin::default_nearest());
    
    let startup_systems = (
        setup_camera,
        setup_character,
        setup_tilemap,
    );

    let update_systems = (
        move_character,
        animate_character,
    );
    
    App::new()
        .add_plugins(plugins)
        .add_systems(Startup, startup_systems.chain())
        .add_systems(Update, update_systems)
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
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("textures/rpg/mobs/fox-run.png");

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 6, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_data = AnimationData::new(0, 5, 10);
    
    commands.spawn((
        Character {
            speed: 256.0,
            facing: 0,
        }, 
        Sprite {
            image: texture_handle.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_data.frame_index_start,
            }),
            ..default()
        },
        Transform::from_xyz(0., 0., 0.)
            .with_scale(Vec3::splat(6.0)),
        animation_data,
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

#[derive(Component)]
struct AnimationData {
    frame_index_start: usize,
    frame_index_end: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationData {
    fn new(frame_index_start: usize, frame_index_end: usize, fps: u8) -> Self {
        Self {
            frame_index_start,
            frame_index_end,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

fn animate_character(
    time: Res<Time>,
    mut query: Query<(&mut AnimationData, &mut Sprite)>
) {
    for (mut animation_data, mut sprite) in &mut query {
        animation_data.frame_timer.tick(time.delta());

        if animation_data.frame_timer.just_finished() && let Some(atlas) = &mut sprite.texture_atlas {
            if atlas.index == animation_data.frame_index_end {
                atlas.index = animation_data.frame_index_start;
            } else {
                atlas.index += 1;
            }

            animation_data.frame_timer = AnimationData::timer_from_fps(animation_data.fps);
        }
    }
}
