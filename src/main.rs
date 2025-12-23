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

        let q = index.0 as i32 - dim;
        let r = index.1 as i32 - dim;
        let s = r + q;

        if -dim <= q && q <= dim && -dim <= r && r <= dim && -dim <= s && s <= dim {
            *hex = Hex::Valid(ValidHex {
                size: 1,
                in_radius: 1,
                q,
                r,
                s: -q - r, // TODO Error handling for too large dims
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
    q: i32,
    r: i32,
    s: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_boards() {
        check_board(2);
        check_board(5);
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
                assert_eq!((-hex.q - hex.r), hex.s);
                assert_eq!((hex.q + hex.r + hex.s), 0)
            });
    }
}
