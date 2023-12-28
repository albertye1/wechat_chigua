// use std::arch::x86_64::_SIDD_POSITIVE_POLARITY;

use bevy::math::quat;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::*;
use wechat_chigua::physics_engine::PhysicsEngine;
use wechat_chigua::vector2d::Vector2D;

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
const FIRST: usize = 6;
const FPS: f32 = 240.0;
const MULT: f32 = 4.0;
const EPS: f32 = 0.00001;
static FRUIT_SIZES: [f32; 11] = [3.0, 5.0, 7.0, 9.0, 11.0, 13.0, 15.0, 17.0, 19.0, 21.0, 23.0];
static FRUIT_COLORS: [Color; 11] = [
    Color::RED, // cherry
    Color::SALMON, // strawberry
    Color::PURPLE, // grape
    Color::GOLD, // orange
    Color::ORANGE, // persimmon
    Color::CRIMSON, // apple
    Color::LIME_GREEN, // pear
    Color::PINK, // peach
    Color::YELLOW, // pineapple
    Color::BEIGE, // cantaloupe
    Color::GREEN, // watermelon
];

#[derive(Component)]
struct Cursor(f32); // cursor with location
#[derive(Component)]
struct Fruit; // fruit
#[derive(Component)]
struct CursorFruit; // the fruit on the cursor right now
#[derive(Component)]
struct FallingFruit; // fruit being affected by gravity, on each tick
#[derive(Component)]
struct FruitSize(usize); // size of fruit
#[derive(Component)]
struct FruitID(u8);
#[derive(Component)]
struct FruitInfo(Vector2D, Vector2D); // position and velocity of the fruit.
#[derive(Resource)]
struct DropSound(Handle<AudioSource>);
#[derive(Resource)]
struct PhysicsTimer(Timer);

fn create_fruits(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    cursor: &Cursor,
) {
    let size: usize = get_rand(FIRST);
    let pos: Vector2D = Vector2D::new(cursor.0, CURSOR_Y);
    let vel: Vector2D = Vector2D::new(0.0, 0.0);
    // spawn a fruit, with the given size and initial position, at the cursor, and draw it according to given specs.
    commands.spawn((
        Fruit,
        FruitSize(size),
        FruitInfo(pos, vel),
        CursorFruit,
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Circle::new(MULT * FRUIT_SIZES[size]).into())
                .into(),
            material: materials.add(ColorMaterial::from(FRUIT_COLORS[size])),
            transform: Transform::from_translation(Vec3::new(cursor.0, CURSOR_Y, 0.)),
            ..default()
        },
    ));
    println!("size: {}", size);
}

// when prompted by key, drop the fruit
fn drop_fruit(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    c_query: Query<&Cursor>,
    mut cf_query: Query<(Entity, &FruitSize, &FruitInfo), With<CursorFruit>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // requires physics be implemented first
    if keys.just_pressed(KeyCode::Space) {
        if let Ok(mut cursor_fruit) = cf_query.get_single_mut() {
            let size: usize = cursor_fruit.1 .0;
            let pos = cursor_fruit.2 .0;
            let vel = cursor_fruit.2 .1;
            commands.spawn((
                Fruit,
                FruitSize(size),
                FruitInfo(pos, vel),
                FallingFruit,
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::Circle::new(MULT * FRUIT_SIZES[size]).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(FRUIT_COLORS[size])),
                    transform: Transform::from_translation(Vec3::new(pos.x(), pos.y(), 0.)),
                    ..default()
                },
            ));
            commands.entity(cursor_fruit.0).despawn();
        }
        if let Ok(cursor) = c_query.get_single() {
            create_fruits(commands, meshes, materials, cursor);
        }
    }
}

// check all pairs of fruits for any collisions
fn update_colliding(
    time: Res<Time>,
    mut timer: ResMut<PhysicsTimer>,
    mut f_query: Query<(&mut FruitInfo, &mut Transform, &mut FruitSize), With<FallingFruit>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut i = 0;
        // for each fruit:
        for mut fruit in &mut f_query {
            // store positions in easily accessible form
            let mut pos = fruit.0 .0;
            let mut vel = fruit.0 .1;
            let radius = MULT * FRUIT_SIZES[fruit.2 .0] as f32;

            // check for wall collisions
            if pos.x() - radius < L_WALL {
                pos.set_x(L_WALL + radius);
                vel.set_x(vel.x().abs() * 0.7);
            }
            if pos.x() + radius > R_WALL {
                pos.set_x(R_WALL - radius);
                vel.set_x(-vel.x().abs() * 0.7);
            }
            if pos.y() - radius < B_WALL {
                pos.set_y(B_WALL + radius);
                // vel.set_y(vel.y().abs() * 0.7);
                vel.set_y(0.0);
                vel.set_x(vel.x() * 0.99); // friction
            }
            // apply velocity, then apply gravity
            PhysicsEngine::fall(&mut pos, &mut vel);

            // apply transform changes
            fruit.0 .0 = pos;
            fruit.0 .1 = vel;
            fruit.1.translation.x = fruit.0 .0.x();
            fruit.1.translation.y = fruit.0 .0.y();
        }
        for i in 1..4 {
            let mut combinations = f_query.iter_combinations_mut();
            while let Some([mut fruit, mut other]) = combinations.fetch_next() {
                let mut pos = fruit.0 .0;
                let mut vel = fruit.0 .1;
                let radius = MULT * FRUIT_SIZES[fruit.2 .0] as f32;
                let mut pos2 = other.0 .0;
                let mut vel2 = other.0 .1;
                let radius2 = MULT * FRUIT_SIZES[other.2 .0] as f32;
                PhysicsEngine::collide(&mut pos, &mut vel, radius, &mut pos2, &mut vel2, radius2);
                // update with values gotten from the physics engine
                other.0 .0 = pos2;
                other.0 .1 = vel2;
                other.1.translation.x = other.0 .0.x();
                other.1.translation.y = other.0 .0.y();
                fruit.0 .0 = pos;
                fruit.0 .1 = vel;
                fruit.1.translation.x = fruit.0 .0.x();
                fruit.1.translation.y = fruit.0 .0.y();
            }
        }
    }
}

