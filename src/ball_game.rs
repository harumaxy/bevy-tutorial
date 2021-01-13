use std::unimplemented;

mod paddle;
use paddle::*;

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
    speed: f32,
    direction: Vec2,
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

impl Ball {
    fn velocity(&self) -> Vec2 {
        self.direction * self.speed
    }

    fn ball_size_and_translation(&self, window_height: usize) -> (Vec2, Vec3) {
        let ball_width = 0.05 * (window_height as f32);
        let size = Vec2::new(ball_width, ball_width);
        let translation = Vec3::default();
        (size, translation)
    }
    fn update_after_window_resize(&mut self, window_height: usize) {
        self.speed = (window_height as f32) / 1.;
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
        transform.translation += delta * ball.velocity().extend(0.0);
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
            let height = event.height as usize;
            let (size, translation) = ball.ball_size_and_translation(height);
            sprite.size = size;
            transform.translation = translation;
            ball.update_after_window_resize(height);
        }
    });
}

fn print_window_descriptor(window_descriptor: Res<WindowDescriptor>) {
    // println!("{}", window_descriptor.width);
    // println!("{}", window_descriptor.height);
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
            let velocity = ball.velocity();
            let (reflect_x, reflect_y) = match direction {
                Left => (velocity.x > 0.0, false),
                Right => (velocity.x < 0.0, false),
                Top => (false, velocity.y < 0.0),
                Bottom => (false, velocity.y > 0.0),
            };

            let reflection_multiplier =
                Vec2::new(sign_from_bool(reflect_x), sign_from_bool(reflect_y));
            ball.direction *= reflection_multiplier;
        }
    }
}

const fn sign_from_bool(boolean: bool) -> f32 {
    if boolean {
        -1.0
    } else {
        1.0
    }
}
