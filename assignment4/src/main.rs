use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const WINDOWHEIGHT: f32 = 1000.0;
const WINDOWWIDTH: f32 = 1200.0;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "assignment 4".to_string(),
            width: WINDOWWIDTH,
            height: WINDOWHEIGHT,
            ..Default::default()
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(add_player)
        .add_system(move_player)
        .add_system(rotate_player)
        .add_system(cast_ray)
        .run();
}

// The float value is the player movement speed in 'pixels/second'.
#[derive(Component)]
struct Player {
    speed: f32,
    rotation_speed: f32,
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(4.0, 4.0)),
            ..Default::default()
        },
        ..Default::default()
    });

    // The ceiling
    let ceiling_size_x = WINDOWWIDTH;
    let ceiling_size_y = 40.0;

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(10.0, 70.0, 70.0),
                custom_size: Some(Vec2::new(ceiling_size_x, ceiling_size_y)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, WINDOWHEIGHT / 2.0, 1.0),
            ..Default::default()
        })
        .insert(Collider::cuboid(
            ceiling_size_x / 2.0,
            ceiling_size_y / 2.0,
        ));

    // The floor
    let floor_size_x = WINDOWWIDTH;
    let floor_size_y = 40.0;

    commands.spawn().insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(10.0, 70.0, 70.0),
            custom_size: Some(Vec2::new(floor_size_x, floor_size_y)),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, -WINDOWHEIGHT / 2.0, 1.0),
        ..Default::default()
    });

    // The Left Wall
    let left_wall_size_x = 40.0;
    let left_wall_size_y = WINDOWHEIGHT;

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(10.0, 70.0, 70.0),
                custom_size: Some(Vec2::new(left_wall_size_x, left_wall_size_y)),
                ..Default::default()
            },
            transform: Transform::from_xyz(-WINDOWWIDTH / 2.0, 0.0, 1.0),
            ..Default::default()
        })
        .insert(Collider::cuboid(
            left_wall_size_x / 2.0,
            left_wall_size_y / 2.0,
        ));

    // The Right Wall
    let right_wall_size_x = 40.0;
    let right_wall_size_y = WINDOWHEIGHT;

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(10.0, 70.0, 70.0),
                custom_size: Some(Vec2::new(right_wall_size_x, right_wall_size_y)),
                ..Default::default()
            },
            transform: Transform::from_xyz(WINDOWWIDTH / 2.0, 0.0, 1.0),
            ..Default::default()
        })
        .insert(Collider::cuboid(
            right_wall_size_x / 2.0,
            right_wall_size_y / 2.0,
        ));
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
        .insert(Collider::cuboid(30.0 / 2.0, 30.0 / 2.0));
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

fn cast_ray(
    rapier_context: Res<RapierContext>,
    player_query: Query<(&Transform, Entity), With<Player>>,
) {
    let (transform, entity) = player_query
        .get_single()
        .expect("Could not find a single player");

    println!("player id is {:?}", entity);

    // TODO you almost got it (I think), you dealing with spaces, with is confusing you.
    let ray_pos = Vec2::new(transform.translation.x, transform.translation.y + 17.0);
    let ray_dir = Vec2::new(0.0, 1.0);
    let max_toi = 80.0;
    let solid = false;
    let filter = QueryFilter::default();
    if let Some((entity, intersection)) =
        rapier_context.cast_ray_and_get_normal(ray_pos, ray_dir, max_toi, solid, filter)
    {
        // This is similar to `QueryPipeline::cast_ray` illustrated above except
        // that it also returns the normal of the collider shape at the hit point.
        let hit_point = intersection.point;
        let hit_normal = intersection.normal;
        println!(
            "Entity {:?} hit at point {} with normal {}",
            entity, hit_point, hit_normal
        );
    }
}
