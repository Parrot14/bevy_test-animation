use std::{env, time::Duration};

use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, (setup, setup_animation))
        .add_systems(Update, update)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup_animation(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let val;

    let animation_file = match env::var("animation") {
        Ok(v) => {
            val = v;
            match val.as_str() {
            "2" => "test-2keys.glb",
            "1" =>  "test-1key.glb",
            "0" => "test-keyless.glb",
            _v =>  _v
        }},
        Err(_) => panic!("idk"),
    };
    commands.insert_resource(AnimationTimer(Timer::new(Duration::from_secs(1), TimerMode::Once)));

    commands.insert_resource(Animations([
        assets.load(format!("{}#Animation0", animation_file)),
        assets.load(format!("{}#Animation1", animation_file)),
        assets.load(format!("{}#Animation2", animation_file)),
        assets.load(format!("{}#Animation3", animation_file)),
    ]));

    commands.spawn(SceneBundle {
        scene: assets.load(format!("{}#Scene0", animation_file)),
        ..default()
    });
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(20.0, 10.0, 10.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 6.0, 4.0),
        ..default()
    });
}

fn update(
    mut animation: Query<&mut AnimationPlayer>,
    mut animation_step: Local<u8>,
    mut animation_timer: ResMut<AnimationTimer>,
    mut started: Local<bool>,
    animations: Res<Animations>,
    time: Res<Time>,
) {
    if animation_timer.0.tick(time.delta()).finished() {
        if let Ok(mut player) = animation.get_single_mut(){
            if player.is_finished() {
                player.play(animations.0[*animation_step as usize].clone_weak());

                *animation_step = if *animation_step >= 3 {
                    animation_timer.0.reset();
                    0
                }else{
                    *animation_step + 1
                };
            }else if !*started {
                player.play(animations.0[0].clone());
                *started = true;
            }
        }
    }
}

#[derive(Resource)]
struct AnimationTimer(Timer);

#[derive(Resource)]
struct Animations([Handle<AnimationClip>;4]);