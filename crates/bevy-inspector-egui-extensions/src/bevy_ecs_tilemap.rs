use super::macros::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::bevy_egui::egui::Grid;
use bevy_inspector_egui::egui::WidgetType::ComboBox;
use bevy_inspector_egui::Inspectable;
use bevy_inspector_egui::{bevy_egui, Context, InspectableRegistry};

pub struct TilemapInspectorPlugin;

pub fn register(registry: &mut InspectableRegistry) {
    registry.register_raw::<TilePos, _>(tile_pos_ui);
    registry.register_raw::<Tile, _>(tile_ui);
    registry.register_raw::<TileParent, _>(tile_parent_ui);
    registry.register_raw::<Map, _>(map_ui);
    registry.register_raw::<Chunk, _>(chunk_ui);
    registry.register_raw::<ChunkSettings, _>(chunk_settings_ui);
    registry.register_raw::<HexType, _>(hex_type_ui);
    registry.register_raw::<IsoType, _>(iso_type_ui);
}

impl_ui_for_simple_enum!(
    hex_type_ui,
    HexType: RowEven,
    RowOdd,
    ColumnEven,
    ColumnOdd,
    Row,
    Column
);

impl_ui_for_simple_enum!(iso_type_ui, IsoType: Diamond, Staggered);

fn chunk_settings_ui(
    element: &mut ChunkSettings,
    ui: &mut bevy_egui::egui::Ui,
    context: &Context,
) -> bool {
    let mut changed = false;
    ui.vertical_centered(|ui| {
        Grid::new(context.id()).show(ui, |ui| {
            ui.label("Layer ID");
            changed |= element.layer_id.ui(ui, Default::default(), context);
            ui.end_row();
        });
    });
    changed
}

fn chunk_ui(element: &mut Chunk, ui: &mut bevy_egui::egui::Ui, context: &Context) -> bool {
    let mut changed = false;
    ui.vertical_centered(|ui| {
        Grid::new(context.id()).show(ui, |ui| {
            ui.label("Settings");
            changed |= chunk_settings_ui(&mut element.settings, ui, context);
            ui.end_row();
            ui.label("Needs Remesh");
            changed |= element.needs_remesh.ui(ui, Default::default(), context);
            ui.end_row();
            ui.label("Map Entity");
            changed |= element.map_entity.ui(ui, Default::default(), context);
        });
    });
    changed
}

fn tile_parent_ui(
    element: &mut TileParent,
    ui: &mut bevy_egui::egui::Ui,
    context: &Context,
) -> bool {
    let mut changed = false;
    ui.vertical_centered(|ui| {
        Grid::new(context.id()).show(ui, |ui| {
            ui.label("Layer ID");
            changed |= element.layer_id.ui(ui, Default::default(), context);
            ui.label("Map ID");
            changed |= element.map_id.ui(ui, Default::default(), context);
            ui.end_row();
            ui.label("Chunk Entity");
            changed |= element.chunk.ui(ui, Default::default(), context);
        });
    });
    changed
}

fn map_ui(element: &mut Map, ui: &mut bevy_egui::egui::Ui, context: &Context) -> bool {
    let mut changed = false;
    ui.vertical_centered(|ui| {
        Grid::new(context.id()).show(ui, |ui| {
            ui.label("ID");
            changed |= element.id.ui(ui, Default::default(), context);
            ui.end_row();
            ui.label("Map Entity");
            changed |= element.map_entity.ui(ui, Default::default(), context);
        });
    });
    changed
}

fn tile_ui(element: &mut Tile, ui: &mut bevy_egui::egui::Ui, context: &Context) -> bool {
    let mut changed = false;
    ui.vertical_centered(|ui| {
        Grid::new(context.id()).show(ui, |ui| {
            ui.label("Texture Index");
            changed |= element.texture_index.ui(ui, Default::default(), context);
            ui.label("Color");
            changed |= element.color.ui(ui, Default::default(), context);
            ui.end_row();
            ui.label("Flip X");
            changed |= element.flip_x.ui(ui, Default::default(), context);
            ui.label("Flip Y");
            changed |= element.flip_y.ui(ui, Default::default(), context);
            ui.label("Flip D");
            changed |= element.flip_d.ui(ui, Default::default(), context);
            ui.end_row();
            ui.label("Visible");
            changed |= element.visible.ui(ui, Default::default(), context);
        });
    });
    changed
}

fn tile_pos_ui(element: &mut TilePos, ui: &mut bevy_egui::egui::Ui, context: &Context) -> bool {
    let mut changed = false;
    ui.vertical_centered(|ui| {
        Grid::new(context.id()).show(ui, |ui| {
            ui.label("X");
            changed |= element.0.ui(ui, Default::default(), context);
            ui.label("Y");
            changed |= element.1.ui(ui, Default::default(), context);
            ui.end_row();
        });
    });
    changed
}
