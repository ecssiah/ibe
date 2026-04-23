use bevy::{prelude::*, image::ImagePlugin};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(
                ImagePlugin::default_nearest()
            )
        )
        .add_systems(Startup, setup)
        .add_systems(Update, move_character)
        .run();
}

#[derive(Component)]
struct Character {
    speed: f32,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Character { speed: 256.0 }, 
        Sprite::from_image(asset_server.load("textures/rpg/chars/hat-guy/hat-guy.png")),
        Transform::from_xyz(0., 0., 0.)
            .with_scale(Vec3::splat(6.0))
    ));
}

fn move_character(
    time: Res<Time>, 
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Sprite, &Character)>
) {
    let mut input_value = 0.0;

    if keyboard_input.pressed(KeyCode::KeyD) {
        input_value += 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        input_value -= 1.0;
    }

    for (mut transform, mut sprite, character) in &mut query {
        let displacement = input_value * character.speed * time.delta_secs();

        transform.translation.x += displacement;

        if input_value > 0.0 {
            sprite.flip_x = false;
        } else if input_value < 0.0 {
            sprite.flip_x = true;
        }
    }
}
