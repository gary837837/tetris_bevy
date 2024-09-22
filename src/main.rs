mod resource;
use bevy::audio::Volume;
use resource::*;

use bevy::prelude::*;
use bevy::window::WindowResolution;

const TILE_SIZE: f32 = 64.;
const BOARD_WIDTH: f32 = 10. * 64.;
const BOARD_HEIGHT: f32 = 20. * 64.;

#[derive(Component)]
struct Block {
    row: u8,
    column: u8,
    tile: Option<Tetromino>,
}

#[derive(Component)]
struct Board;

#[derive(Component)]
struct TetrominoPreview {
    index: usize,
    tetromino: Tetromino,
}

#[derive(Component)]
struct ScoreText;

#[derive(Resource)]
struct DroppingTimer(Timer);

fn main() {
    App::new()
        .insert_resource(TileAssets::new())
        .insert_resource(TetrominoAssets::new())
        .insert_resource(BoardMap::new())
        .insert_resource(TetrominoSupplier::new())
        .insert_resource(DroppingTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // window resolution, scale
                resolution: WindowResolution::new(1280., 720.).with_scale_factor_override(0.5),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Startup, board_startup)
        .add_systems(Update, board_update)
        .add_systems(Update, score_update)
        .add_systems(Update, preview_update)
        .add_systems(Update, player_update)
        .add_systems(Update, board_event_update)
        .run();
}

fn setup(
    mut commands: Commands,
    mut tile_assets: ResMut<TileAssets>,
    mut tetromino_assets: ResMut<TetrominoAssets>,
    asset_server: Res<AssetServer>,
) {
    tile_assets.setup(&asset_server);
    tetromino_assets.setup(&asset_server);

    // camera setup
    commands.spawn(Camera2dBundle::default());

    // board setup
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("Board.png"),
            ..default()
        },
        Board,
    ));

    // block setup
    for y in 0..20 {
        for x in 0..10 {
            commands.spawn((
                SpriteBundle {
                    texture: tile_assets.get(Tetromino::G),
                    transform: Transform::from_xyz(
                        x as f32 * TILE_SIZE - BOARD_WIDTH * 0.5 + TILE_SIZE * 0.5,
                        y as f32 * TILE_SIZE - BOARD_HEIGHT * 0.5 + TILE_SIZE * 0.5,
                        1.,
                    ),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                Block {
                    row: y,
                    column: x,
                    tile: None,
                },
            ));
        }
    }

    // score text
    commands.spawn((
        TextBundle::from_section(
            "00000000",
            TextStyle {
                font_size: 128.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        }),
        ScoreText,
    ));

    // preview bg
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("blocks/Ghost.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(192.0, 640.0)),
                ..default()
            },
            transform: Transform::from_xyz(
                BOARD_WIDTH * 0.5 + 256.0,
                BOARD_HEIGHT * 0.5 - 512.0,
                0.,
            ),
            ..default()
        },
        ImageScaleMode::Sliced(TextureSlicer {
            border: BorderRect::square(10.),
            center_scale_mode: SliceScaleMode::Tile { stretch_value: 0.5 },
            sides_scale_mode: SliceScaleMode::Tile { stretch_value: 0.5 },
            ..default()
        }),
    ));

    // next tetrominoe preview
    for i in 0..5 {
        commands.spawn((
            SpriteBundle {
                texture: tetromino_assets.get(Tetromino::I),
                transform: Transform::from_xyz(
                    BOARD_WIDTH * 0.5 + 256.0,
                    BOARD_HEIGHT * 0.5 - 256.0 - 128.0 * (i as f32),
                    1.,
                )
                .with_scale(Vec3::splat(0.5)),
                ..default()
            },
            TetrominoPreview {
                index: i,
                tetromino: Tetromino::I,
            },
        ));
    }

    // guide text
    commands.spawn((TextBundle::from_section(
        "Move: Arrow Key\nRotate: Q E\nPut: Speace",
        TextStyle {
            font_size: 96.0,
            ..default()
        },
    )
    .with_style(Style {
        position_type: PositionType::Absolute,
        top: Val::Px(5.0),
        left: Val::Px(15.0),
        ..default()
    }),));
}

