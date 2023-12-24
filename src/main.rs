use bevy::prelude::*;
use rand::prelude::*;

const WIDTH: usize = 83;
const HEIGHT: usize = 150;
const FIRST: usize = 5;

#[derive(Component)]
struct Fruit;
#[derive(Component)]
struct FruitSize(usize);

fn startup_sequence(mut commands: Commands) {
    // make a canvas
    // spawn a cloud (cursor) 
    // put a fruit at the location of the cloud
    commands.spawn((Fruit, FruitSize(get_rand(FIRST))));
}

fn get_rand(n: usize) -> usize {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    (x * (n as f64)).floor() as usize
}

fn print_rand() {
    println!("{}", get_rand(5));
}

fn main() {
    App::new()
        .add_systems(Startup, startup_sequence)
        .add_systems(Update, print_rand)
        .run();
}
