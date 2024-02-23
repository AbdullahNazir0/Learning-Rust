use std::fmt;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "( {0} {1} )\n( {2} {3} )",
            self.0, self.1, self.2, self.3
        )
    }
}

// fn reverse(pair: (f32, f32, f32, f32)) -> (f32, f32, f32, f32) {}

fn transpose(matrix: Matrix) -> Matrix {
    let (first, second, third, fourth) = matrix;

    Matrix(first, third, second, fourth)
}

fn main() {
    let my_matrix = Matrix(1.1, 1.2, 1.3, 1.4);
    // println!("{}", my_matrix);
    // println!("{:?}", my_matrix);
    // println!("{:#?}", my_matrix);
    // let transposed_matrix = transpose(my_matrix);
    println!("matrix:\n{}", my_matrix);
    println!("transpose:\n{}", transpose(my_matrix));
}
