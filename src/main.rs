#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::{prelude::*, window::EnabledButtons};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod animation_system;
mod attacker;

fn main() {
    App::new()
        .add_plugins((
            EmbeddedAssetPlugin {
                mode: bevy_embedded_assets::PluginMode::ReplaceDefault,
            },
            DefaultPlugins
                .set(AssetPlugin {
                    watch_for_changes_override: Some(true),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (1600., 900.).into(),
                        title: "Reaction Test".into(),
                        resizable: false,
                        enabled_buttons: EnabledButtons {
                            minimize: false,
                            maximize: false,
                            close: true,
                        },
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        ))
        .add_plugins(bevy_framepace::FramepacePlugin)
        .add_plugins(bevy_framepace::debug::DiagnosticsPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .add_systems(Update, ui_example_system)
        .add_plugins(character_data::CharacterDataPlugin)
        .add_plugins(attacker::AttackerPlugin)
        .run();
}

fn setup(mut commands: Commands, mut settings: ResMut<bevy_framepace::FramepaceSettings>) {
    settings.limiter = bevy_framepace::Limiter::from_framerate(60.);
    commands.spawn(Camera2dBundle::default());
}

fn ui_example_system(mut contexts: EguiContexts, settings: Res<bevy_framepace::FramepaceSettings>) {
    egui::Window::new("Debug").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("FPS: {}", settings.limiter));
    });
}
