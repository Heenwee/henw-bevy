use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    time::FixedTimestep,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};
use rand::Rng;

const TIME_STEP: f32 = 1.0 / 60.0;

const GRAVITY: f32 = -9.81;
const MULT: f32 = 25.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);

const CUBE_COLOUR: Color = Color::rgb(1.0, 1.0, 1.0);
const CUBE_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const CUBE_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);

#[derive(Component)]
struct Cube;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);
#[derive(Component)]
struct Reloadable;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(start)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(apply_velocity)
                .with_system(move_cube.before(apply_velocity))
                .with_system(despawn_entities),
        )
        //.add_system_set(
        //    SystemSet
        //)
        .run();
}

fn start(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    //asset_server: Res<AssetServer>,
) {
    println!("Starting...");

    //spawn camera
    commands.spawn(Camera2dBundle::default());

    // spawn boxes

    for _i in 0..100 {
        let mut rng  = rand::thread_rng();
        let num: f32 = rng.gen_range(0.0..50.0);
        let x: f32 = rng.gen_range(-1.0..1.0);
        let y: f32 = rng.gen_range(-1.0..1.0);
        let dir: Vec2 = Vec2::new(x, y).normalize();

        let size_mult: f32 = rng.gen_range(0.0..1.0);

        let rot: f32 = rng.gen_range(0.0..360.0);

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Cube::default().into()).into(),
                material: materials.add(ColorMaterial::from(CUBE_COLOUR)),
                transform: Transform::from_translation(CUBE_STARTING_POSITION)
                    .with_scale(CUBE_SIZE*size_mult)
                    .with_rotation(Quat::from_axis_angle(
                        Vec3::new(0.0, 0.0, 1.0),
                        rot*180.0/std::f32::consts::PI
                    )),
                ..default()
            },
            Cube,
            //Velocity(Vec2::new(0.0,0.0)),
            Velocity(dir*num),
            Reloadable
        ));
    }

}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * TIME_STEP * MULT;
        transform.translation.y += velocity.y * TIME_STEP * MULT;
    }
}

fn move_cube(
    mut cube_query: Query<&mut Velocity, With<Cube>>,
) {
    if !cube_query.is_empty() {
        for mut cube_vel in &mut cube_query {
            cube_vel.y += GRAVITY*TIME_STEP;
        }
    }
}

fn despawn_entities(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<Entity, With<Reloadable>>
) {
    if keyboard_input.pressed(KeyCode::R) {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}