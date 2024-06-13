use bevy::{
    ecs::query,
    prelude::*,
    scene::ron::de,
    transform::{self, commands},
    window::{PrimaryWindow, Window, WindowPlugin, WindowResolution},
};

pub enum PlayerState {
    IdleRight,
    IdleLeft,
    WalkingRight,
    WalkingLeft,
}

#[derive(Component)]
pub struct Player {
    state: PlayerState,
}

#[derive(Component)]
pub struct PlayerPosition {}

#[derive(Component)]
pub struct SpacePressed {
    times: usize,
}

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 500.0;
const INDEX_FIRST: usize = 33;
const INDEX_LAST: usize = 40;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("sprites/cat_sprite.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 8, 10, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices {
        first: INDEX_FIRST,
        last: INDEX_LAST,
    };
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            transform: Transform::from_scale(Vec3::splat(4.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Player {
            state: PlayerState::IdleRight,
        },
    ));
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        // .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup)
        .add_systems(Update, animate_sprite)
        // .add_systems(Startup, spawn_player)
        // .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_text)
        .add_systems(Update, keyboard_control)
        .add_systems(Update, text_update_position)
        .run();
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {
            state: PlayerState::IdleLeft,
        },
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
        ..default()
    });
}

fn keyboard_control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_position: Query<&mut Transform, With<Player>>,
    mut space_text: Query<(&mut Text, &mut SpacePressed), With<SpacePressed>>,
) {
    let mut transform = player_position.get_single_mut().unwrap();

    if keyboard_input.pressed(KeyCode::ArrowLeft) && transform.translation.x > 0. {
        transform.translation.x -= 10.;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) && transform.translation.x < WINDOW_WIDTH {
        transform.translation.x += 10.;
    }
    // if keyboard_input.pressed(KeyCode::ArrowDown) && transform.translation.y > 0. {
    //     transform.translation.y -= 10.;
    // }
    // if keyboard_input.pressed(KeyCode::ArrowUp) && transform.translation.y < WINDOW_HEIGHT {
    //     transform.translation.y += 10.;
    // }

    if keyboard_input.just_pressed(KeyCode::Space) {
        let mut text = space_text.get_single_mut().unwrap();

        text.1.times += 1;
        text.0.sections[0].value = format!("{}", text.1.times);
    }
}

fn text_update_position(
    player_position: Query<&Transform, With<Player>>,
    mut text_query: Query<&mut Text, With<PlayerPosition>>,
) {
    let transform = player_position.get_single().unwrap();
    let mut text = text_query.get_single_mut().unwrap();

    text.sections[0].value = format!("{}, {}", transform.translation.x, transform.translation.y);
}

fn spawn_text(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section("HEllo!", TextStyle::default()),
        PlayerPosition {},
    ));

    commands.spawn((
        TextBundle::from_section(String::new(), TextStyle::default()).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        SpacePressed { times: 0 },
    ));
}
