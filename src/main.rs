use bevy::prelude::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 2D 摄像机
    commands.spawn(Camera2d::default());

    // 加载图片并创建精灵
    commands.spawn(Sprite {
        image: asset_server.load("image.jpg"),
        ..Default::default()
    });
}

fn keyboard_input_system(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::KeyA) {
        println!("Go Left!");
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        println!("Go Right!");
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update , keyboard_input_system)
        .run();
}