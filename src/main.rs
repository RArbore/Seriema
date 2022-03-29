mod ecs;

fn main() {
    let x = ecs::Test { x: 0.0 };
    println!("... {}", x.x);
}