// friction -- prevents the fruits from rolling continuously
fn friction(
    time: Res<Time>,
    mut timer: ResMut<PhysicsTimer>,
    mut f_query: Query<(&mut FruitInfo, &mut FruitSize), With<FallingFruit>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut fruit in &mut f_query {
            let vel = fruit.0 .1.x();
            if fruit.0 .1.y() < EPS {
                // fruit.0.1.set_x(vel * 0.9);
            }
        }
    }
}

// each time we run this we check for 2 circles and replace them with a larger item centered.
fn check_merges(
    mut commands: Commands,
    mut f_query: Query<(Entity, &mut FruitInfo, &mut FruitSize), With<FallingFruit>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut has_uncollided = true;
    let mut combinations = f_query.iter_combinations_mut();
    while let Some([mut fruit, mut other]) = combinations.fetch_next() {
        if fruit.2.0 != other.2.0 || fruit.2.0 == 10 {
            continue;
        }
        let mut pos1 = fruit.1.0;
        let radius = MULT * FRUIT_SIZES[fruit.2.0] as f32;
        let mut pos2 = other.1.0;
        let depth = 2.0 * radius - (pos2 - pos1).magnitude();
        if depth >= 0.0 {
            // we have a collision -- now merge!
            has_uncollided = false;
            let size = fruit.2.0 + 1;
            let pos = (pos1 + pos2) / 2.0;
            commands.spawn((
                Fruit,
                FruitSize(size),
                FruitInfo(pos, Vector2D::new(0.0,0.0)),
                FallingFruit,
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::Circle::new(MULT * FRUIT_SIZES[size]).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(FRUIT_COLORS[size])),
                    transform: Transform::from_translation(Vec3::new(pos.x(), pos.y(), 0.)),
                    ..default()
                },
            ));
            
            commands.entity(fruit.0).despawn();
            commands.entity(other.0).despawn();
            break;
        }
    }
}

// move cursor and associated fruit
fn move_cursor(
    keys: Res<Input<KeyCode>>,
    mut c_query: Query<&mut Cursor>,
    mut t_query: Query<&mut Transform, With<CursorFruit>>,
    mut p_query: Query<&mut FruitInfo, With<CursorFruit>>,
) {
    if let Ok(mut cursor) = c_query.get_single_mut() {
        if keys.pressed(KeyCode::Left) {
            cursor.0 = L_WALL.max(cursor.0 - CURSOR_STEP);
            // println!("{}", cursor.0);
        } else if keys.pressed(KeyCode::Right) {
            // move cursor right
            cursor.0 = R_WALL.min(cursor.0 + CURSOR_STEP);
            // println!("{}", cursor.0);
        } else {
            return;
        }
    }

    if let Ok(cursor) = c_query.get_single() {
        if let Ok(mut trans) = t_query.get_single_mut() {
            trans.translation.x = cursor.0;
        }

        if let Ok(mut info) = p_query.get_single_mut() {
            let mut pos = info.0;
            // println!("{}, {}", pos.x(), pos.y());
            pos.set_x(cursor.0);
            // println!("{}, {}", pos.x(), pos.y());
            info.0 = pos;
        }
    }
}

// to be implemented -- spawn a game over screen if any fruit reaches height of U_WALL or greater.
fn death_check() {

}

fn startup_sequence(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let engine = PhysicsEngine::new();
    engine.test();
    commands.spawn(Cursor((L_WALL + R_WALL) / 2.0));
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
    create_fruits(
        commands,
        meshes,
        materials,
        &Cursor((L_WALL + R_WALL) / 2.0),
    );
}

fn get_rand(n: usize) -> usize {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    (x * (n as f64)).floor() as usize
}

pub struct InitialPlugin;

impl Plugin for InitialPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_sequence);
    }
}

fn main() {
    App::new()
        .insert_resource(PhysicsTimer(Timer::from_seconds(
            1.0 / FPS,
            TimerMode::Repeating,
        )))
        .add_plugins((DefaultPlugins, InitialPlugin))
        .add_systems(
            Update,
            (move_cursor, drop_fruit, check_merges, update_colliding, friction),
        )
        .run();
}
