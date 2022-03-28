use bevy::prelude::*;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

fn add_player(mut commands: Commands) {
    commands
        .spawn()
        .insert(Position { x: 0.0, y: 0.0 })
        .insert(Velocity { x: 1.0, y: 1.0 });
}

fn update_pos(time: Res<Time>, mut query: Query<(&mut Position, &Velocity)>) {
    let dt = time.delta_seconds();
    for (mut pos, vel) in query.iter_mut() {
        pos.x += vel.x * dt;
        pos.y += vel.y * dt;
        println!("{} {}", pos.x, pos.y);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_player)
        .add_system(update_pos)
        .run()
}
