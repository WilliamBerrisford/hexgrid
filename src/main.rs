use ndarray::Array3;

fn main() {
    let board = create_array(2, 2, 2);

    println!("{:?}", board);
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

    Board { grid: arr }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_boards() {
        check_board(2, 2, 2);
        check_board(99, 1, 3);
        check_board(100, 100, 100);
    }

    fn check_board(x: usize, y: usize, z: usize) {
        let board = create_array(x, y, z);
        assert_eq!(board.grid.dim(), (x, y, z));
    }
}
