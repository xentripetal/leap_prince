/**
#![windows_subsystem = "windows"]
**/
use crate::player::PlayerPlugin;
use crate::world::WorldPlugin;
use bevy::prelude::*;
use bevy::window::WindowId;
use bevy::winit::WinitWindows;
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::{InspectorPlugin, WorldInspectorPlugin};
use bevy_inspector_egui_extensions::InspectorExtensionPlugin;
use heron::{Gravity, PhysicsPlugin};
use rand::{thread_rng, Rng};

mod helpers;
mod player;
mod world;

#[derive(Default)]
struct LastUpdate {
    value: f64,
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("Leap Prince"),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.3)))
        .insert_resource(Gravity::from(Vec2::new(0.0, -9.8)))
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(TilemapPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InspectorExtensionPlugin)
        .add_system(helpers::texture::set_texture_filters_to_nearest.system())
        //.add_startup_system(set_window_icon.system())
        //.add_system(update_map.system())
        .run();
}
