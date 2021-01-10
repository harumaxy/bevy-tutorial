use bevy::prelude::*;

struct Pos {
    x: i32,
    y: i32,
}

struct Obj;

fn print_pos_system(query: Query<&Pos>) {
    for pos in query.iter() {
        println!("{:?}, {:?}", pos.x, pos.y);
    }
}

fn init_system(commands: &mut Commands) {
    commands.spawn((Obj, Pos { x: 1, y: 2 }));
}

fn spam_spam_spam_system(query: Query<&Pos>) {
    query.iter().for_each(|pos| print!("spam"));
}

fn main() {
    App::build()
        .add_startup_system(init_system.system())
        .add_system(print_pos_system.system())
        .add_system(spam_spam_spam_system.system())
        .run();
}