use na::{U2, U3, Dynamic, MatrixArray, MatrixVec};
use typenum::U1000;

pub struct Board {
    pub height: usize,
    pub width: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        Board { width: width, height: height }
    }

    fn build() {
        type Matrix2x3f = Matrix<f32, U2, U3, MatrixArray<f32, U2, U3>>;
        let b = Matrix2x3::from_fn(|r, c| (r + 1) as f32 + (c + 1) as f32 / 10.0);
    }

    // fn get_cell(&self, position: &Place) -> Option<Cell> {
    // }

    // fn has_cell(&self, position: &Place) -> bool {
    // }
}

/* fn count_neighbours(cells: &Vec<Cell>, position: &Place) -> usize {
    let mut neighbours = 0;

    for cell in cells {
        if cell.is_dead() {
            continue;
        }

        let distance = distance(cell.get_position(), position);

        if distance > 0.0 && distance < 2.0 {
            neighbours += 1;
        }
    }

    neighbours
}*/