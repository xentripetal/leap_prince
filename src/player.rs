use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
    let rigid_body = RigidBodyBundle {
        position: Vec2::new(8.0, 8.0).into(),
        body_type: RigidBodyType::KinematicVelocityBased,
        ..Default::default()
    };

    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(0.5, 0.5),
        material: ColliderMaterial {
            restitution: 0.7,
            ..Default::default()
        },
        flags: ColliderFlags::from(ActiveEvents::CONTACT_EVENTS),
        ..Default::default()
    };
    let texture_handle = asset_server.load("player.png");
    let sprite_bundle = SpriteBundle {
        material: materials.add(texture_handle.into()),
        transform: Transform::identity(),
        ..Default::default()
    };

    commands
        .spawn_bundle(sprite_bundle)
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Player {
            state: PlayerState::Standing,
        });

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

// A simple camera system for moving and zooming the camera.
fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut RigidBodyVelocity, &mut Player)>,
    mut collisions: EventReader<ContactEvent>,
) {
    for collision in collisions.iter() {
        info!("{:?}", collision);
    }
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

        velocity.linvel = (Vec2::new(direction.x, -9.8 * time.delta_seconds())).into();
    }
}
