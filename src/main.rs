//#![allow(unused)]

use bevy::prelude::*;
use bevy::window::WindowResizeConstraints;
//use bevy::core::FixedTimestep;
//use bevy_egui::{egui, EguiContext};
//use rand::Rng;

#[derive(Component)]
struct Person;
#[derive(Component)]
struct Name(String);
#[derive(Component)]
struct SnakeHead;
#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_people).add_system(greet_people);
    }
}

const TITLE: &str = "たいとる";
const DEFAULT_WIDTH: f32 = 860.0;
const DEFAULT_HEIGHT: f32 = 512.0;
const WINDOW_SIZE_CONSTRAINTS: WindowResizeConstraints = WindowResizeConstraints {
    min_width: 64.0,
    min_height: 64.0,
    max_width: 3072.0,
    max_height: 3072.0,
};
const SNAKE_HEAD_COLOR: Color = Color::rgb(0.5, 0.6, 0.4);
const BACK_COLOR: Color = Color::rgb(1.0, 0.9, 0.7);

fn main() {
    App::new()
        .add_startup_system(setup_camera)
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_snake)
        .add_system(snake_movement)
        .insert_resource(ClearColor(BACK_COLOR))
        .insert_resource(WindowDescriptor {
            title: TITLE.to_string(),
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            resize_constraints: WINDOW_SIZE_CONSTRAINTS,
            ..Default::default()
        })
        .run();
}

//fn setup(mut commands: Commands) {commands.spawn_bundle(OrthographicCameraBundle::new_2d());}

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("もるか".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("てつ".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("りんりん".to_string()));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("ようこそ {}", name.0); //< ようこそ もるか\nようこそ てつ\nようこそ りんりん
    }
}

// fn test_ui(egui_context: ResMut<EguiContext>) {egui::Window::new("Hello").show(egui_context.ctx(), |ui| {ui.label("world");});}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_snake(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(48.0, 48.0, 48.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SnakeHead);
}

fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<&mut Transform, With<SnakeHead>>,
) {
    let dash = if keyboard_input.pressed(KeyCode::LControl) {
        2.2
    } else {
        1.
    }; // ダッシュ(Ctrl)時の移動速度乗数
    let speed = 3.6 * dash;
    for mut pos in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            pos.translation.x -= speed;
        }
        if keyboard_input.pressed(KeyCode::D) {
            pos.translation.x += speed;
        }
        if keyboard_input.pressed(KeyCode::S) {
            pos.translation.y -= speed * 0.75;
        }
        if keyboard_input.pressed(KeyCode::W) {
            pos.translation.y += speed;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            pos.rotate(rotate_quat(Direction::Right));
        }
        if keyboard_input.pressed(KeyCode::Left) {
            pos.rotate(rotate_quat(Direction::Left));
        }
        if keyboard_input.pressed(KeyCode::Space) {
            pos.translation.x = 0.;
            pos.translation.y = 0.;
            pos.rotation = rotate_quat(Direction::Init);
        }
    }
}

enum Direction {
    Right,
    Left,
    Init,
}

fn rotate_quat(rd: Direction) -> Quat {
    let ag_f = 360. / 6144.;
    let angle = match rd {
        Direction::Right => -ag_f,
        Direction::Left => ag_f,
        Direction::Init => 0.,
    };
    Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle)
}