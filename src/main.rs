use bevy::prelude::*;

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

fn initialize_entities(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.2, 0.2, 0.2),
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(100.0, 100.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Velocity { x: 10.0, y: 10.0 });
}

fn update_pos(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    let dt = time.delta_seconds();
    for (mut pos, vel) in query.iter_mut() {
        pos.translation.x += vel.x * dt;
        pos.translation.y += vel.y * dt;
        println!("{} {}", pos.translation.x, pos.translation.y);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(|mut commands: Commands| {
            commands.spawn_bundle(OrthographicCameraBundle::new_2d());
        })
        .add_startup_system(initialize_entities)
        .add_system(update_pos)
        .run()
}
