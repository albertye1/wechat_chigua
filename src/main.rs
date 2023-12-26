use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use wechat_chigua::physics_engine::PhysicsEngine;
use rand::prelude::*;

// game dimensions
const WIDTH: f32 = 450.0;
const HEIGHT: f32 = 300.0; // update to be window size or smth?
// walls
const U_WALL: f32 = 300.0;
const L_WALL: f32 = -500.0;
const B_WALL: f32 = -300.0;
const R_WALL: f32 = 10.0;
const LINE_WIDTH: f32 = 3.0;
// cursor params
const CURSOR_Y: f32 = 10.0 + HEIGHT;
const CURSOR_STEP: f32 = 2.5;
// other constants, categorize later
const FIRST: usize = 5;
const MULT: f32 = 4.0;
static FRUIT_SIZES: [f32; 9] = [1.0, 3.0, 5.0, 7.0, 9.0, 11.0, 13.0, 15.0, 17.0];

#[derive(Component)]
struct Cursor(f32); // cursor with location
#[derive(Component)]
struct Fruit; // fruit
#[derive(Component)]
struct CursorFruit; // the fruit on the cursor right now
#[derive(Component)]
struct FruitSize(usize); // size of fruit
#[derive(Component)]
struct FruitPos(f32, f32);
#[derive(Resource)]
struct DropSound(Handle<AudioSource>);

fn create_fruits(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let size: usize = get_rand(FIRST);
    let mid: f32 = (L_WALL + R_WALL) / 2.0;
    commands.spawn(Cursor(mid));
    // spawn a fruit, with the given size and initial position, at the cursor, and draw it according to given specs.
    commands.spawn((Fruit, FruitSize(size), FruitPos(mid, CURSOR_Y), CursorFruit, MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Circle::new(MULT * FRUIT_SIZES[size]).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        transform: Transform::from_translation(Vec3::new(mid, CURSOR_Y, 0.)),
        ..default()
    }));
    println!("size: {}", size);
}

// when prompted by key, drop the fruit
fn drop_fruit() {
    // requires physics be implemented first
}

// merge two fruits and re-center, updating the physics engine after?
fn merge_fruits() {}

fn move_cursor(keys: Res<Input<KeyCode>>, mut c_query: Query<&mut Cursor>, mut t_query: Query<&mut Transform, With<CursorFruit>>) {
    if let Ok(mut cursor) = c_query.get_single_mut() {
        if keys.pressed(KeyCode::Left) {
            cursor.0 = L_WALL.max(cursor.0 - CURSOR_STEP);
            println!("{}", cursor.0);
        }
        if keys.pressed(KeyCode::Right) {
            // move cursor right
            cursor.0 = R_WALL.min(cursor.0 + CURSOR_STEP);
            println!("{}", cursor.0);
        }
    }

    if let Ok(cursor) = c_query.get_single() {
        if let Ok(mut trans) = t_query.get_single_mut() {
            trans.translation.x = cursor.0;
        }
    }
}

fn startup_sequence(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let engine = PhysicsEngine::new();
    engine.test();
    // make a canvas
    commands.spawn(Camera2dBundle::default());
    // we can import the bgm as well as the dropping and collision noises here.
    let drop_sound = asset_server.load("placeholder.wav"); // just a placeholder until i actually get this figured out
    commands.insert_resource(DropSound(drop_sound));
    // build walls
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(LINE_WIDTH, U_WALL - B_WALL)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        transform: Transform::from_translation(Vec3::new(R_WALL, 0., 0.)),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(LINE_WIDTH, U_WALL - B_WALL)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        transform: Transform::from_translation(Vec3::new(L_WALL, 0., 0.)),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(R_WALL - L_WALL, LINE_WIDTH)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::WHITE)),
        transform: Transform::from_translation(Vec3::new((L_WALL + R_WALL) / 2.0, B_WALL, 0.)),
        ..default()
    });
    // spawn a cloud (cursor)
    // put a fruit at the location of the cloud
    create_fruits(commands, meshes, materials);
}

fn get_rand(n: usize) -> usize {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    (x * (n as f64)).floor() as usize
}

fn print_fruits(query: Query<&FruitPos, With<Fruit>>) {
    for fruit in &query {
        println!("{}, {}", fruit.0, fruit.1);
    }
}

pub struct InitialPlugin;

impl Plugin for InitialPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_sequence);
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, InitialPlugin))
        .add_systems(Update, move_cursor)
        .run();
}
