use crate::linalg::Matrix;
use std::f64::consts::E;

pub fn purelin(input: &Matrix<f64>) -> Matrix<f64> {
    input.clone()
}

pub fn tansig(input: &Matrix<f64>) -> Matrix<f64> {
    let mut output = input.clone();
    for i in 0..output.rows() {
        for j in 0..output.columns() {
            output.set(i, j, 2.0/(1.0 + E.powf(-2.0*output.get(i, j))) - 1.0);
        }
    }
    output
}

pub fn logsig(input: &Matrix<f64>) -> Matrix<f64> {
    let mut output = input.clone();
    for i in 0..output.rows() {
        for j in 0..output.columns() {
            output.set(i, j, 1.0/(1.0 + E.powf(-output.get(i, j))));
        }
    }
    output 
}

pub struct Layer {
    weights: Matrix<f64>,
    biases: Matrix<f64>,
    activation: fn(&Matrix<f64>) -> Matrix<f64>,
}

impl Layer {
    pub fn new(weights: Matrix<f64>, biases: Matrix<f64>, activation: fn(&Matrix<f64>) -> Matrix<f64>) -> Layer {
        Layer { weights, biases, activation}
    }
    pub fn apply(&self, input: &Matrix<f64>) -> Matrix<f64> {
        let out: Matrix<f64> = &(input * &self.weights) + &self.biases;
        (self.activation)(&out)
    }
}