use bevy::{prelude::*, render::camera::ScalingMode};

pub mod player;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "2d Game".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(Update, player::player_movement)
        .add_systems(Startup, setup)
        .run();
    
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 600.0,
        min_height: 800.0,
    };

    commands.spawn(camera);

    let texture = asset_server.load("rick.png");

    commands.spawn(SpriteBundle {
            texture,
            ..default()})
    .insert(player::Player{ speed: 100.0});
}
