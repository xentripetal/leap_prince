use bevy_app::{AppBuilder, Plugin};
use bevy_inspector_egui::InspectableRegistry;

pub struct InspectorExtensionPlugin;

#[cfg(feature = "ecs_tilemap")]
mod bevy_ecs_tilemap;
mod macros;

impl Plugin for InspectorExtensionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let mut registry = app
            .world_mut()
            .get_resource_or_insert_with(InspectableRegistry::default);

        #[cfg(feature = "ecs_tilemap")]
        bevy_ecs_tilemap::register(&mut registry)
    }
}
