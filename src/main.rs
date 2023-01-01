use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/icon.png"),
        sprite: Sprite {
            // Flip the logo to the left
            flip_x: false,
            // And don't flip it upside-down ( the default )
            flip_y: true,
            ..default()
        },
        ..default()
    });
}
