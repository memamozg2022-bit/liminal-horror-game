use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

mod texture_generator;
mod level_generator;
mod atmosphere;

use level_generator::LevelGenerator;
use atmosphere::{setup_atmosphere, update_fog};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_linear()))
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_atmosphere)
        .add_systems(Update, (
            player_movement,
            move_camera,
            spawn_new_chunks,
            despawn_far_chunks,
            update_fog,
        ))
        .run();
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct Chunk {
    x: i32,
    y: i32,
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    commands.spawn(Camera2dBundle::default());

    let player_texture = create_player_texture(&mut images);
    commands
        .spawn(SpriteBundle {
            texture: player_texture,
            transform: Transform::default().with_scale(Vec3::splat(4.0)),
            ..default()
        })
        .insert(Player);

    let mut level_gen = LevelGenerator::new();
    for x in -2..=2 {
        for y in -2..=2 {
            spawn_chunk(&mut commands, &mut images, &mut level_gen, x, y);
        }
    }
}

fn spawn_chunk(
    commands: &mut Commands,
    images: &mut ResMut<Assets<Image>>,
    level_gen: &mut LevelGenerator,
    chunk_x: i32,
    chunk_y: i32,
) {
    let chunk_size = 256.0;
    let x = (chunk_x as f32) * chunk_size;
    let y = (chunk_y as f32) * chunk_size;

    let texture = level_gen.generate_chunk_texture(chunk_x, chunk_y, images);

    commands
        .spawn(SpriteBundle {
            texture,
            transform: Transform::default()
                .with_translation(Vec3::new(x + chunk_size / 2.0, y + chunk_size / 2.0, 0.0)),
            ..default()
        })
        .insert(Chunk { x: chunk_x, y: chunk_y });
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let speed = 2.0;

    for mut transform in query.iter_mut() {
        let mut velocity = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            velocity.y += speed;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            velocity.y -= speed;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            velocity.x -= speed;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            velocity.x += speed;
        }

        transform.translation += velocity;
    }
}

fn move_camera(
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            let target = player_transform.translation;
            camera_transform.translation = Vec3::new(target.x, target.y, camera_transform.translation.z);
        }
    }
}

fn spawn_new_chunks(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    chunk_query: Query<&Chunk>,
    mut images: ResMut<Assets<Image>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let chunk_size = 256.0;
        let player_chunk_x = (player_transform.translation.x / chunk_size).floor() as i32;
        let player_chunk_y = (player_transform.translation.y / chunk_size).floor() as i32;

        let mut existing_chunks = std::collections::HashSet::new();
        for chunk in chunk_query.iter() {
            existing_chunks.insert((chunk.x, chunk.y));
        }

        let mut level_gen = LevelGenerator::new();

        for x in (player_chunk_x - 2)..=(player_chunk_x + 2) {
            for y in (player_chunk_y - 2)..=(player_chunk_y + 2) {
                if !existing_chunks.contains(&(x, y)) {
                    spawn_chunk(&mut commands, &mut images, &mut level_gen, x, y);
                }
            }
        }
    }
}

fn despawn_far_chunks(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    chunk_query: Query<(Entity, &Chunk)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let chunk_size = 256.0;
        let player_chunk_x = (player_transform.translation.x / chunk_size).floor() as i32;
        let player_chunk_y = (player_transform.translation.y / chunk_size).floor() as i32;

        for (entity, chunk) in chunk_query.iter() {
            let distance = ((chunk.x - player_chunk_x).abs().max((chunk.y - player_chunk_y).abs())) as f32;
            if distance > 3.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn create_player_texture(images: &mut ResMut<Assets<Image>>) -> Handle<Image> {
    let size = 16;
    let mut data = vec![0u8; size * size * 4];

    for y in 0..size {
        for x in 0..size {
            let idx = (y * size + x) * 4;
            let dx = (x as f32 - size as f32 / 2.0).abs();
            let dy = (y as f32 - size as f32 / 2.0).abs();

            if dx < 4.0 && dy < 6.0 {
                data[idx] = 200;
                data[idx + 1] = 200;
                data[idx + 2] = 220;
                data[idx + 3] = 200;
            } else if dx < 6.0 && dy < 4.0 {
                data[idx] = 220;
                data[idx + 1] = 220;
                data[idx + 2] = 240;
                data[idx + 3] = 180;
            } else {
                data[idx + 3] = 0;
            }
        }
    }

    images.add(Image::new(
        Extent3d {
            width: size as u32,
            height: size as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
    ))
}
