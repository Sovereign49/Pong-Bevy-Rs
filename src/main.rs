// Include Libraries
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::seq::SliceRandom;

// Set up constant values
const PADDLE_SPEED: f32 = 375.0;
const BALL_SPEED: f32 = 300.0;

//Set up resources and components
enum GameState {
    Start,
    Game,
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Left;

#[derive(Component)]
struct Right;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Score;

#[derive(Component)]
struct Title;

#[derive(Component)]
struct Dir(f32, f32);

struct State(GameState);

struct Scoreboard {
    p1: usize,
    p2: usize,
}

// Add all plugins and Resources
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(Scoreboard { p1: 0, p2: 0 })
        .insert_resource(State(GameState::Start))
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup)
        .add_system(start_manager)
        .add_system(left_movement)
        .add_system(right_movement)
        .add_system(ball_physics)
        .add_system(collision)
        .add_system(scoreboard)
        .add_system(end_manager)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create Shape for Paddles
    let rect = shapes::Rectangle {
        extents: Vec2::new(15.0, 150.0),
        origin: shapes::RectangleOrigin::Center,
    };
    // Create Shape for Ball
    let circle = shapes::Circle {
        radius: 10.0,
        center: Vec2::new(0.0, 0.0),
    };
    // Create list of directions that will be randomly chosen 
    let dirs: [f32; 2] = [-1.0, 1.0];
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // Create Left Paddle
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &rect,
            DrawMode::Fill(FillMode {
                options: FillOptions::DEFAULT,
                color: Color::rgb(0.75, 0.75, 0.75),
            }),
            Transform::from_xyz(-600.0, 0.0, 0.0),
        ))
        .insert(Paddle)
        .insert(Left);
    // Create Right Paddle
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &rect,
            DrawMode::Fill(FillMode {
                options: FillOptions::DEFAULT,
                color: Color::rgb(0.75, 0.75, 0.75),
            }),
            Transform::from_xyz(600.0, 0.0, 0.0),
        ))
        .insert(Paddle)
        .insert(Right);
    // Create Ball
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &circle,
            DrawMode::Fill(FillMode {
                options: FillOptions::DEFAULT,
                color: Color::rgb(0.75, 0.75, 0.75),
            }),
            Transform::from_xyz(0.0, 0.0, 0.0),
        ))
        .insert(Ball)
        .insert(Dir(
            *dirs.choose(&mut rand::thread_rng()).unwrap() as f32,
            *dirs.choose(&mut rand::thread_rng()).unwrap() as f32,
        ));
    // Create Scoreboard
    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "P1: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "0".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::GOLD,
                        },
                    },
                    TextSection {
                        value: " P2: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "0".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..default()
            },
            transform: Transform::from_xyz(-125.0, 350.0, 1.0),
            ..default()
        })
        .insert(Score);
    // Create Title Text
    commands
        .spawn_bundle(Text2dBundle {
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "     Pong!\n ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 120.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "Press Space to Start".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 60.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..default()
            },
            transform: Transform::from_xyz(-250.0, 150.0, 0.0),
            ..default()
        })
        .insert(Title);
}

// Set up start screen
fn start_manager(
    mut state: ResMut<State>,
    mut scores: ResMut<Scoreboard>,
    input: Res<Input<KeyCode>>,
    mut title_query: Query<&mut Transform, With<Title>>,
) {
    let mut title = title_query.single_mut();
    if let GameState::Start = state.0 {
        // Move title to Screen
        title.translation.y = 150.0;
        if input.pressed(KeyCode::Space) {
            // Set up Game by reseting the score and moving the title off screen, then changing the game state
            title.translation.y = -10000.0;
            scores.p1 = 0;
            scores.p2 = 0;
            state.0 = GameState::Game;
        }
    }
}

fn left_movement(
    time: Res<Time>,
    state: Res<State>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Left>>,
) {
    if let GameState::Game = state.0 {
        let mut pos = query.single_mut();
        let mut dir = 0.0;

        // Change the paddle Direction depending on what key is pressed
        if keyboard_input.pressed(KeyCode::W) {
            dir = 1.0;
        } else if keyboard_input.pressed(KeyCode::S) {
            dir = -1.0;
        } else {
            dir = 0.0;
        }

        // Move paddle
        pos.translation.y += dir * PADDLE_SPEED * time.delta_seconds();

        // Clamp paddle to screen size
        if pos.translation.y > 280.0 {
            pos.translation.y = 280.0;
        }
        if pos.translation.y < -280.0 {
            pos.translation.y = -280.0
        }
    }
}

