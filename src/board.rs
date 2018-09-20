use na::{U2, U3, MatrixArray, Matrix};

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
        let _m = Matrix2x3f::new(11 as f32, 12 as f32, 13 as f32,
                                 14 as f32, 31 as f32, 32 as f32);
    }

    fn get_cell(&self, position: &Place) -> Option<Cell> {
    }

    fn has_cell(&self, position: &Place) -> bool {
    }

    fn count_neighbours(cells: &Vec<Cell>, position: &Place) -> usize {
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
    }
}
