use std::{ops::{Add, Sub, Mul, AddAssign} , usize};
use num::Num;

const SHAPE_MISMATCH : &str = "Matrix shape must be the same!";
const COLUMNS_ROWS_MISMATCH : &str = "Columns of the first matrix must match rows of the second matrix!";

#[derive(Debug, Clone)]
pub struct Matrix<T: Default + Copy + Clone + Num + AddAssign> {
    rows: usize,
    columns: usize,
    values: Vec<T>
}

impl<T: Default + Copy + Clone + Num + AddAssign> Matrix<T> {
    pub fn new(rows: usize, columns: usize) -> Self {
        return Matrix{
            rows,
            columns,
            values : vec![Default::default(); rows * columns]
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn size(&self) -> usize {
        self.rows * self.columns
    }

    pub fn index(&self, row: usize, column: usize) -> usize {
        (row * self.columns) + column
    }

    pub fn get(&self, row: usize, column: usize) -> T {
        self.values[self.index(row,column)]
    }

    pub fn set(&mut self, row: usize, column:usize, value: T) {
        let index: usize = self.index(row, column);
        self.values[index] = value;
    }
}

impl<T: Default + Copy + Clone + Num + AddAssign> Add<&Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, other: &Matrix<T>) -> Matrix<T>{
        if self.rows != other.rows || self.columns != other.columns {
            panic!("{}", SHAPE_MISMATCH);
        }
        let mut values: Vec<T> = Vec::new();
        for i in 0..self.size() {
            values.push(self.values[i] + other.values[i]);
        }
        Matrix { rows: self.rows, columns: self.columns, values }
    }
}

impl<T: Default + Copy + Clone + Num + AddAssign> Sub<&Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;

    fn sub(self, other: &Matrix<T>) -> Self::Output {
        if self.rows != other.rows || self.columns != other.columns {
            panic!("{}", SHAPE_MISMATCH);
        }
        let mut values: Vec<T> = Vec::new();
        for i in 0..self.size() {
            values.push(self.values[i] - other.values[i])
        }
        Matrix { rows: self.rows, columns: self.columns, values }
    }
}

impl<T: Default + Copy + Clone + Num + AddAssign> Mul<&Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, other: &Matrix<T>) -> Self::Output {
        if self.columns != other.rows {
            panic!("{}", COLUMNS_ROWS_MISMATCH);
        }
        let mut values: Vec<T> = Vec::new();
        for i in 0..self.rows {
            for j in 0..other.columns {             
                let mut val : T = Default::default();
                for k in 0..self.columns {
                    val += self.values[self.index(i, k)] * other.values[other.index( k, j)]
                }
                values.push(val);
            }
        }
        Matrix {
            rows : self.rows,
            columns : other.columns,
            values
        }
    }
}

impl<T: Default + Copy + Clone + Num + AddAssign> Mul<T> for &Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, mat: T) -> Self::Output {
        let mut values: Vec<T> = Vec::new();
        for i in 0..self.size() {
            values.push(self.values[i] * mat);
        }
        Matrix { rows: self.rows, columns: self.columns, values}
    }
}