fn right_movement(
    time: Res<Time>,
    state: Res<State>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Right>>,
) {
    if let GameState::Game = state.0 {
        let mut pos = query.single_mut();
        let mut dir = 0.0;

        // Change the paddle Direction depending on what key is pressed
        if keyboard_input.pressed(KeyCode::Up) {
            dir = 1.0;
        } else if keyboard_input.pressed(KeyCode::Down) {
            dir = -1.0;
        } else {
            dir = 0.0;
        }

        // Move paddle
        pos.translation.y += dir * PADDLE_SPEED * time.delta_seconds();

        // Clamp paddle to screen size
        if pos.translation.y > 280.0 {
            pos.translation.y = 280.0;
        }
        if pos.translation.y < -280.0 {
            pos.translation.y = -280.0
        }
    }
}

fn ball_physics(
    time: Res<Time>,
    state: Res<State>,
    mut scores: ResMut<Scoreboard>,
    mut ball_query: Query<&mut Transform, With<Ball>>,
    mut dir_query: Query<&mut Dir>,
) {
    if let GameState::Game = state.0 {
        let mut pos = ball_query.single_mut();
        let mut dir = dir_query.single_mut();
        let dirs = [-1.0, 1.0];

        // Set up collisions with top and bottom sides of the screen
        if pos.translation.y > 350.0 {
            pos.translation.y = 350.0;
            dir.1 = -1.0 * dir.1;
        }
        if pos.translation.y < -350.0 {
            pos.translation.y = -350.0;
            dir.1 = -1.0 * dir.1;
        }
        // Reset the ball & Increment the score when ball passes a paddle 
        if pos.translation.x > 625.0 {
            scores.p1 += 1;
            pos.translation.y = 0.0;
            pos.translation.x = 0.0;
            dir.0 = *dirs.choose(&mut rand::thread_rng()).unwrap() as f32;
            dir.1 = *dirs.choose(&mut rand::thread_rng()).unwrap() as f32;
        }
        if pos.translation.x < -625.0 {
            scores.p2 += 1;
            pos.translation.y = 0.0;
            pos.translation.x = 0.0;
            dir.0 = *dirs.choose(&mut rand::thread_rng()).unwrap() as f32;
            dir.1 = *dirs.choose(&mut rand::thread_rng()).unwrap() as f32;
        }
        
        // Move the ball
        pos.translation.x += dir.0 * BALL_SPEED * time.delta_seconds();
        pos.translation.y += dir.1 * BALL_SPEED * time.delta_seconds();
    }
}

fn collision(
    ball_query: Query<&Transform, With<Ball>>,
    paddle_query: Query<&Transform, With<Paddle>>,
    mut dir_query: Query<&mut Dir>,
) {
    let ball = ball_query.single();
    let mut dir = dir_query.single_mut();
    for paddle in paddle_query.iter() {
        // Check if ball is within paddle bounds
        if (ball.translation.x >= (paddle.translation.x - 15.0)
            && ball.translation.x <= (paddle.translation.x + 15.0))
            && (ball.translation.y >= (paddle.translation.y - (150.0 / 2.0))
                && ball.translation.y <= (paddle.translation.y + (150.0 / 2.0)))
        {
            dir.0 = -dir.0;
            dir.1 = -dir.1;
        }
    }
}

fn scoreboard(scores: Res<Scoreboard>, mut scoreboard_query: Query<&mut Text, With<Score>>) {
    let mut scoreboard = scoreboard_query.single_mut();
    // Update scoreboard text with current score
    scoreboard.sections[1].value = format!("{}", scores.p1);
    scoreboard.sections[3].value = format!("{}", scores.p2);
}

fn end_manager(scores: Res<Scoreboard>, mut state: ResMut<State>, mut query: Query<&mut Transform, With<Paddle>>) {
    // Detect it there has been a game over
    if scores.p1 >= 10 || scores.p2 >= 10 {
        for mut pos in query.iter_mut() {
            pos.translation.y = 0.0;
        }
        state.0 = GameState::Start;
    }
}