fn board_startup(
    mut board_map: ResMut<BoardMap>,
    mut tetromino_supplier: ResMut<TetrominoSupplier>,
) {
    let rng = &mut rand::thread_rng();
    board_map.player_spawn(Tetromino::gen(rng));
    tetromino_supplier.fill(rng);
}

fn board_update(
    mut query: Query<(&mut Block, &mut Handle<Image>, &mut Visibility)>,
    board_map: Res<BoardMap>,
    tile_assets: Res<TileAssets>,
) {
    for (mut block, mut tile_image, mut visibility) in &mut query {
        let tile = board_map.tile_get(block.row, block.column);
        if block.tile == tile {
            continue;
        }
        block.tile = tile;
        if let Some(tile) = tile {
            *tile_image = tile_assets.get(tile);
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

fn score_update(mut query: Query<&mut Text, With<ScoreText>>, board_map: Res<BoardMap>) {
    let mut score_text = query.single_mut();
    let score = board_map.score_get();
    score_text.sections[0].value = format!("{0:<08}", score);
}

fn preview_update(
    mut query: Query<(&mut TetrominoPreview, &mut Handle<Image>)>,
    tetromino_supplier: Res<TetrominoSupplier>,
    tetromino_assets: Res<TetrominoAssets>,
) {
    for (mut tetromino_preview, mut tetromino_image) in &mut query {
        let tetromino = tetromino_supplier.get(tetromino_preview.index);
        if tetromino_preview.tetromino == tetromino {
            continue;
        }
        tetromino_preview.tetromino = tetromino;
        *tetromino_image = tetromino_assets.get(tetromino);
    }
}

fn player_update(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut board_map: ResMut<BoardMap>,
    mut dropping_timer: ResMut<DroppingTimer>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        board_map.player_move(MoveDirection::Down);
        dropping_timer.0.reset();
    } else if dropping_timer.0.tick(time.delta()).just_finished() {
        board_map.player_move(MoveDirection::Down);
    }
    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        board_map.player_move(MoveDirection::Left);
    }
    if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        board_map.player_move(MoveDirection::Right);
    }
    // if keyboard_input.just_pressed(KeyCode::ArrowUp) {
    //     board_map.player_move(MoveDirection::Up);
    // }
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        board_map.player_rotate(RotateDirection::Left);
    }
    if keyboard_input.just_pressed(KeyCode::KeyW) {
        board_map.player_rotate(RotateDirection::Right);
    }
    if keyboard_input.just_pressed(KeyCode::Space) {
        board_map.player_move_to_bottom();
        dropping_timer.0.reset();
    }
}

fn board_event_update(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut board_map: ResMut<BoardMap>,
    mut tetromino_supplier: ResMut<TetrominoSupplier>,
) {
    if board_map.event_get(BoardEvent::LineCompleted) {
        commands.spawn(AudioBundle {
            source: asset_server.load::<AudioSource>("sounds/Line.wav"),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: Volume::new(0.2),
                ..default()
            },
            ..default()
        });
    }
    if board_map.event_get(BoardEvent::TetrominoMoved) {
        commands.spawn(AudioBundle {
            source: asset_server.load::<AudioSource>("sounds/Move.wav"),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: Volume::new(0.1),
                ..default()
            },
            ..default()
        });
    }
    if board_map.event_get(BoardEvent::TetrominoPut) {
        commands.spawn(AudioBundle {
            source: asset_server.load::<AudioSource>("sounds/Put.wav"),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: Volume::new(0.2),
                ..default()
            },
            ..default()
        });
        let rng = &mut rand::thread_rng();
        board_map.player_spawn(tetromino_supplier.pop(rng));
    }
    if board_map.event_get(BoardEvent::GameOver) {
        info!("GameOver");
    }
    board_map.event_reset();
}
