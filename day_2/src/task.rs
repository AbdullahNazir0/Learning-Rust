use std::fmt;

#[derive(Debug)]
struct Matrix([[f64; 2]; 2]);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{0} {1}\n{2} {3}",
            self.0[0][0], self.0[0][1], self.0[1][0], self.0[1][1]
        )
    }
}

fn add_matrices(matrix1: Matrix, matrix2: Matrix) -> Matrix {
    let mut result = Matrix([[0.0, 0.0], [0.0, 0.0]]);
    for i in 0..2 {
        for j in 0..2 {
            result.0[i][j] = matrix1.0[i][j] + matrix2.0[i][j];
        }
    }
    result
}

fn transpose_matrix(matrix: Matrix) -> Matrix {
    let result = Matrix([
        [matrix.0[0][0], matrix.0[1][0]],
        [matrix.0[0][1], matrix.0[1][1]],
    ]);

    result
}

fn multiply_matrices(matrix1: Matrix, matrix2: Matrix) -> Matrix {
    let mut result = Matrix([[0.0; 2]; 2]);
    for i in 0..2 {
        for j in 0..2 {
            result.0[i][j] = matrix1.0[i][0] * matrix2.0[0][j] + matrix1.0[i][1] * matrix2.0[1][j];
        }
    }

    result
}

fn extract_submatrix<'a>(matrix: &'a Matrix, row_start: usize, row_end: usize) -> &'a [[f64; 2]] {
    // For now lets assume that the given row start and end is correct.
    &matrix.0[row_start..row_end]
}

fn main() {
    // Create two matrices
    let matrix1 = Matrix([[1.0, 2.0], [3.0, 4.0]]);
    let matrix2 = Matrix([[5.0, 6.0], [7.0, 8.0]]);

    // Display the original matrices
    println!("Matrix 1:\n{}", matrix1);
    println!("Matrix 2:\n{}", matrix2);

    // Test addition of matrices
    let sum_matrix = add_matrices(matrix1, matrix2);
    println!("Sum of matrices:\n{}", sum_matrix);

    // Test matrix transposition
    let transposed_matrix = transpose_matrix(matrix1);
    println!("Transposed Matrix 1:\n{}", transposed_matrix);

    // Test matrix multiplication
    let product_matrix = multiply_matrices(matrix1, matrix2);
    println!("Product of matrices:\n{}", product_matrix);

    // Test extracting a submatrix
    let submatrix = extract_submatrix(&matrix1, 0, 1);
    println!("Submatrix of Matrix 1:\n{:?}", submatrix);
}
