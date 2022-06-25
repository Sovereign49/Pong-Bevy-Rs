use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use impacted::CollisionShape;
use rand::seq::SliceRandom;

const PADDLE_SPEED: f32 = 275.0;
const BALL_SPEED: f32 = 200.0;

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Left;

#[derive(Component)]
struct Right;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Dir(f32, f32);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup)
        .add_system(left_movement)
        .add_system(right_movement)
        .add_system(ball_physics)
        .add_system(collision)
        .run();
}

fn setup(mut commands: Commands) {
    let rect = shapes::Rectangle {
        extents: Vec2::new(15.0, 150.0),
        origin: shapes::RectangleOrigin::Center,
    };
    let circle = shapes::Circle {
        radius: 10.0,
        center: Vec2::new(0.0, 0.0),
    };
    let dirs: [f32; 2] = [-1.0, 1.0];
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
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
}

fn left_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Left>>,
) {
    let mut pos = query.single_mut();
    let mut dir = 0.0;

    if keyboard_input.pressed(KeyCode::W) {
        dir = 1.0;
    } else if keyboard_input.pressed(KeyCode::S) {
        dir = -1.0;
    } else {
        dir = 0.0;
    }

    pos.translation.y += dir * PADDLE_SPEED * time.delta_seconds();

    if pos.translation.y > 280.0 {
        pos.translation.y = 280.0;
    }
    if pos.translation.y < -280.0 {
        pos.translation.y = -280.0
    }
}

fn right_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Right>>,
) {
    let mut pos = query.single_mut();
    let mut dir = 0.0;

    if keyboard_input.pressed(KeyCode::Up) {
        dir = 1.0;
    } else if keyboard_input.pressed(KeyCode::Down) {
        dir = -1.0;
    } else {
        dir = 0.0;
    }

    pos.translation.y += dir * PADDLE_SPEED * time.delta_seconds();

    if pos.translation.y > 280.0 {
        pos.translation.y = 280.0;
    }
    if pos.translation.y < -280.0 {
        pos.translation.y = -280.0
    }
}

fn ball_physics(
    time: Res<Time>,
    mut ball_query: Query<&mut Transform, With<Ball>>,
    mut dir_query: Query<&mut Dir>,
) {
    let mut pos = ball_query.single_mut();
    let mut dir = dir_query.single_mut();
    let dirs = [-1.0, 1.0];

    if pos.translation.y > 350.0 {
        pos.translation.y = 350.0;
        dir.1 = -1.0 * dir.1;
    }
    if pos.translation.y < -350.0 {
        pos.translation.y = -350.0;
        dir.1 = -1.0 * dir.1;
    }
    if pos.translation.x > 625.0 {
        pos.translation.y = 0.0;
        pos.translation.x = 0.0;
        dir.0 = *dirs.choose(&mut rand::thread_rng()).unwrap() as f32;
        dir.1 = *dirs.choose(&mut rand::thread_rng()).unwrap() as f32;
    }
    if pos.translation.x < -625.0 {
        pos.translation.y = 0.0;
        pos.translation.x = 0.0;
        dir.0 = *dirs.choose(&mut rand::thread_rng()).unwrap() as f32;
        dir.1 = *dirs.choose(&mut rand::thread_rng()).unwrap() as f32;
    }

    pos.translation.x += dir.0 * BALL_SPEED * time.delta_seconds();
    pos.translation.y += dir.1 * BALL_SPEED * time.delta_seconds();
}

fn collision(
    ball_query: Query<&Transform, With<Ball>>,
    paddle_query: Query<&Transform, With<Paddle>>,
    mut dir_query: Query<&mut Dir>,
) {
    let ball = ball_query.single();
    let mut dir = dir_query.single_mut();
    for paddle in paddle_query.iter() {
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
