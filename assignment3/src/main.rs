use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(show_origin)
        .add_startup_system(add_player)
        .add_system(move_player)
        .add_system(rotate_player)
        .run();
}

// Show origin of the screen for easier visualization
fn show_origin(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(4.0, 4.0)),
            ..Default::default()
        },
        ..Default::default()
    });
}

#[derive(Component)]
struct Rect;

// The float value is the player movement speed in 'pixels/second'.
#[derive(Component)]
struct Player {
    speed: f32,
    rotation_speed: f32,
}

fn add_player(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(30.0, 30.0)),
                color: Color::ORANGE,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(120.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert(Player {
            speed: 300.0,
            // degrees per second
            rotation_speed: f32::to_radians(360.0),
        })
        .with_children(|player| {
            // child cube
            player
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(10.0, 10.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new(-50.0, 100.0, 0.0)),
                    ..Default::default()
                })
                .insert(Rect);
        });
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform)>,
    time: Res<Time>,
) {
    let (player, mut transform) = player_query
        .get_single_mut()
        .expect("Could not find single player");

    // Get input from the keyboard (WASD)
    let up: bool = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
    let down: bool = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
    let left: bool = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
    let right: bool = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

    // If left is pressed than it will be -1, right 1, both they cancel out.
    let x_axis: i8 = -(left as i8) + right as i8;
    let y_axis: i8 = -(down as i8) + up as i8;
    let move_delta: Vec2 = Vec2::new(x_axis as f32, y_axis as f32);

    // move the player
    let delta_time = time.delta_seconds();
    transform.translation.x += move_delta.x * player.speed * delta_time;
    transform.translation.y += move_delta.y * player.speed * delta_time;
}

fn rotate_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform)>,
    time: Res<Time>,
) {
    let (player, mut transform) = player_query
        .get_single_mut()
        .expect("Could not find a single player");

    let mut rotation_factor = 0.0;

    if keyboard_input.pressed(KeyCode::J) {
        rotation_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::K) {
        rotation_factor -= 1.0;
    }

    println!("rotation factor is {}", rotation_factor);

    // update the player rotation around the Z axis (perpendicular to the 2D plane of the screen)
    let delta_time = time.delta_seconds();
    transform.rotate(Quat::from_rotation_z(
        rotation_factor * player.rotation_speed * delta_time,
    ));
}

// TODO
// 3. Draw a rect from the origin to the cube and
//    show the global coordinates of the vector using vector math.
