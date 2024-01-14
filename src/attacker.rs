use bevy::prelude::*;

pub struct AttackerPlugin;

impl Plugin for AttackerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_attacker)
            .add_systems(Update, animate_attacker);
    }
}

#[derive(Component)]
struct Attacker;

#[derive(Component, Default)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

fn spawn_attacker(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("animations/attacker_long.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(720., 720.), 10, 8, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let animation_indices = AnimationIndices { first: 0, last: 79 };

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            ..default()
        },
        animation_indices,
        Name::new("Attacker"),
        Attacker,
    ));
}

fn animate_attacker(
    mut query: Query<(
        &mut AnimationIndices,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut sprite) in &mut query {
        sprite.index = if sprite.index == indices.last {
            indices.first
        } else {
            sprite.index + 1
        };
    }
}

