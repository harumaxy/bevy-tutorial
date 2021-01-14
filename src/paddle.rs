use bevy::{prelude::*, window::WindowResized};

use crate::{Collider, Player};

#[derive(Default)]
pub struct Paddle {
    pub speed: f32,
}

impl Paddle {
    pub const SPEED: f32 = 200.0;
    pub const WIDTH: f32 = 20.0;
    pub const MARGIN: f32 = 50.0;

    pub fn update_after_window_resize(
        &mut self,
        player: &Player,
        resize_event: &WindowResized,
        size_component: &mut Vec2,
        translation_component: &mut Vec3,
    ) {
        let window_height = resize_event.height as f32;
        let window_width = resize_event.width as f32;
        self.speed = (window_height as f32) / 3.;
        *size_component = Vec2::new(Paddle::WIDTH, 0.2 * window_height);
        let horizontal_distance_from_center = (window_width / 2.0) - Paddle::MARGIN;
        use Player::*;
        let x_translation = match player {
            Left => -horizontal_distance_from_center,
            Right => horizontal_distance_from_center,
        };
        *translation_component = Vec3::new(x_translation, 0., 0.);
    }
}

pub fn paddle_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Paddle, &Player, &mut Transform)>,
) {
    query
        .iter_mut()
        .for_each(|(paddle, player, mut transform)| {
            let (up_keycode, down_keycode) = player.movement_keys();
            let dt = time.delta_seconds();
            if keyboard_input.pressed(up_keycode) {
                transform.translation += Vec2::new(0.0, paddle.speed * dt).extend(0.);
            } else if keyboard_input.pressed(down_keycode) {
                transform.translation += Vec2::new(0.0, -paddle.speed * dt).extend(0.);
            }
        });
}

pub type PaddleBundle = (SpriteBundle, Paddle, Player, Collider);

pub fn spawn_paddle(
    commands: &mut Commands,
    player: Player,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
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
            material: materials.add(match player {
                Player::Left => ColorMaterial::color(Color::RED),
                Player::Right => ColorMaterial::color(Color::BLUE),
            }),
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
