use anyhow::Error;
use ndarray::Array2;

#[derive(Debug)]
pub struct Board {
    pub grid: Array2<Hex>,
}

impl Board {
    pub fn create_with_dim(dim: usize) -> Board {
        let mut arr: Array2<Hex> =
            Array2::<Hex>::from_elem(((2 * dim) + 1, (2 * dim) + 1), Hex::default());
        arr.indexed_iter_mut().for_each(|(index, hex)| {
            let dim = dim as i32;

            let q = index.0 as i32 - dim;
            let r = index.1 as i32 - dim;
            let s = r + q;

            //if -dim <= q && q <= dim && -dim <= r && r <= dim && -dim <= s && s <= dim {
            *hex = Hex::Valid(ValidHex {
                size: 1,
                in_radius: 1,
                q,
                r,
                s: -q - r,
                centre: (q == 0 && r == 0 && -q - r == 0),
            });
            //}
        });

        Board { grid: arr }
    }
}

#[derive(Clone, Default, Debug)]
pub enum Hex {
    Valid(ValidHex),
    #[default]
    Invalid,
}

impl ValidHex {
    // axial → odd-r offset (works for drawing only)
    fn to_offset(&self) -> (i32, i32) {
        //let col = self.q + (self.r - (self.r & 1)) / 2;
        //let row = self.r;
        let col = 3 / 2 * self.q;
        let row = ((f64::sqrt(3.0) / 2.0 * self.q as f64) + f64::sqrt(3.0) * self.r as f64).round()
            as i32;
        (col, row)
    }

    // pixel centre of the hex (cell size 4×4 chars)
    pub fn to_pixel(&self, width: i32, height: i32) -> (i32, i32) {
        let (x, y) = self.to_offset();

        let cx = width / 2;
        let cy = height / 2;

        let px = cx + x * 7; // 7 cols per hex
        let py = cy + y * 4 + (x & 1) * 2; // stagger rows
        (px, py)
    }

    pub fn from_centre(&self) -> i32 {
        (self.q.abs() + self.r.abs() + self.s.abs()) / 2
    }
}

#[derive(Clone, Default, Debug)]
pub struct ValidHex {
    pub size: i32,
    pub in_radius: i32,
    pub q: i32,
    pub r: i32,
    pub s: i32,
    pub centre: bool,
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
        let board = Board::create_with_dim(x);
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
