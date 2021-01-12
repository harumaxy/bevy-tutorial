use bevy::{input::keyboard::KeyboardInput, prelude::*};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_system(spawn_ball.system())
        .add_system(ball_movement_system.system())
        .add_system(paddle_movement_system.system())
        .run()
}

struct Ball {
    velocity: Vec2,
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
        .with(Paddle);
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