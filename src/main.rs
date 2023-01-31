use bevy::{prelude::*, input::mouse::{MouseMotion, MouseScrollUnit}};
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_plugin(ShapePlugin)
        .insert_resource(ScrollState { x: 0.0, y: 0.0 })
        .add_startup_system(setup)
        .add_system(animate_sprite)
        .run();
}

#[derive(Default, Resource)]
struct ScrollState {
    x: f32,
    y: f32,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn scroll_system(
    mouse_motion_events: Res<Events<MouseMotion>>,
    mut scroll_state: ResMut<ScrollState>,
    mut queries: Query<(Entity, &mut Transform)>,
) {
    for event in mouse_motion_events.iter_current_update_events() {
        // Check if mouse scroll goes up or down
        if event.delta.y > 0.0 {
            // Add values to entity position
            scroll_state.x += 1.0;
            scroll_state.y += 1.0;
        } else if event.delta.y < 0.0 {
            // Sub values to entity position
            scroll_state.x -= 1.0;
            scroll_state.y -= 1.0;
        }
        // Update entity's position with the new values
        for (entity, mut transform) in &mut queries.iter() {
            transform.translation = Vec3::new(scroll_state.x, scroll_state.y, 0.0);
        }

        // Print values on screen
        println!("x: {}, y: {}", scroll_state.x, scroll_state.y);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let shape = shapes::RegularPolygon {
        sides: 20,
        feature: shapes::RegularPolygonFeature::Radius(200.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
    commands.spawn(GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::BLUE),
            outline_mode: StrokeMode::new(Color::BLACK, 2.0),
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));
}
