use bevy::prelude::*;
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};

pub struct AnimationSystemPlugin;

impl Plugin for AnimationSystemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AnimationData>()
            .add_systems(Update, animation_system);
    }
}

#[derive(Component, Default, Debug, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct AnimationData {
    index: usize,
    frame: usize,
    counter: i32,
}

fn animation_system(
    mut query: Query<(
        &mut AnimationData,
        &crate::character_data::AttackerStates,
        &crate::character_data::ActionData,
        &mut TextureAtlas,
    )>,
) {
    for (mut animation, state, data, mut sprite) in &mut query {
        let current_action = &data.actions[*state as usize];
        let current_frame = animation.frame;

        if animation.counter == 0 {
            animation.index = current_action.timeline[current_frame].index;
            animation.counter = current_action.timeline[current_frame].duration;
            animation.frame = (animation.frame + 1) % current_action.timeline.len();
        }

        sprite.index = animation.index;
        animation.counter -= 1;
    }
}

