use std::ops::{Add, Sub, Mul};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    rows: usize,
    columns: usize,
    matrix: Vec<Vec<T>>
}

impl<T> Matrix<T>
    where T: Copy + Default {
        pub fn new(rows: usize, columns: usize, value: T) -> Self {
            let mut matrix: Vec<Vec<T>> = Vec::new();
            for _ in 0..rows {
                let row = vec![value; columns];
                matrix.push(row);
            }
            Matrix {
                rows,
                columns,
                matrix,
            }
        }

        pub fn from_vector(vector: Vec<Vec<T>>) -> Self {
            let rows = vector.len();
            let columns = vector[0].len();
            for row in 0..rows {
                assert!(vector[row].len() == columns);
            }
            Matrix {
                rows,
                columns,
                matrix: vector,
            }
        }

        pub fn transpose(self) -> Self {
            let mut output = Matrix::new(self.columns, self.rows, Default::default());

            for row in 0..self.rows {
                for column in 0..self.columns {
                    output.matrix[column][row] = self.matrix[row][column];
                }
            }

            output
        }

        pub fn multiply(self, scalar: T) -> Self
            where T: Mul<Output=T> + Copy + Default {
                let mut output = Matrix::new(self.rows, self.columns, Default::default());

                for row in 0..self.rows {
                    for column in 0..self.columns {
                        output.matrix[row][column] = self.matrix[row][column] * scalar;
                    }
                }

                output
            }
    }

impl Matrix<u32> {
    pub fn identity(dimensions: usize) -> Self {
        let mut matrix = Matrix::new(dimensions, dimensions, Default::default());
        for row in 0..dimensions {
            matrix.matrix[row][row] = 1;
        }

        matrix
    }
}

impl<T> fmt::Display for Matrix<T>
    where T: fmt::Display {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for row in 0..self.rows {
                try!(write!(f, "[ "));
                for column in 0..self.columns {
                    try!(write!(f, "{} ", self.matrix[row][column]));
                }
                try!(write!(f, "]"));
                try!(write!(f, "\n"));
            }
            Ok(())
        }
    }

impl<T> Add for Matrix<T>
    where T: Add<Output=T> + Copy + Default {
        type Output = Matrix<T>;

        fn add(self, other: Matrix<T>) -> Self {
            let mut output = Matrix::new(self.rows, self.columns, Default::default());
            assert!(self.columns == other.columns);
            assert!(self.rows == other.rows);

            for row in 0..self.rows {
                for column in  0..self.columns {
                    output.matrix[row][column] = self.matrix[row][column] + other.matrix[row][column];
                }
            }

            output
        }
    }

impl<T> Sub for Matrix<T>
    where T: Sub<Output=T> + Copy + Default {
        type Output = Matrix<T>;

        fn sub(self, other: Matrix<T>) -> Self {
            assert!(self.columns == other.columns);
            assert!(self.rows == other.rows);
            let mut output = Matrix::new(self.rows, self.columns, Default::default());

            for row in 0..self.rows {
                for column in  0..self.columns {
                    output.matrix[row][column] = self.matrix[row][column] - other.matrix[row][column];
                }
            }

            output
        }
    }

impl<T> Mul for Matrix<T>
    where T: Mul<Output=T> + Copy + Default + Add<Output=T> {
        type Output = Matrix<T>;

        fn mul(self, other: Matrix<T>) -> Self {
            assert!(self.columns == other.rows);
            let mut output = Matrix::new(self.rows, other.columns, Default::default());
            for row in 0..self.rows {
                for column in  0..other.columns {
                    let mut total: T = Default::default();
                    for row2 in 0..other.rows {
                        total = total + self.matrix[row][row2] * other.matrix[row2][column];
                    }
                    output.matrix[row][column] = total;
                }
            }
            output
        }
    }

#[cfg(test)]
mod tests {
    use super::*;

    fn get_matrix() -> Matrix<u32> {
        let output = Matrix::new(3, 3, 3);
        output
    }

    #[test]
    fn matrix_generation() {
        let matrix = get_matrix();
        let matrix2 = Matrix::from_vector(vec![
                vec![1,0,1],
                vec![0,1,3]
            ]);
        let identity = Matrix::identity(3);

        assert_eq!(matrix.matrix, vec![
                vec![3,3,3],
                vec![3,3,3],
                vec![3,3,3]
            ]);
        assert_eq!(matrix2.matrix, vec![
                vec![1,0,1],
                vec![0,1,3]
            ]);
        assert_eq!(identity.matrix, vec![
                vec![1,0,0],
                vec![0,1,0],
                vec![0,0,1]
            ]);

        let matrix2 = matrix2.transpose();

        assert_eq!(matrix2.matrix, vec![
                vec![1,0],
                vec![0,1],
                vec![1,3]
            ]);

    }

    #[test]
    fn matrix_operations() {
        let matrix = get_matrix();
        let matrix2 = get_matrix();
        let vector = Matrix::from_vector(vec![
                vec![1],
                vec![1],
                vec![1]
            ]);

        assert_eq!(vec![
                vec![0,0,0],
                vec![0,0,0],
                vec![0,0,0]
            ],
            (matrix.clone() - matrix2.clone()).matrix);
        assert_eq!(vec![
                vec![6,6,6],
                vec![6,6,6],
                vec![6,6,6]
            ],
            (matrix.clone() + matrix2.clone()).matrix);
        assert_eq!(vec![
                vec![9],
                vec![9],
                vec![9]
            ],
            (matrix.clone() * vector).matrix);
        assert_eq!(matrix.multiply(3).matrix, vec![
                vec![9,9,9],
                vec![9,9,9],
                vec![9,9,9]
            ]);

    }
}
