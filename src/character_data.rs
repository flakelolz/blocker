#![allow(unused)]
use std::io::Read;

use bevy::prelude::*;
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions, quick::ResourceInspectorPlugin};
use miniserde::{json, Deserialize};

pub struct CharacterDataPlugin;

impl Plugin for CharacterDataPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ActionData>()
            .add_systems(PostStartup, insert_actions);
    }
}

#[derive(Deserialize, Debug, Reflect, InspectorOptions)]
pub struct ImageRange {
    sequence: String,
    index: usize,
    start: usize,
    duration: i32,
}

impl Default for ImageRange {
    fn default() -> Self {
        Self {
            duration: 1,
            ..default()
        }
    }
}

#[derive(Default, Deserialize, Debug, Reflect, InspectorOptions)]
pub struct ActionProperties {
    pub name: String,
    pub duration: i32,
    pub timeline: Vec<ImageRange>,
}

#[derive(Component)]
pub enum AttackerStates {
    Idle,
    Hit5A,
    Hit2A,
    Hit5B,
    Hit2B,
    Hit6A,
    Throw,
}

#[derive(Component, Default, Deserialize, Debug, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
pub struct ActionData {
    pub actions: Vec<ActionProperties>,
}

fn insert_actions(mut commands: Commands, query: Query<(Entity, &Name)>) {
    let mut attacker_data;
    let mut defender_data;

    for (entity, name) in &query {
        if name.as_str() == "Attacker" {
            attacker_data = load_data("attacker").expect("Failed to load attacker data");
            commands.entity(entity).insert(attacker_data);
            println!("Attacker loaded");
        } else if name.as_str() == "Defender" {
            defender_data = load_data("defender").expect("Failed to load defender data");
            commands.entity(entity).insert(defender_data);
        }
    }
}

fn load_data(name: &str) -> Option<ActionData> {
    match std::fs::File::options()
        .read(true)
        .open(format!("assets/data/{name}_data.json"))
    {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            json::from_str(&contents).unwrap()
        }
        Err(err) => {
            println!("{:?}", err);
            None
        }
    }
}

