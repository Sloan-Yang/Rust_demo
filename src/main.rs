use bevy::prelude::*;

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());
    commands
        .spawn(Sprite {
            image: asset_server.load("image.jpg"),
            ..Default::default()
        })
        .insert(Player);
}

fn move_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut players: Query<(&mut Transform ,  &mut Sprite ),  With<Player>>,
) {

    let (mut player, mut sprite) = players.iter_mut().next().unwrap();
    if keyboard_input.pressed(KeyCode::KeyA) {
        sprite.flip_x = false;
        player.translation.x -= 1.0;
        
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        sprite.flip_x = true;
        player.translation.x += 1.0;
        
    }
}



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update , move_system)
        .run();
}