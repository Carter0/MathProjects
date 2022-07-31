use bevy::prelude::*;


// TODO come back when you can draw a vector that rotates across the screen for easier visualization
// But still, I got it right

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(show_origin)
        .add_startup_system(add_rectangle)
        .add_startup_system(add_moving_rectangle)
        .add_system(move_player)
        .add_system(calculate_if_player_facing_rect)
        .run();
}


// Show origin of the screen for easier visualization
fn show_origin(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn_bundle(SpriteBundle {
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

fn calculate_if_player_facing_rect(
    player_query: Query<&Transform, (With<Player>, Without<Rect>)>,
    mut rect_query: Query<(&Transform, &mut Sprite), With<Rect>>,
) {
    let player_transform = player_query
        .get_single()
        .expect("Error: Could not find a single player.");

    let (rect_transform, mut rect_sprite) = rect_query
        .get_single_mut()
        .expect("Error: Could not find a single rect.");

    // Make sure both vectors are normalized so the dot product is between 0 and 1/-1.
    let normalized_player_translation = player_transform.translation.normalize_or_zero();
    let normalized_rect_translation = rect_transform.translation.normalize_or_zero();

    let dot_product = normalized_player_translation.x * normalized_rect_translation.x
        + normalized_player_translation.y * normalized_rect_translation.y;

    // Rect color gets darker the less the player faces the rect
    // And the rect color gets bright the more the player faces the rect
    println!("Dot product is {}", dot_product);
    rect_sprite.color = Color::rgb(dot_product, dot_product, dot_product);
}
