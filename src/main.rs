use std::default;

use ndarray::{Array3, iter};

fn main() {
    create_array(2, 2, 2);
}

fn create_array(max_x: usize, max_y: usize, max_z: usize) -> Board {
    let mut arr: Array3<Hex> = Array3::<Hex>::from_elem((max_x, max_y, max_z), Hex::default());
    arr.indexed_iter_mut().for_each(|(index, hex)| {
        *hex = Hex {
            size: 1,
            in_radius: 1,
            x: index.0,
            y: index.1,
            z: index.2,
        }
    });

    println!("Array {:?}", arr);
    todo!();
}

#[derive(Debug)]
struct Board {
    grid: Array3<Hex>,
}

#[derive(Clone, Default, Debug)]
struct Hex {
    size: i32,
    in_radius: i32,
    x: usize,
    y: usize,
    z: usize,
}
