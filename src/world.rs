use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::bevy_egui::egui::Grid;
use bevy_inspector_egui::egui::Ui;
use bevy_inspector_egui::{bevy_egui, Context, Inspectable, InspectableRegistry};
use heron::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_map.system())
            .add_system(update_maps.system());
    }
}

fn update_maps(
    changed_tiles: Query<(&TileParent), (With<Tile>, Or<(Changed<TilePos>, Changed<Tile>)>)>,
    mut map_query: MapQuery,
) {
    for chunk in changed_tiles.iter() {
        map_query.notify_chunk(chunk.chunk);
    }
}

fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = asset_server.load("tiles.png");
    let material_handle = materials.add(ColorMaterial::texture(texture_handle));

    // Create map entity and component:
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let (mut layer_builder, layer_entity) = LayerBuilder::<TileBundle>::new(
        &mut commands,
        LayerSettings::new(
            MapSize(2, 2),
            ChunkSize(8, 8),
            TileSize(16.0, 16.0),
            TextureSize(96.0, 16.0),
        ),
        0u16,
        0u16,
        None,
    );
    for x in 0..16_i32 {
        for y in 0..16_i32 {
            let mut build_tile = false;
            if x == 0 || y == 0 || x == 15 || y == 15 {
                build_tile = true;
            }

            if x > 3 && x < 6 && y == 7 {
                build_tile = true;
            }

            if build_tile {
                layer_builder
                    .set_tile(TilePos(x as u32, y as u32), TileBundle::default())
                    .unwrap();
                let tile_entity = layer_builder
                    .get_tile_entity(&mut commands, TilePos(x as u32, y as u32))
                    .unwrap();
                commands.entity(tile_entity).insert(CollisionShape::Cuboid {
                    half_extends: Vec3::new(0.5, 0.5, 0.5),
                    border_radius: None,
                });
            }
        }
    }

    map_query.build_layer(&mut commands, layer_builder, material_handle);

    // Required to keep track of layers for a map internally.
    map.add_layer(&mut commands, 0u16, layer_entity);

    // Spawn Map
    // Required in order to use map_query to retrieve layers/tiles.
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(0.0, 0.0, 0.0))
        .insert(GlobalTransform::default());
}
