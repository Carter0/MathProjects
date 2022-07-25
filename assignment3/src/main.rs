use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(show_origin)
        .add_startup_system(add_rectangle)
        .add_startup_system(add_moving_rectangle)
        .add_system(move_player)
        .add_system(calculate_coordinates)
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

fn add_rectangle(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(-50.0, 100.0, 0.0)),
            ..Default::default()
        })
        .insert(Rect);
}

// The float value is the player movement speed in 'pixels/second'.
#[derive(Component)]
struct Player {
    pub speed: f32,
}

fn add_moving_rectangle(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(10.0, 10.0)),
                color: Color::ORANGE,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(120.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert(Player { speed: 300.0 });
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform)>,
    time: Res<Time>,
) {
    if let Ok((player, mut transform)) = player_query.get_single_mut() {
        // Get input from the keyboard (WASD)
        let up: bool = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
        let down: bool =
            keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
        let left: bool =
            keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let right: bool =
            keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

        // If left is pressed than it will be -1, right 1, both they cancel out.
        let x_axis: i8 = -(left as i8) + right as i8;
        let y_axis: i8 = -(down as i8) + up as i8;
        let move_delta: Vec2 = Vec2::new(x_axis as f32, y_axis as f32);

        // move the player
        let delta_time = time.delta_seconds();
        transform.translation.x += move_delta.x * player.speed * delta_time;
        transform.translation.y += move_delta.y * player.speed * delta_time;
    }
}

// Transform between local and global coordinate space
// TODO figure out how to do the rotation as well
fn calculate_coordinates(
    player_query: Query<&Transform, With<Player>>,
    rect_query: Query<&Transform, (With<Rect>, Without<Player>)>,
) {
    let player_transform = player_query
        .get_single()
        .expect("Error: Could not find a single player.");

    let rect_transform = rect_query
        .get_single()
        .expect("Error: Could not find a single rect.");

    println!("Global coordinates of player ({}, {})", player_transform.translation.x, player_transform.translation.y);

    let local_coords_relative_to_rect = rect_transform.translation - player_transform.translation;


    println!("Local coordinates of player to the rect ({}, {})", local_coords_relative_to_rect.x, local_coords_relative_to_rect.y);


    let back_to_global_transform = rect_transform.translation - local_coords_relative_to_rect;

    println!("Global coords of player again ({}, {})", back_to_global_transform.x, back_to_global_transform.y);
}
