use bevy::{prelude::*, render::camera::ScalingMode};

pub mod player;
pub mod animations;

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
        .add_systems(Update, animations::animate_sprite)
        .add_systems(Startup, setup)
        .run();
    
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>,) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 600.0,
        min_height: 800.0,
    };

    commands.spawn(camera);

    let texture_handle = asset_server.load("gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let gabe_animation_indices = animations::Animation { first: 1, last: 6 };

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(gabe_animation_indices.first),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        gabe_animation_indices,
        animations::AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    )).insert(player::Player{ speed: 200.0 });
    
}
