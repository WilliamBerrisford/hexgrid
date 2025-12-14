use std::default;

use ndarray::{Array3, iter};

fn main() {
    println!("Hello, world!");
}

fn create_array(max_x: usize, max_y: usize, max_z: usize) -> Board {
    let arr: Array3<Hex> = Array3::<Hex>::from_elem((max_x, max_y, max_z), Hex::default());
    todo!()
}

struct Board {
    grid: Array3<Hex>,
}

#[derive(Clone, Default)]
struct Hex {
    size: i32,
    in_radius: i32,
    x: i32,
    y: i32,
    z: i32,
}
