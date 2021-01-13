use bevy::{prelude::*, window::WindowResized};

use crate::Collider;

pub enum Goal {
    Left,
    Right,
}

impl Goal {
    const THICKNESS: f32 = 20.0;

    pub fn update_after_window_resize(
        &mut self,
        resize_event: &WindowResized,
        size_component: &mut Vec2,
        translation_component: &mut Vec3,
    ) {
        let window_height = resize_event.height as f32;
        let window_width = resize_event.width as f32;
        *size_component = Vec2::new(Self::THICKNESS, window_height);
        let horizontal_distance_from_center = window_width / 2.0;
        let x_translation = match self {
            Goal::Left => -horizontal_distance_from_center,
            Goal::Right => horizontal_distance_from_center,
        };
        *translation_component = Vec3::new(x_translation, 0., 0.);
    }
}

pub fn spwan_goals(commands: &mut Commands) {
    spwan_goal(commands, Goal::Left);
    spwan_goal(commands, Goal::Right);
}

fn spwan_goal(commands: &mut Commands, goal: Goal) {
    commands
        .spawn(SpriteBundle {
            sprite: Default::default(),
            transform: Default::default(),
            ..Default::default()
        })
        .with(goal);
}
