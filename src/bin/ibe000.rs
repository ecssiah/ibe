use bevy::{prelude::*, image::ImagePlugin};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(
                ImagePlugin::default_nearest()
            )
        )
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    let sprite_image_handle = asset_server.load("textures/rpg/chars/hat-guy/hat-guy.png");

    commands.spawn((
        Sprite::from_image(sprite_image_handle),
        Transform::from_xyz(0., 0., 0.)
            .with_scale(Vec3::splat(6.0))
    ));
}
