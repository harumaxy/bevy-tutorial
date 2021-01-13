use std::unimplemented;

mod paddle;
use paddle::*;

mod ball;
use ball::*;

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    window::{self, WindowMode},
};
use window::WindowResized;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_system(spawn_ball.system())
        .add_system(ball_movement_system.system())
        .add_system(paddle_movement_system.system())
        .add_system(print_window_descriptor.system())
        .add_system(window_resize_listenr.system())
        .add_system(ball_collision_system.system())
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

fn setup(commands: &mut Commands) {
    commands.spawn(Camera2dBundle::default());
    spawn_ball(commands);
    spawn_paddle(commands, Player::Right);
    spawn_paddle(commands, Player::Left);
    commands.insert_resource(ClearColor(Color::BLACK)); // clear up Background Color (default = DARK_GRAY)
    commands.insert_resource(WindowDescriptor {
        title: "pong clone".to_string(),
        width: 1280.,
        height: 720.,
        vsync: true,
        resizable: true,
        decorations: true,
        cursor_locked: false,
        cursor_visible: true,
        mode: WindowMode::Windowed,
        #[cfg(target_arch = "wasm32")]
        canvas: None,
    });
    commands.insert_resource(EventReader::<WindowResized>::default());
}

fn spawn_ball(commands: &mut Commands) {
    const SIZE: f32 = 50.0;

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                size: Vec2::new(SIZE, SIZE),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, -100.0, 0.0),
                rotation: Quat::from_rotation_z(std::f32::consts::PI / 4.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Ball::default());
}

pub enum Player {
    Left,
    Right,
}

impl Player {
    fn start_position(&self) -> Vec2 {
        let x_position = match self {
            Player::Left => -500.,
            Player::Right => 500.,
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

type PaddleBundle = (SpriteBundle, Paddle, Player, Collider);

fn spawn_paddle(commands: &mut Commands, player: Player) {
    let (sprite_bundle, paddle, player, collider): PaddleBundle = (
        SpriteBundle {
            sprite: Sprite {
                size: Vec2::new(20., 200.),
                ..Default::default()
            },
            transform: Transform {
                translation: player.start_position().extend(0.),
                ..Default::default()
            },
            ..Default::default()
        },
        Paddle { speed: 1280. / 3. },
        player,
        Collider::Paddle,
    );
    commands
        .spawn((paddle, player, collider))
        .with_bundle(sprite_bundle);
}

fn window_resize_listenr(
    mut resize_listenr: ResMut<EventReader<WindowResized>>,
    resize_events: Res<Events<WindowResized>>,
    mut paddles: Query<(&mut Sprite, &mut Transform, &mut Paddle, &Player)>,
    mut ball: Query<(&mut Sprite, &mut Transform, &mut Ball)>,
) {
    resize_listenr.latest(&resize_events).map(|event| {
        for (mut sprite, mut transform, mut paddle, player) in paddles.iter_mut() {
            paddle.update_after_window_resize(
                player,
                event,
                &mut sprite.size,
                &mut transform.translation,
            );
        }
        for (mut sprite, mut transform, mut ball) in ball.iter_mut() {
            ball.update_after_window_resize(event, &mut sprite.size, &mut transform.translation)
        }
    });
}

fn print_window_descriptor(window_descriptor: Res<WindowDescriptor>) {
    // println!("{}", window_descriptor.width);
    // println!("{}", window_descriptor.height);
}
