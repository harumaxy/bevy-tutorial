use bevy::prelude::*;

#[macro_use]
extern crate bevy_discovery;

// bevy_discovery Plugin を Auto Deriving
// #[system] macro をつけた関数をすべて App にシステムとして追加するマクロ
#[derive(DiscoveryPlugin)]
struct DiscoveryPulugin;

#[system]
fn hello_world_system() {
    print!("Hello World")
}

fn main() {
    App::build().add_plugin(DiscoveryPulugin).run()
}
