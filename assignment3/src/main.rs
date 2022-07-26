use bevy::prelude::*;

// NOTE I think I am going to put this on hold for now. I think I got the general concept,
// and I think I am wasting more time on this than it is really worth.
// Perhaps I should come back after I learn more about rotation.


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(add_player)
        .add_system(move_player)
        .add_system(rotate_player)
        .add_system(local_to_global_transform)
        .run();
}

#[derive(Component)]
struct LocalToGlobalRect;

// Show origin of the screen for easier visualization
fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(4.0, 4.0)),
            ..Default::default()
        },
        ..Default::default()
    });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(LocalToGlobalRect);
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

    // update the player rotation around the Z axis (perpendicular to the 2D plane of the screen)
    let delta_time = time.delta_seconds();
    transform.rotate(Quat::from_rotation_z(
        rotation_factor * player.rotation_speed * delta_time,
    ));
}

// Find the global transform of the rectangle given the local transform from the player parent.
// The default transform is the transform relative to its parent position, so its local for the rect
fn local_to_global_transform(
    rect_query: Query<&Transform, With<Rect>>,
    player_query: Query<&Transform, (With<Player>, Without<Rect>)>,
    mut local_to_global_rect_query: Query<&mut Transform, (With<LocalToGlobalRect>, Without<Player>, Without<Rect>)>,
) {
    let rect_local_transform = rect_query.get_single().expect("Could not find single rect");

    let player_transform = player_query
        .get_single()
        .expect("Could not find a single player");

    let mut origin_to_rect_transform = local_to_global_rect_query
        .get_single_mut()
        .expect("Cannot find global to local rect");

    let x_vector = rect_local_transform.local_x() * rect_local_transform.translation.x;
    let y_vector = rect_local_transform.local_y() * rect_local_transform.translation.y;
    let rect_vector_from_player = x_vector + y_vector;

    let rect_vector_from_origin = rect_vector_from_player + player_transform.translation;

    // Put the rect at the midpoint because scaling a transform occures in the middle
    origin_to_rect_transform.translation.x = rect_vector_from_origin.x / 2.0;
    origin_to_rect_transform.translation.y = rect_vector_from_origin.y / 2.0;

    // TODO
    // I think something is wrong with the math, but I do think you are close.
    // I think you first need to rotate the vector so that it is always facing the rect
    let distance_vec_magnitude = origin_to_rect_transform.translation.length();
    origin_to_rect_transform.scale.y = distance_vec_magnitude / 2.0;

    origin_to_rect_transform.look_at(-Vec3::Z, Vec3::new(rect_vector_from_origin.x, rect_vector_from_origin.y, 0.0));
    println!("Scale of the vector {}", origin_to_rect_transform.scale.y);
}

// TODO
// 3. Draw a rect from the origin to the cube and
//    show the global coordinates of the vector using vector math.
