use bevy::{log::tracing_subscriber::fmt::format, prelude::*};
use rand_chacha::ChaCha8Rng;
use rand::{Rng , SeedableRng};
use std::{ f32::consts::PI};

struct Cell{
    height:f32, 
}
#[derive(Default)]
struct Player{
    entity:Option<Entity>,
    i:usize,
    j:usize,
    move_cooldown: Timer,
}

#[derive(Default)] 
struct Bonus {
    entity:Option<Entity> , 
    i: usize ,
    j: usize ,
    handle: Handle<Scene>,    
}
#[derive (Resource,Default)]
struct   Game {
    board: Vec<Vec<Cell>>,
    player:Player , 
    bonus : Bonus , 
    score : i32 ,
    cake_eaten: u32 , 
    camera_is_focus:Vec3 , 
    camera_should_focus : Vec3 ,

}
#[derive(Resource,Deref,DerefMut)]
struct Random(ChaCha8Rng);

const BOARD_SIZE_I : usize = 14;
const BOARD_SIZE_J : usize = 21;

const  RESET_FOCUS : [f32; 3] = 
[
    BOARD_SIZE_I as f32 / 2.0,
    0.,
    BOARD_SIZE_J as f32 / 2.0  -0.5,   
];

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug,Default,States)]
enum GameState{
    #[default]
    Playing,
    GameOver ,
}

fn setup_cameras(
    mut commands : Commands ,
    mut game : ResMut<Game> ,
){
    game.camera_should_focus = Vec3::from(RESET_FOCUS) ; 
    game.camera_is_focus = game.camera_should_focus ; 
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(
            -(BOARD_SIZE_I as f32 /2.0), 
            (BOARD_SIZE_I as f32 /2.)+2.0, 
            (BOARD_SIZE_J as f32 /2.0)-0.5    )
            .looking_at( game.camera_should_focus,
             Vec3::Y) ,
    )   ) 
    ;

}

fn gameover_keyboard(
    keyboard_input : Res<ButtonInput<KeyCode>>,
    mut next_state : ResMut<NextState<GameState>>
){
    if keyboard_input.just_pressed(KeyCode::Space){
        next_state.set(GameState::Playing);
    }
}

fn setup(
    mut commands:Commands , asset_server : Res<AssetServer> , 
    mut game : ResMut<Game>,
)
{
    let mut rng=    if std::env::var("GITHUB_ACTION") == Ok("true".to_string()){
        ChaCha8Rng::seed_from_u64(121313131322323)
    }else{
        let mut os_rng = ChaCha8Rng::from_seed([0;32]);
        ChaCha8Rng::from_rng(&mut os_rng)
    };
    game.score = 0;
    game.cake_eaten=0 ;
    game.player.i = BOARD_SIZE_I  / 2 ;
    game.player.j = BOARD_SIZE_J  / 2 ;
    game.player.move_cooldown = Timer::from_seconds(0.1, TimerMode::Once);

    commands.spawn((
        StateScoped(GameState::Playing),
        PointLight{
            intensity: 2_000_000.0,
            range: 30.0,
            shadows_enabled : true, 
            ..default()
        },
        Transform::from_xyz(BOARD_SIZE_I as f32 /2., BOARD_SIZE_J as f32, BOARD_SIZE_J as f32 /2.),
    
    ));
    
    let scene_tile  = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/AlienCake/tile.glb"));
    game.board = 
            (0..BOARD_SIZE_J).map(|j|{
                (0..BOARD_SIZE_I).map(|i|{
                    let height =rng.random_range(-0.1..0.1);

                    commands.spawn((
                        SceneRoot(scene_tile.clone()),
                        Transform::from_xyz(i as f32 , height-0.2, j as f32),
                        StateScoped(GameState::Playing),
                    ));
                        Cell{height}
                }).collect()
    }).collect();

    game.player.entity = Some(
       commands.spawn((
        StateScoped(GameState::Playing),
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/AlienCake/alien.glb"))),   
        Transform{
            translation: Vec3::new(
                game.player.i as f32 ,
                 game.board[game.player.j][game.player.j].height   , 
                 game.player.j as f32),
            rotation: Quat::from_rotation_y(-PI/2.), 
            ..default() 
        },) 
    ).id());

    game.bonus.handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/AlienCake/cakeBirthday.glb"));

    commands.spawn((
        Text::new("Score :"  ),
        TextFont{
            font_size : 32.0 ,
            ..default()
        },
        TextColor(Color::srgb(0.2, 0.3, 0.5)),
        Node{
            position_type: PositionType::Absolute,
            left : Val::Px(5.),
            top :  Val::Px(5.),
            ..default()  
        },
        StateScoped(GameState::Playing),
    )); 
    commands.insert_resource(Random(rng));   
}


#[derive(Resource)]
struct BonusSpawnTimer(Timer);


fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        .insert_resource(BonusSpawnTimer(Timer::from_seconds(
            5.0, TimerMode::Repeating)))
        .init_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        .add_systems(Startup, setup_cameras)
        .add_systems(OnEnter(GameState::Playing), setup)
        .add_systems(
            Update,
            (
                move_player,
                focus_camera,
                rotate_bonus,
                scoreboard_system,
                spawn_bonus,
            )
            .run_if(in_state(GameState::Playing))
        )
        .add_systems(OnEnter(GameState::GameOver),display_score)
        .add_systems(Update,
        gameover_keyboard.run_if(in_state(GameState::GameOver) ),  )
        .run();
}


