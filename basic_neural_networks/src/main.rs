mod linalg;
mod neuralnetworks;

use linalg::Matrix;
use neuralnetworks::Layer;

fn main() {
    let mut matrix1: Matrix<f64> = Matrix::new(3, 3);
    matrix1.set(0, 0, 1.0);
    matrix1.set(0, 1, 3.0);
    matrix1.set(0, 2, 5.0);
    matrix1.set(1, 0, 0.0);
    matrix1.set(1, 1, -1.0);
    matrix1.set(1, 2, 3.0);
    matrix1.set(2, 0, 0.0);
    matrix1.set(2, 1, 0.0);
    matrix1.set(2, 2, 10.0);
    let mut matrix2: Matrix<f64> = Matrix::new(1, 3);
    matrix2.set(0, 0, 2.0);
    matrix2.set(0, 1, 5.0);
    matrix2.set(0, 1, 5.0);
    let matrix3: Matrix<f64> = &matrix2 * &matrix1;
    let five: f64 = 5.0;
    let matrix4: Matrix<f64> = &matrix1 * five;
    println!("{} {} {}", matrix1.get(0,0), matrix1.get(0, 1), matrix1.get(0, 2));
    println!("{} {} {}", matrix1.get(1,0), matrix1.get(1, 1), matrix1.get(1, 2));
    println!("{} {} {}", matrix1.get(2,0), matrix1.get(2, 1), matrix1.get(2, 2));
    println!("");
    println!("{} {} {}", matrix2.get(0,0), matrix2.get(0, 1), matrix2.get(0, 2));
    println!("");
    println!("{} {} {}", matrix3.get(0,0), matrix3.get(0, 1), matrix3.get(0, 2));
    println!("");
    println!("{} {} {}", matrix4.get(0,0), matrix4.get(0, 1), matrix4.get(0, 2));
    println!("{} {} {}", matrix4.get(1,0), matrix4.get(1, 1), matrix4.get(1, 2));
    println!("{} {} {}", matrix4.get(2,0), matrix4.get(2, 1), matrix4.get(2, 2));
    let mut input1: Matrix<f64> = Matrix::new(1, 3);
    input1.set(0, 0, 0.5);
    input1.set(0, 1, 15.0);
    input1.set(0, 2, -5.0);
    let layer = Layer::new(matrix1, matrix2, neuralnetworks::tansig);
    let output1 = layer.apply(&input1);
    println!("{} {} {}", input1.get(0,0), input1.get(0, 1), input1.get(0, 2));
    println!("{} {} {}", output1.get(0,0), output1.get(0, 1), output1.get(0, 2));

}
