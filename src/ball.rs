use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    window::WindowResized,
};

use crate::Collider;

pub struct Ball {
    pub speed: f32,
    pub direction: Vec2,
}

impl Ball {
    pub fn velocity(&self) -> Vec2 {
        self.direction * self.speed
    }

    pub fn update_after_window_resize(
        &mut self,
        resize_event: &WindowResized,
        size: &mut Vec2,
        translation: &mut Vec3,
    ) {
        let window_height = resize_event.height as f32;
        self.speed = window_height / 1.5;
        let ball_width = 0.05 * (window_height as f32);
        *size = Vec2::new(ball_width, ball_width);
        *translation = Vec3::default();
    }
}
pub fn ball_movement_system(time: Res<Time>, mut query: Query<(&Ball, &mut Transform)>) {
    let delta: f32 = time.delta_seconds();
    query.iter_mut().for_each(|(ball, mut transform)| {
        transform.translation += delta * ball.velocity().extend(0.0);
    });
}

pub fn ball_collision_system(
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
