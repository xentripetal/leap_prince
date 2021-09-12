use bevy::prelude::*;
use heron::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_player.system())
            .add_system(movement.system());
    }
}

enum PlayerState {
    Jumping,
    Standing,
    Moving,
    Crouching { crouch_time: f64 },
}

struct Player {
    pub state: PlayerState,
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("player.png");
    let sprite_bundle = SpriteBundle {
        material: materials.add(texture_handle.into()),
        transform: Transform::from_translation(Vec3::new(8.0, 8.0, 0.0)),
        ..Default::default()
    };

    commands
        .spawn()
        //.spawn_bundle(sprite_bundle)
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(0.5, 0.5, 0.5),
            border_radius: None,
        })
        .insert(Player {
            state: PlayerState::Standing,
        });

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

// A simple camera system for moving and zooming the camera.
fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Player)>,
) {
    for (mut velocity, mut player) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }
    }
}
