use bevy::prelude::*;
use bevy::sprite::{Sprite, SpriteBundle};
use bevy::text::{Text, Text2dBundle, TextStyle};
use bevy::core_pipeline::core_2d::Camera2dBundle;

const GRAVITY: f32 = 9.8 * 100.0;
const JUMP_FORCE: f32 = 500.0;
const PIPE_SPEED: f32 = 200.0;
const PIPE_GAP: f32 = 150.0;
const PIPE_WIDTH: f32 = 80.0;
const BIRD_SIZE: f32 = 40.0;

#[derive(Component)]
struct Bird {
    velocity: f32,
}

#[derive(Component)]
struct Pipe;

#[derive(Resource)]
struct PipeTimer(Timer);

#[derive(Resource, Default)]
struct Score {
    value: u32,
}

#[derive(Resource, Default)]
struct GameState {
    game_over: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<GameState>()
        .init_resource::<Score>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                bird_movement,
                bird_jump,
                pipe_spawning,
                pipe_movement,
                collision_detection,
                scoring,
                game_over_text,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Bird (yellow square)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 1.0, 0.0),
                custom_size: Some(Vec2::splat(BIRD_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
        Bird { velocity: 0.0 },
    ));

    // Score text
    commands.spawn(
        Text2dBundle {
            text: Text::from_section(
                "Score: 0",
                TextStyle {
                    font_size: 40.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                    ..default()
                },
            ),
            transform: Transform::from_xyz(-300.0, 200.0, 2.0),
            ..default()
        },
    );

    // Ground line (green)
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.0, 1.0, 0.0),
            custom_size: Some(Vec2::new(800.0, 10.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, -300.0, 0.0),
        ..default()
    });

    // Initial pipes
    spawn_pipe(&mut commands, 400.0);
    spawn_pipe(&mut commands, 700.0);

    // Pipe spawning timer
    commands.insert_resource(PipeTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
}

fn spawn_pipe(commands: &mut Commands, x_position: f32) {
    let gap_center = (rand::random::<f32>() - 0.5) * 300.0;
    let gap_center = gap_center.clamp(-200.0, 200.0);

    // Top pipe (dark green)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.0, 0.5, 0.0),
                custom_size: Some(Vec2::new(PIPE_WIDTH, 600.0)),
                ..default()
            },
            transform: Transform::from_xyz(x_position, gap_center + PIPE_GAP / 2.0 + 300.0, 0.0),
            ..default()
        },
        Pipe,
    ));

    // Bottom pipe (dark green)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.0, 0.5, 0.0),
                custom_size: Some(Vec2::new(PIPE_WIDTH, 600.0)),
                ..default()
            },
            transform: Transform::from_xyz(x_position, gap_center - PIPE_GAP / 2.0 - 300.0, 0.0),
            ..default()
        },
        Pipe,
    ));
}

fn bird_movement(
    time: Res<Time>,
    mut bird_query: Query<(&mut Bird, &mut Transform)>,
    game_state: Res<GameState>,
) {
    if game_state.game_over {
        return;
    }

    for (mut bird, mut transform) in bird_query.iter_mut() {
        bird.velocity -= GRAVITY * time.delta_secs();
        transform.translation.y += bird.velocity * time.delta_secs();

        // Keep bird in bounds
        if transform.translation.y < -300.0 + BIRD_SIZE / 2.0 {
            transform.translation.y = -300.0 + BIRD_SIZE / 2.0;
            bird.velocity = 0.0;
        }
    }
}

fn bird_jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut bird_query: Query<&mut Bird>,
    game_state: Res<GameState>,
) {
    if game_state.game_over {
        return;
    }

    for mut bird in bird_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            bird.velocity = JUMP_FORCE;
        }
    }
}

fn pipe_spawning(
    mut commands: Commands,
    time: Res<Time>,
    mut pipe_timer: ResMut<PipeTimer>,
    game_state: Res<GameState>,
) {
    if game_state.game_over {
        return;
    }

    pipe_timer.0.tick(time.delta());
    if pipe_timer.0.is_finished() {
        spawn_pipe(&mut commands, 400.0);
        pipe_timer.0.reset();
    }
}

fn pipe_movement(
    time: Res<Time>,
    mut pipe_query: Query<(Entity, &mut Transform), With<Pipe>>,
    mut commands: Commands,
) {
    for (entity, mut transform) in pipe_query.iter_mut() {
        transform.translation.x -= PIPE_SPEED * time.delta_secs();

        // Remove pipes that are off screen
        if transform.translation.x < -500.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn collision_detection(
    bird_query: Query<&Transform, (With<Bird>, Without<Pipe>)>,
    pipe_query: Query<&Transform, (With<Pipe>, Without<Bird>)>,
    mut game_state: ResMut<GameState>,
) {
    if game_state.game_over {
        return;
    }

    let bird_transform = bird_query.single();
    let bird_pos = bird_transform.translation;
    let bird_size = BIRD_SIZE;

    // Check if bird hits ground or ceiling
    if bird_pos.y <= -300.0 + bird_size / 2.0 || bird_pos.y >= 300.0 {
        game_state.game_over = true;
        return;
    }

    // Check collision with pipes
    for pipe_transform in pipe_query.iter() {
        let pipe_pos = pipe_transform.translation;
        let pipe_size = PIPE_WIDTH;

        // Simple AABB collision
        if bird_pos.x - bird_size / 2.0 < pipe_pos.x + pipe_size / 2.0
            && bird_pos.x + bird_size / 2.0 > pipe_pos.x - pipe_size / 2.0
            && bird_pos.y - bird_size / 2.0 < pipe_pos.y + 300.0
            && bird_pos.y + bird_size / 2.0 > pipe_pos.y - 300.0
        {
            game_state.game_over = true;
            return;
        }
    }
}

fn scoring(
    mut score: ResMut<Score>,
    pipe_query: Query<&Transform, (With<Pipe>, Changed<Transform>)>,
    bird_query: Query<&Transform, With<Bird>>,
) {
    let bird_x = bird_query.single().translation.x;

    for pipe_transform in pipe_query.iter() {
        // Check if bird passed a pipe (simplified scoring)
        if pipe_transform.translation.x < bird_x - 10.0 && pipe_transform.translation.x > bird_x - 15.0 {
            score.value += 1;
        }
    }
}

fn game_over_text(
    mut text_query: Query<&mut Text>,
    score: Res<Score>,
    game_state: Res<GameState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    bird_query: Query<Entity, With<Bird>>,
    pipe_query: Query<Entity, With<Pipe>>,
) {
    let mut text = text_query.single_mut();
    if game_state.game_over {
        text.sections[0].value = format!("Game Over! Score: {}. Press R to restart", score.value);

        // Restart game
        if keyboard_input.just_pressed(KeyCode::KeyR) {
            // Reset game state
            game_state.game_over = false;
            score.value = 0;

            // Clear existing entities
            for entity in bird_query.iter() {
                commands.entity(entity).despawn();
            }
            for entity in pipe_query.iter() {
                commands.entity(entity).despawn();
            }

            // Respawn bird
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(1.0, 1.0, 0.0),
                        custom_size: Some(Vec2::splat(BIRD_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 1.0),
                    ..default()
                },
                Bird { velocity: 0.0 },
            ));

            // Respawn initial pipes
            spawn_pipe(&mut commands, 400.0);
            spawn_pipe(&mut commands, 700.0);
        }
    } else {
        text.sections[0].value = format!("Score: {}", score.value);
    }
}
