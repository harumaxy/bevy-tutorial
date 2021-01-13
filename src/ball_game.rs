use std::unimplemented;

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

struct Ball {
    velocity: Vec2,
}
enum Collider {
    Paddle,
    Wall,
}

impl Default for Ball {
    fn default() -> Self {
        const DEFAULT_VELOCITY: f32 = 200.0;
        Self {
            velocity: Vec2::new(1.0, 1.0).normalize() * DEFAULT_VELOCITY,
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

fn ball_movement_system(time: Res<Time>, mut query: Query<(&Ball, &mut Transform)>) {
    let delta: f32 = time.delta_seconds();
    query.iter_mut().for_each(|(ball, mut transform)| {
        transform.translation += delta * ball.velocity.extend(0.0);
    });
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

struct Paddle;

impl Paddle {
    const SPEED: f32 = 200.0;
}

enum Player {
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

fn spawn_paddle(commands: &mut Commands, player: Player) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                size: Vec2::new(20., 200.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(Transform {
            translation: player.start_position().extend(0.),
            ..Default::default()
        })
        .with(player)
        .with(Paddle)
        .with(Collider::Paddle);
}

fn paddle_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Paddle, &Player, &mut Transform)>,
) {
    query
        .iter_mut()
        .for_each(|(_paddle, player, mut transform)| {
            let (up_keycode, down_keycode) = player.movement_keys();
            let dt = time.delta_seconds();
            if keyboard_input.pressed(up_keycode) {
                transform.translation += Vec2::new(0.0, Paddle::SPEED * dt).extend(0.);
                println!("Up!")
            } else if keyboard_input.pressed(down_keycode) {
                transform.translation += Vec2::new(0.0, -Paddle::SPEED * dt).extend(0.);
                println!("Down!")
            }
        });
}

fn window_resize_listenr(
    mut resize_listenr: ResMut<EventReader<WindowResized>>,
    resize_events: Res<Events<WindowResized>>,
) {
    resize_listenr.latest(&resize_events).map(|event| {
        // println!("{:?}", event);
        println!("window resized to {:}x{:}", event.width, event.height);
    });
}

fn print_window_descriptor(window_descriptor: Res<WindowDescriptor>) {
    println!("{}", window_descriptor.width);
    println!("{}", window_descriptor.height);
}

fn ball_collision_system(
    mut ball_query: Query<(&mut Ball, &Transform, &Sprite)>,
    collider_query: Query<(&Collider, &Transform, &Sprite)>,
) {
    for (mut ball, ball_transform, ball_sprite) in ball_query.iter_mut() {
        for (_collider, collider_transform, collider_sprite) in collider_query.iter() {
            let collision = collide(
                ball_transform.translation,
                ball_sprite.size,
                collider_transform.translation,
                collider_sprite.size,
            );
            let direction = match collision {
                Some(direction) => direction,
                None => continue,
            };

            use Collision::*;
            let (reflect_x, refrect_y) = match direction {
                Left => (ball.velocity.x > 0.0, false),
                Right => (ball.velocity.x < 0.0, false),
                Top => (false, ball.velocity.y < 0.0),
                Bottom => (false, ball.velocity.y > 0.0),
            };

            if reflect_x {
                ball.velocity.x *= -1.;
            };
            if refrect_y {
                ball.velocity.y *= -1.;
            }
        }
    }
}
