// ML
use rand::Rng;

/// Credit for this code goes to codemoon on YouTube. I followed along with the video

pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>
}

pub fn randomm(rows: usize, cols: usize) -> Matrix {
    let mut rbuffer = Vec::<f64>::with_capacity(rows * cols);

    // Generate a random number for each value that will be in the Matrix 
    for _ in 0..rows*cols {
        let num = rand::thread_rng().gen_range(0.0..1.0);

        rbuffer.push(num);
    }

    // Build the matrix and return it
    Matrix{rows,cols,data:rbuffer}
}

pub fn addm (&self, other: &Matrix) -> Matrix {
    if self.rows != other.rows || self.cols != other.cols {
        panic!("Attempted to add matrix of incorrect dimensions") //// REPLACE THIS FOR PRODUCTION
    }
    
    let mut abuffer = Vec::<f64>::with_capacity(self.rows * self.cols);

    for i in 0..self.data.len() {
        let result = self.data[i] + other.data[i];

        abuffer.push(result)
    }

    Matrix {
        rows: self.rows,
        cols: self.cols,
        data: abuffer
    }
}

fn main() {

}