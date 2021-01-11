use bevy::prelude::*;

fn hello_world_system() {
    println!("Hello World")
}

struct Person;
struct Name(String);

// 基本的に、Component は 引数で コンパイル時に Dynamic に決定される
// &mut, or & が基本
fn add_people_to_world_system(commands: &mut Commands) {
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

fn great_people_system(query: Query<(&Person, &Name)>) {
    for (_, Name(name)) in query.iter() {
        println!("Hello {}", name)
    }
}

#[bevy_main]
fn main() {
    App::build()
        .add_startup_system(add_people_to_world_system(commands).system())
        .add_system(hello_world_system.system())
        .add_system(great_people_system.system())
        .run()
}
