use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const TAU: f32 = 6.283185;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(draw_regular_polygons)
        .run()
}

// Draw a regular polygon
// A regular polygon has the same side length and the same angle between all the sides
// Draw the polygon by calculating the angle between all the points with TAU / the number of points.
// Then calculate where to put the points by using from_angle and multiplying the angle by 1 -> num of points.
fn draw_regular_polygons(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    let num_points = 5;
    let angle_between_points = TAU / num_points as f32;

    for point in 1..=num_points {
        let vec = Vec2::from_angle(point as f32 * angle_between_points) * 50.0;

        // Circle
        commands.spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(vec.extend(0.0)),
            ..default()
        });
    }
}