fn display_score(mut commands: Commands,game:Res<Game>){
    commands
        .spawn((StateScoped(GameState::GameOver),
        Node{
            width: Val::Percent(100.),
            align_items:AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
    ))
    .with_child((
        Text::new(format!("Cake eaten:{}",game.cake_eaten)),
        TextFont{
            font_size:67.0,
            ..default()
        },
        TextColor(Color::srgb(0.5, 0.5, 1.0)),
    ))   ;
}


fn scoreboard_system(game:Res<Game>,mut display: Single<&mut Text>){
    display.0 = format!("Sugar Rush: {}", game.score);
}

fn spawn_bonus(    
    time:Res<Time>,
    mut timer: ResMut<BonusSpawnTimer>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut rng: ResMut<Random>,

){
    if !timer.0.tick(time.delta()).finished(){
        return;
    }
    if let Some(entity) = game.bonus.entity{
        game.score -= 3 ;
        commands.entity(entity).despawn_recursive();
        game.bonus.entity=None;
        if game.score <= -5{
            next_state.set(GameState::GameOver);
            return;
        }
    } 

    loop{
        game.bonus.i = rng.random_range(0..BOARD_SIZE_I);
        game.bonus.j = rng.random_range(0..BOARD_SIZE_J);  
        if game.bonus.i !=game.player.i || game.bonus.j != game.player.j {
            break;
        }
    } 
    game.bonus.entity =Some(
        commands
            .spawn(
                (
                    SceneRoot(game.bonus.handle.clone()),
                    StateScoped(GameState::Playing),
                    Transform::from_xyz(
                        game.bonus.i as f32,
                        game.board[game.bonus.j][game.bonus.i].height +0.2,
                        game.bonus.j as f32 ,
                    ),
                ) ).with_child((
                    PointLight{
                        color:Color::srgb(0.0,2.0,0.0) ,
                        intensity: 500_000.0,
                        range: 10.0,
                        ..default()

                    },
                    Transform::from_xyz (0.0,2.0,0.0),
                )).id(),
    );
    
}

fn rotate_bonus(
    game:Res<Game>,
    time: Res<Time>,
    mut transforms: Query<&mut Transform>,
){
    if let  Some(entity) = game.bonus.entity {
        if let Ok(mut cake_trans) = transforms.get_mut(entity){
            cake_trans.rotate_y(time.delta_secs());
            cake_trans.scale =
                Vec3::splat(1.0+(game.score as f32 / 10.0 * ops::sin(time.elapsed_secs())).abs());
            }
    }

}

fn move_player(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
    time : Res<Time>,
){
    if game.player.move_cooldown.tick(time.delta()).finished(){
        let mut moved =false;
        let mut rotation =0.0;
        
        if keyboard_input.pressed(KeyCode::ArrowUp){
            if game.player.i < BOARD_SIZE_I - 1 {
                game.player.i +=1 ;
            }
            rotation = -PI /2. ;
            moved = true ; 
        }
        if keyboard_input.pressed(KeyCode::ArrowDown){
            if game.player.i  > 0 {
                game.player.i -=1 ;
            }
            rotation =PI/2. ;
            moved = true ;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight){
            if game.player.j  < BOARD_SIZE_J -1 {
                game.player.j +=1 ;
            }
            rotation =PI ;
            moved = true ;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft){
            if game.player.j  > 0  {
                game.player.j -=1 ;
            }
            rotation =0.0 ;
            moved = true ;
        }
        if moved{
            game.player.move_cooldown.reset();
            *transforms.get_mut(game.player.entity.unwrap()).unwrap()   =  Transform {
                translation: Vec3::new(
                    game.player.i as f32 , 
                    game.board[game.player.j][game.player.i].height,
                    game.player.j as f32 , 
                ),
                rotation: Quat::from_rotation_y(rotation),
                ..default()
            };
        }
    }
    if let Some(entity) = game.bonus.entity{
        if game.player.i == game.bonus.i  && game.player.j == game.bonus.j {
            game.score +=2;
            game.cake_eaten +=1;
            commands.entity(entity).despawn_recursive();
            game.bonus.entity = None ;

        }
    }

}

fn focus_camera(
    time:Res<Time>,
    mut game: ResMut<Game>,
    mut transforms: ParamSet<(Query<&mut Transform , With<Camera3d> >,Query<&Transform>)>,
){
    const SPEED :f32  = 2.0 ;
    if let (Some(player_entity),Some(bonus_entity)) = (game.player.entity, game.bonus.entity ) {
        let transform_query =  transforms.p1();
        if let (Ok(player_transform),Ok(bonus_transform)) = (
            transform_query.get(player_entity),
            transform_query.get(bonus_entity) ,
        ){
            game.camera_should_focus = player_transform
                .translation
                .lerp(bonus_transform.translation, 0.5  );
        }
    }
    else if  let Some(player_entity) = game.player.entity{
        if let Ok(player_transform) = transforms.p1().get(player_entity){
            game.camera_should_focus  = player_transform.translation;
        } 
    } else{
        game.camera_should_focus=Vec3::from(RESET_FOCUS);
    }
    let mut camera_motion = game.camera_should_focus  - game.camera_is_focus;
    if camera_motion.length() > 0.2 {
        camera_motion *= SPEED * time.delta_secs();
        game.camera_is_focus += camera_motion ;
    }
    for mut transform in transforms.p0().iter_mut(){
        *transform = transform.looking_at(game.camera_is_focus, Vec3::Y);
    }
}



