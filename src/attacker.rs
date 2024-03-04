use bevy::prelude::*;

pub struct AttackerPlugin;

impl Plugin for AttackerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_attacker);
    }
}

#[derive(Component)]
pub struct Attacker;

pub fn spawn_attacker(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("animations/attacker.png");
    let texture_atlas = TextureAtlasLayout::from_grid(Vec2::new(720., 720.), 17, 2, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        SpriteSheetBundle {
            sprite: Sprite::default(),
            atlas: TextureAtlas {
                layout: texture_atlas_handle,
                index: 0,
            },
            texture: texture_handle,
            ..default()
        },
        crate::animation_system::AnimationData::default(),
        crate::character_data::AttackerStates::Hit6A,
        Name::new("Attacker"),
        Attacker,
    ));
}
