use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::*;

// width and height of the game area
const WIDTH: usize = 83;
const HEIGHT: usize = 150;
const CURSOR_Y: usize = 10 + HEIGHT;
const FIRST: usize = 5;

#[derive(Component)]
struct Cursor(usize); // cursor with initial location
#[derive(Component)]
struct Fruit; // fruit
#[derive(Component)]
struct FruitSize(usize); // size of fruit
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
    commands.spawn(Cursor(WIDTH / 2));
    commands.spawn((Fruit, FruitSize(get_rand(FIRST))));
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
        ..default()
    });
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
