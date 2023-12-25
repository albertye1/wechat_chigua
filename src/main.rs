use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::*;

// width and height of the game area
const WIDTH: f32 = 150.0;
const HEIGHT: f32 = 300.0;
const CURSOR_Y: f32 = 10.0 + HEIGHT;
const FIRST: usize = 5;
const MULT: f32 = 4.0;
static FRUIT_SIZES: [f32; 9] = [1.0, 3.0, 5.0, 7.0, 9.0, 11.0, 13.0, 15.0, 17.0];

#[derive(Component)]
struct Cursor(f32); // cursor with initial location
#[derive(Component)]
struct Fruit; // fruit
#[derive(Component)]
struct FruitSize(usize); // size of fruit
#[derive(Component)]
struct FruitPos(f32, f32);
#[derive(Resource)]
struct DropSound(Handle<AudioSource>);

fn startup_sequence(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>, 
    asset_server: Res<AssetServer>) {
    // make a canvas
    commands.spawn(Camera2dBundle::default());
    // we can import the bgm as well as the dropping and collision noises here.
    let drop_sound = asset_server.load("placeholder.wav"); // just a placeholder until i actually get this figured out
    commands.insert_resource(DropSound(drop_sound));
    // spawn a cloud (cursor) 
    // put a fruit at the location of the cloud
    commands.spawn(Cursor(WIDTH / 2.0));
    let size: usize = get_rand(FIRST);
    commands.spawn((Fruit, FruitSize(size)));
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(MULT * FRUIT_SIZES[size]).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(WIDTH / 2.0, CURSOR_Y, 0.)),
        ..default()
    });
    println!("size: {}", size);
}

fn get_rand(n: usize) -> usize {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    (x * (n as f64)).floor() as usize
}

fn print_fruits(query: Query<&FruitSize, With<Fruit>>) {
    for fruit in &query {
        println!("{}", fruit.0);
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
        .run();
}
