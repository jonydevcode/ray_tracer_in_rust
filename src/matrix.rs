use crate::math_utils;
use crate::tuple;

pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub values: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            values: vec![vec![0.0; cols]; rows],
        }
    }

    pub fn identity() -> Self {
        Matrix {
            rows: 4,
            cols: 4,
            values: vec![
                vec![1.0, 0.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0, 0.0],
                vec![0.0, 0.0, 1.0, 0.0],
                vec![0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn from_vec(v: &Vec<Vec<f64>>) -> Self {
        Matrix {
            rows: v.len(),
            cols: v[0].len(),
            values: v.clone(),
        }
    }

    pub fn get_value(&self, row: usize, col: usize) -> f64 {
        self.values[row][col]
    }

    pub fn equals(&self, other: &Self) -> bool {
        if self.rows != other.rows || self.cols != other.cols {
            return false;
        }
        for r in 0..self.rows {
            for c in 0..self.cols {
                if !math_utils::f64_equals(self.get_value(r, c), other.get_value(r, c)) {
                    return false;
                }
            }
        }
        true
    }

    pub fn multiply_matrix(&self, other: &Self) -> Self {
        assert!(!self.values.is_empty() && !other.values.is_empty());
        assert_eq!(self.cols, other.rows);

        let mut result = Matrix::new(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    result.values[i][j] += self.values[i][k] * other.values[k][j];
                }
            }
        }

        result
    }

    pub fn multiply_tuple(&self, other: &tuple::Tuple) -> tuple::Tuple {
        assert!(!self.values.is_empty());
        assert_eq!(self.cols, 4);

        let other_vec = other.to_vec();
        let mut result: [f64; 4] = [0.0; 4];

        for i in 0..self.rows {
            for k in 0..self.cols {
                result[i] += self.values[i][k] * other_vec[k];
            }
        }

        tuple::Tuple {
            x: result[0],
            y: result[1],
            z: result[2],
            w: result[3],
        }
    }

    pub fn transpose(&self) -> Self {
        let mut transposed_matrix = Matrix::new(self.rows, self.cols);

        for i in 0..self.values.len() {
            for j in 0..self.values[i].len() {
                transposed_matrix.values[j][i] = self.values[i][j];
            }
        }

        transposed_matrix
    }

    // next chapter: determining determinants
}

#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;

    use super::*;

    #[test]
    fn creating_matrix() {
        // 4x4 matrix
        let m = Matrix::from_vec(&vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5],
        ]);
        assert_eq!(m.get_value(0, 0), 1.0);
        assert_eq!(m.get_value(0, 3), 4.0);
        assert_eq!(m.get_value(1, 0), 5.5);
        assert_eq!(m.get_value(1, 2), 7.5);
        assert_eq!(m.get_value(2, 2), 11.0);
        assert_eq!(m.get_value(3, 0), 13.5);
        assert_eq!(m.get_value(3, 2), 15.5);

        // 2x2 matrix
        let m = Matrix::from_vec(&vec![
            vec![-3.0, 5.0, 0.0],
            vec![1.0, -2.0, -7.0],
            vec![0.0, 1.0, 1.0],
        ]);
        assert_eq!(m.get_value(0, 0), -3.0);
        assert_eq!(m.get_value(0, 1), 5.0);
        assert_eq!(m.get_value(1, 0), 1.0);
        assert_eq!(m.get_value(1, 1), -2.0);
    }

    #[test]
    fn matrix_equality() {
        let a = Matrix::from_vec(&vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::from_vec(&vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        assert!(a.equals(&b));

        let a = Matrix::from_vec(&vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::from_vec(&vec![
            vec![2.0, 3.0, 4.0, 5.0],
            vec![6.0, 7.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        assert!(!a.equals(&b));
    }

    #[test]
    fn matrix_multiplication() {
        // Two matrices
        let a = Matrix::from_vec(&vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::from_vec(&vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ]);
        let c = Matrix::from_vec(&vec![
            vec![20.0, 22.0, 50.0, 48.0],
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0],
        ]);
        assert!(a.multiply_matrix(&b).equals(&c));

        // Matrix x tuple
        let a = Matrix::from_vec(&vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);
        let b = tuple::Tuple::new(1.0, 2.0, 3.0, 1.0);
        let c = tuple::Tuple::new(18.0, 24.0, 33.0, 1.0);
        assert!(a.multiply_tuple(&b).equals(&c));
    }

    #[test]
    fn identity_matrix() {
        let a = Matrix::from_vec(&vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        assert!(a.multiply_matrix(&Matrix::identity()).equals(&a));

        let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
        assert!(Matrix::identity().multiply_tuple(&a).equals(&a));
    }

    #[test]
    fn matrix_transpose() {
        let mat_a = Matrix::from_vec(&vec![
            vec![0.0, 9.0, 3.0, 0.0],
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0],
        ]);
        let mat_b = Matrix::from_vec(&vec![
            vec![0.0, 9.0, 1.0, 0.0],
            vec![9.0, 8.0, 8.0, 0.0],
            vec![3.0, 0.0, 5.0, 5.0],
            vec![0.0, 8.0, 3.0, 8.0],
        ]);
        assert!(mat_a.transpose().equals(&mat_b));

        // identity transposed == identity
        assert!(Matrix::identity().transpose().equals(&Matrix::identity()));
    }
}
