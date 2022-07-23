use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_rectangle)
        .add_startup_system(add_moving_rectangle)
        .add_system(move_player)
        .add_system(check_if_player_in_square)
        .run();
}

#[derive(Component)]
struct Rect;

fn add_rectangle(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..Default::default()
            },
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

// Do some vector math to determine whether one object is inside another
fn check_if_player_in_square(
    player_query: Query<&Transform, (With<Player>, Without<Rect>)>,
    mut rect_query: Query<(&Transform, &mut Sprite), With<Rect>>,
) {
    let player_transform = player_query
        .get_single()
        .expect("Error: Could not find a single player.");

    let (rect_transform, mut rect_sprite) = rect_query
        .get_single_mut()
        .expect("Error: Could not find a single rect.");

    // Get the vector from the player to the rectangle
    let dist_p_to_r_vec: Vec3 = player_transform.translation - rect_transform.translation;

    // Do the pythagoreian theorum to get the magnitude or length of the vector
    let magnitude_dist = f32::sqrt(dist_p_to_r_vec.x.powf(2.0) + dist_p_to_r_vec.y.powf(2.0)).abs();

    // If the length of the vector is less than the the side of the square / 2, its inside the square
    // 100 / 2 for side of rect, really should be square
    if magnitude_dist < 50.0 {
        rect_sprite.color = Color::CYAN;
    } else {
        rect_sprite.color = Color::CRIMSON;
    }
}
