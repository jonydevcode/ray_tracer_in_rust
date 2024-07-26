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

    pub fn determinant(&self) -> f64 {
        if self.rows == 2 && self.cols == 2 {
            let a = self.values[0][0];
            let b = self.values[0][1];
            let c = self.values[1][0];
            let d = self.values[1][1];
            return a * d - b * c;
        }
        return 0.0;
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Self {
        let mut new_values = Vec::new();

        for i in 0..self.rows {
            if i == row {
                continue;
            }
            let left = &self.values[i][0..col];
            let right = &self.values[i][col + 1..self.cols];
            let combined = left.iter().chain(right.iter()).cloned().collect();
            new_values.push(combined);
        }

        Matrix::from_vec(&new_values)
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        let submat = self.submatrix(row, col);
        submat.determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if row + col % 2 == 1 {
            -minor
        } else {
            minor
        }
    }
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

    #[test]
    fn determinant2x2() {
        let mat_a = Matrix::from_vec(&vec![vec![1.0, 5.0], vec![-3.0, 2.0]]);
        assert!(math_utils::f64_equals(mat_a.determinant(), 17.0));
    }

    #[test]
    fn determinant3x3() {
        let mat_a = Matrix::from_vec(&vec![
            vec![1.0, 2.0, 6.0],
            vec![-5.0, 8.0, -4.0],
            vec![2.0, 6.0, 4.0],
        ]);
        assert!(math_utils::f64_equals(mat_a.cofactor(0, 0), 56.0));
        assert!(math_utils::f64_equals(mat_a.cofactor(0, 1), 12.0));
        assert!(math_utils::f64_equals(mat_a.cofactor(0, 2), -46.0));
        assert!(math_utils::f64_equals(mat_a.determinant(), -196.0));
    }

    #[test]
    fn determinant4x4() {
        let mat_a = Matrix::from_vec(&vec![
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
            vec![1.0, 2.0, -9.0, 6.0],
            vec![-6.0, 7.0, 7.0, -9.0],
        ]);
        assert!(math_utils::f64_equals(mat_a.cofactor(0, 0), 690.0));
        assert!(math_utils::f64_equals(mat_a.cofactor(0, 1), 447.0));
        assert!(math_utils::f64_equals(mat_a.cofactor(0, 2), 210.0));
        assert!(math_utils::f64_equals(mat_a.cofactor(0, 3), 51.0));
        assert!(math_utils::f64_equals(mat_a.determinant(), -4071.0));
    }

    #[test]
    fn submatrix() {
        let mat_a = Matrix::from_vec(&vec![
            vec![1.0, 5.0, 0.0],
            vec![-3.0, 2.0, 7.0],
            vec![0.0, 6.0, -3.0],
        ]);
        let mat_b = Matrix::from_vec(&vec![vec![-3.0, 2.0], vec![0.0, 6.0]]);

        assert!(mat_a.submatrix(0, 2).equals(&mat_b));

        let mat_a = Matrix::from_vec(&vec![
            vec![-6.0, 1.0, 1.0, 6.0],
            vec![-8.0, 5.0, 8.0, 6.0],
            vec![-1.0, 0.0, 8.0, 2.0],
            vec![-7.0, 1.0, -1.0, 1.0],
        ]);
        let mat_b = Matrix::from_vec(&vec![
            vec![-6.0, 1.0, 6.0],
            vec![-8.0, 8.0, 6.0],
            vec![-7.0, -1.0, 1.0],
        ]);

        assert!(mat_a.submatrix(2, 1).equals(&mat_b));
    }

    #[test]
    fn minor() {
        let mat_a = Matrix::from_vec(&vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);
        let mat_b = mat_a.submatrix(1, 0);

        assert!(math_utils::f64_equals(mat_b.determinant(), 25.0));
        assert!(math_utils::f64_equals(mat_a.minor(1, 0), 25.0));
    }

    #[test]
    fn cofactor() {
        let mat_a = Matrix::from_vec(&vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);

        assert!(math_utils::f64_equals(mat_a.minor(0, 0), -12.0));
        assert!(math_utils::f64_equals(mat_a.cofactor(0, 0), -12.0));
        assert!(math_utils::f64_equals(mat_a.minor(1, 0), 25.0));
        assert!(math_utils::f64_equals(mat_a.cofactor(1, 0), -25.0));
    }
}
