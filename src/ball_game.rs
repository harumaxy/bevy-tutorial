use std::unimplemented;

mod paddle;
use paddle::*;
mod ball;
use ball::*;
mod wall;
use wall::*;
mod goal;
use goal::*;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin},
    prelude::*,
    window::{self},
};
use window::WindowResized;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_system(spawn_ball.system())
        .add_system(ball_movement_system.system())
        .add_system(paddle_movement_system.system())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PrintDiagnosticsPlugin::default())
        // .add_system(print_window_descriptor.system())
        .add_system(window_resize_listenr.system())
        .add_system(ball_collision_system.system())
        .add_system(enter_goal.system())
        .run()
}

pub enum Collider {
    Paddle,
    Wall,
}

impl Default for Ball {
    fn default() -> Self {
        const DEFAULT_VELOCITY: f32 = 100.0;
        Self {
            direction: Vec2::new(1.0, 1.0).normalize(),
            speed: DEFAULT_VELOCITY,
        }
    }
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());
    spawn_ball(commands);
    spawn_paddle(commands, Player::Right, &mut materials);
    spawn_paddle(commands, Player::Left, &mut materials);
    spwan_goals(commands, &mut materials);
    spwan_walls(commands);
    commands.insert_resource(ClearColor(Color::BLACK)); // clear up Background Color (default = DARK_GRAY)
    commands.insert_resource(EventReader::<WindowResized>::default());
}

pub enum Player {
    Left,
    Right,
}

impl Player {
    fn start_position(&self) -> Vec2 {
        let window_width = 1280.;
        let x_position = match self {
            Player::Left => -window_width / 2.0 + Paddle::WIDTH,
            Player::Right => window_width / 2.0 - Paddle::WIDTH,
        };
        Vec2::new(x_position, 0.)
    }
    // return Up, Down keycode
    fn movement_keys(&self) -> (KeyCode, KeyCode) {
        match self {
            Player::Left => (KeyCode::W, KeyCode::S),
            Player::Right => (KeyCode::Up, KeyCode::Down),
        }
    }
}

fn window_resize_listenr(
    mut resize_listenr: ResMut<EventReader<WindowResized>>,
    resize_events: Res<Events<WindowResized>>,
    mut paddles: Query<(&mut Sprite, &mut Transform, &mut Paddle, &Player)>,
    mut ball: Query<(&mut Sprite, &mut Transform, &mut Ball)>,
    mut walls: Query<(&mut Sprite, &mut Transform, &mut Wall)>,
    mut goals: Query<(&mut Sprite, &mut Transform, &mut Goal)>,
) {
    for event in resize_listenr.iter(&resize_events) {
        for (mut sprite, mut transform, mut paddle, player) in paddles.iter_mut() {
            paddle.update_after_window_resize(
                player,
                event,
                &mut sprite.size,
                &mut transform.translation,
            );
        }
        for (mut sprite, mut transform, mut wall) in walls.iter_mut() {
            wall.update_after_window_resize(event, &mut sprite.size, &mut transform.translation)
        }
        for (mut sprite, mut transform, mut goal) in goals.iter_mut() {
            goal.update_after_window_resize(event, &mut sprite.size, &mut transform.translation)
        }

        for (mut sprite, mut transform, mut ball) in ball.iter_mut() {
            ball.update_after_window_resize(event, &mut sprite.size, &mut transform.translation)
        }
    }
}

// fn print_window_descriptor(window_descriptor: Res<WindowDescriptor>) {
//     // println!("{}", window_descriptor.width);
//     // println!("{}", window_descriptor.height);
// }
