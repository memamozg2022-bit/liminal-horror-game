use bevy::prelude::*;

#[derive(Component)]
pub struct FogEffect;

pub fn setup_atmosphere(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.1, 0.1, 0.15, 0.2),
                custom_size: Some(Vec2::new(10000.0, 10000.0)),
                ..default()
            },
            transform: Transform::default().with_translation(Vec3::new(0.0, 0.0, 10.0)),
            ..default()
        })
        .insert(FogEffect);
}

pub fn update_fog(
    mut fog_query: Query<(&mut Transform, &mut Sprite), With<FogEffect>>,
    player_query: Query<&Transform, With<crate::Player>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok((mut fog_transform, mut fog_sprite)) = fog_query.get_single_mut() {
            fog_transform.translation = Vec3::new(
                player_transform.translation.x,
                player_transform.translation.y,
                fog_transform.translation.z,
            );

            let time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f32();
            let alpha = 0.15 + ((time * 0.5).sin() * 0.1);

            fog_sprite.color = Color::srgba(0.1, 0.1, 0.15, alpha);
        }
    }
}
