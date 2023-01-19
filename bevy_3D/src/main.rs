use bevy::prelude::*;
use rand::Rng;

const TIME_STEP: f32 = 1.0 / 60.0;

const GRAVITY: f32 = -15.0;
const MULT: f32 = 1.0;

const CUBE_COLOUR: Color = Color::rgb(1.0, 1.0, 1.0);
const CUBE_STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.5, 0.0);
const CUBE_SIZE: Vec3 = Vec3::new(1.0, 1.0, 1.0);
const CUBE_MAX_VEL: f32 = 25.0;
const CUBE_MAX_ROT: f32 = 150.0;

const CAMERA_STARTING_POSITION: Vec3 = Vec3::new(-10.0, 5.0, 12.5);

const AMBIENT_COLOUR: &str = "581C87";

#[derive(Component)]
struct StartingBox;
#[derive(Component)]
struct Cube;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec3);
#[derive(Component, Deref, DerefMut)]
struct Rotation(Vec3);

fn main() {
    println!("Starting...");
    App::new()
        .add_plugins(DefaultPlugins)
        //.add_startup_system(start)
        //.add_startup_system(spawn_lights)
        .add_startup_system_set(
            SystemSet::new()
                .with_system(start)
                .with_system(spawn_lights)
        )
        .add_system_set(
            SystemSet::new()
                .with_system(apply_velocity)
                .with_system(apply_rotation)
                .with_system(move_cube.before(apply_velocity))
                .with_system(explode),
        )
        .run()
}

fn start(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // spawn starting box
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube{size: 1.0})),
            material: materials.add(CUBE_COLOUR.into()),
            transform: Transform::from_translation(CUBE_STARTING_POSITION),
            ..default()
        },
        StartingBox
    ));

    // spawn camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(CAMERA_STARTING_POSITION).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_lights(
    mut commands: Commands,
) {
    // ambient
    commands.insert_resource(AmbientLight {
        color: Color::hex(AMBIENT_COLOUR).unwrap(),
        brightness: 0.6,
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 150000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });
}

fn explode(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keys: Res<Input<KeyCode>>,
    query: Query<Entity, With<StartingBox>>
) {
    if keys.just_pressed(KeyCode::E) {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        for _i in 0..25 {
            // random velocity
            let mut rng  = rand::thread_rng();
            let vel_num: f32 = rng.gen_range(0.0..CUBE_MAX_VEL);
            let x: f32 = rng.gen_range(-1.0..1.0);
            let y: f32 = rng.gen_range(-1.0..1.0);
            let z: f32 = rng.gen_range(-1.0..1.0);
            let dir: Vec3 = Vec3::new(x, y, z).normalize();
    
            // random size
            let size_mult: f32 = rng.gen_range(0.0..1.0);
    
            // random rotation
            let rot_num: f32 = rng.gen_range(0.0..CUBE_MAX_ROT);
            let x: f32 = rng.gen_range(-1.0..1.0);
            let y: f32 = rng.gen_range(-1.0..1.0);
            let z: f32 = rng.gen_range(-1.0..1.0);
            let rot: Vec3 = Vec3::new(x, y, z).normalize();
            // spawn cube
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube{size: 1.0})),
                    material: materials.add(CUBE_COLOUR.into()),
                    transform: Transform::from_translation(CUBE_STARTING_POSITION)
                        .with_scale(CUBE_SIZE*size_mult),
                    ..default()
                },
                Cube,
                Velocity(dir*vel_num),
                Rotation(rot*rot_num)
            ));
        }
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * TIME_STEP * MULT;
        transform.translation.y += velocity.y * TIME_STEP * MULT;
        transform.translation.z += velocity.z * TIME_STEP * MULT;
    }
}

fn apply_rotation(mut query: Query<(&mut Transform, &Rotation)>, time: Res<Time>) {
    for (mut transform, rotation) in &mut query {
        transform.rotate_x(time.delta_seconds() * TIME_STEP * rotation.x);
        transform.rotate_y(time.delta_seconds() * TIME_STEP * rotation.y);
        transform.rotate_z(time.delta_seconds() * TIME_STEP * rotation.z);
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