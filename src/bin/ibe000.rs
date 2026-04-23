use bevy::{prelude::*, image::ImagePlugin};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(
                ImagePlugin::default_nearest()
            )
        )
        .add_systems(Startup, setup_character)
        .add_systems(Update, move_character)
        .run();
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
    commands.spawn(Camera2d);

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
