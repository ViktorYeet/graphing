use bevy::{prelude::*};

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start);
    }
}

fn start() {
    println!("started")
}
        
fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: String::from("my window"),
            decorations: false,
            ..default()
        }),
        ..default()
    }))
    .add_plugins(HelloPlugin)
    .run();
}