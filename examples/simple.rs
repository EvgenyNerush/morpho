use morpho::{X, Y, Z, SimpleV3};

fn main() {
    println!("Hello, world!");
    let mut v = SimpleV3 { x: 1.0, y: 2.0, z: 3.0 };
    println!("x, y, z = {}, {}, {}", v.x, v.y, v.z);
    v.x = 8.0;
    println!("x, y, z = {}, {}, {}", v.x, v.y, v.z);
}
