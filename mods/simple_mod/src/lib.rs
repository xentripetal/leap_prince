#[macro_use]
extern crate bevy_mod_loader;

use bevy_mod_loader::Mod;
use bevy_app::AppBuilder;
use bevy_ecs::prelude::IntoSystem;

#[derive(Debug, Default)]
pub struct SimpleMod;

impl Mod for SimpleMod {
    fn build(&self, app: &mut AppBuilder) {
        info!("Bootstrap!");
        app.add_system(hello_world.system());
    }
}

declare_mod!(SimpleMod, SimpleMod::default);


fn hello_world() {
    info!("Hello World!");
}