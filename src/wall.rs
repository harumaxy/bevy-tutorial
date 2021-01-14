use bevy::{prelude::*, window::WindowResized};

use crate::Collider;

pub enum Wall {
    Top,
    Bottom,
}

impl Wall {
    const THICKNESS: f32 = 20.0;

    pub fn update_after_window_resize(
        &mut self,
        resize_event: &WindowResized,
        size: &mut Vec2,
        translation: &mut Vec3,
    ) {
        let window_height = resize_event.height as f32;
        let window_width = resize_event.width as f32;
        let vertical_distance_from_center = (window_height / 2. - Self::THICKNESS / 2.);
        let y_translation = match self {
            Wall::Top => vertical_distance_from_center,
            Wall::Bottom => -vertical_distance_from_center,
        };

        *size = Vec2::new(window_width, Self::THICKNESS);
        *translation = Vec3::new(0., y_translation, 0.);
    }
}

pub fn spwan_walls(commands: &mut Commands) {
    spwan_wall(commands, Wall::Top);
    spwan_wall(commands, Wall::Bottom);
}

fn spwan_wall(commands: &mut Commands, wall: Wall) {
    commands
        .spawn(SpriteBundle {
            sprite: Default::default(),
            transform: Default::default(),
            ..Default::default()
        })
        .with(wall)
        .with(Collider::Wall);
}
