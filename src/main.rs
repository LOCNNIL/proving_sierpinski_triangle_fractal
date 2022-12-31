use bevy::prelude::*;
//use bevy_input::prelude::*;


// Quick note: systems run in parallel by default whenever possible
fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands) {
    // crie um ponto vermelho na posição (0, 0)
    commands.spawn(SpriteComponents {
        material: Material {
            albedo: Color::rgb(1.0, 0.0, 0.0).into(),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        sprite: Sprite {
            size: Vec2::new(10.0, 10.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

