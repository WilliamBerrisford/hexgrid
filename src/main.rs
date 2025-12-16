use ndarray::Array2;

fn main() {
    let board = create_hex_board(4);

    println!("{:?}", board);
}

fn create_hex_board(dim: usize) -> Board {
    let mut arr: Array2<Hex> =
        Array2::<Hex>::from_elem(((2 * dim) + 1, (2 * dim) + 1), Hex::default());
    arr.indexed_iter_mut().for_each(|(index, hex)| {
        let dim = dim as i32;

        let row = index.0 as i32 - dim;
        let column = index.1 as i32 - dim;
        let combined = column + row;

        if -dim <= row
            && row <= dim
            && -dim <= column
            && column <= dim
            && -dim <= combined
            && combined <= dim
        {
            *hex = Hex::Valid(ValidHex {
                size: 1,
                in_radius: 1,
                x: row,
                y: column,
                z: -row - column, // TODO Error handling for too large dims
            });
        }
    });

    Board { grid: arr }
}

#[derive(Debug)]
struct Board {
    grid: Array2<Hex>,
}

#[derive(Clone, Default, Debug)]
enum Hex {
    Valid(ValidHex),
    #[default]
    Invalid,
}

#[derive(Clone, Default, Debug)]
struct ValidHex {
    size: i32,
    in_radius: i32,
    x: i32,
    y: i32,
    z: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_boards() {
        check_board(2);
        check_board(8);
        check_board(99);
        check_board(100);
    }

    fn check_board(x: usize) {
        let board = create_hex_board(x);
        board
            .grid
            .iter()
            .filter_map(|hex| match hex {
                Hex::Valid(valid_hex) => Some(valid_hex),
                Hex::Invalid => None,
            })
            .for_each(|hex| {
                dbg!(hex);
                assert_eq!((-hex.x - hex.y), hex.z);
                assert_eq!((hex.x + hex.y + hex.z), 0)
            });
    }
}
