#![feature(bool_to_option)]
use std::time::Duration;

use bevy::prelude::*;
use bevy::DefaultPlugins;

fn hello_world() {
    println!("Hello World")
}

struct Person;
struct Name(String);

struct HelloPlugin;

struct GreetTimer(Timer);

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) -> () {
        app.add_system(hello_world.system());
    }
}

// 基本的に、Component は 引数で コンパイル時に Dynamic に決定される
// &mut, or & が基本

fn add_people_to_world(commands: &mut Commands) {
    let people = (1..=100)
        .into_iter()
        .map(|num| num.to_string())
        .map(|name| (Person, Name(name)));

    commands
        .spawn_batch(people)
        .spawn((Person, Name("Max".to_string())))
        .spawn((Person, Name("Hennly".to_string())))
        .spawn((Person, Name("Kenes".to_string())));
}

fn great_people(time: ResMut<GreetTimer>, query: Query<(&Person, &Name)>) {
    time.0.just_finished().then(|| {
        query
            .iter()
            .for_each(|(_, Name(name))| println!("Hello {}", name,))
    });
}

#[bevy_main]
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_people_to_world.system())
        .add_resource(GreetTimer(Timer::new(Duration::from_secs(3), false)))
        .add_system(
            (|mut time: ResMut<GreetTimer>| {
                time.0.tick(0.1);
            })
            .system(),
        )
        // .add_plugin(HelloPlugin)
        .add_system(great_people.system())
        // .add_system((move || println!("hello")).system())
        .run()
}
