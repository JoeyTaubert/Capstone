// ML - RNN
/// Reference: codemoon on YouTube, https://github.com/codemoonsxyz/neural-net-rs
use rand::Rng;



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

pub fn subtractm(&self, other: &Matrix) -> Matrix {

    // Verify that the matrices are of equal dimensions
    assert!(
        self.rows == other.rows && self.cols == other.cols,
        "Cannot subtract matricies with different dimensions"
    );

    let mut sbuffer = Vec::<f64>::with_capacity(self.rows * self.cols);

    for i in 0..self.data.len() {

        let result = self.data[i] - other.data[i];

        sbuffer.push(result);
    }

    Matrix {
        rows: self.rows,
        cols: self.cols,
        data: sbuffer
    }
}

pub fn dot_multiply(&self, other: &Matrix) -> Matrix {
    if self.cols != other.rows {
        panic!("Attempted to multiply by matrix of incorrect dimensions!") //// REPLACE THIS FOR PRODUCTION
    }

    let mut result_data = vec![0.0; self.rows * other.cols];

    for i in 0..self.rows {
        for j in 0..other.cols {
            let mut sum = 0.0;
            for k in 0..self.cols {
                sum += self.data[i * self.cols + k] * other.data[k * other.cols + j];
            }
            result_data[i * other.cols + j] = sum;
        }
    }

    Matrix {
        rows: self.rows,
        cols: other.cols,
        data: result_data,
    }
}

pub struct NNetwork {
    layers: Vec<usize>, // amount of neurons per layer ex: [2, 4, 1]
    weights: Vec<Matrix>,
    biases: Vec<Matrix>,
    data: Vec<Matrix>,
    activation: Activation,
    learning_rate: f64,
}

impl NNetwork {
    pub fn new(layers: Vec<usize>, activation:Activation, learning_rate:f64) -> Self {
        // Initialize vectors to hold weights and biases
        let mut weights = vec![];
        let mut biases = vec![];

        // Iterate over layers of the neural network
        for i in 00..layers.len() -1 {
            weights.push(randomm(layers[i+1], layers[i]));
            biases.push(randomm(layers[i+1], 1));
        }

        // Initialize the network
        NNetwork {
            layers,
            weights,
            biases,
            data: vec![],
            activation,
            learning_rate
        }
    }

    pub fn feed_forward(&mut self, inputs: Matrix) -> Matrix {
        assert!(self.layers[0] == inputs.data.len(), "Invalid number of inputs");

        // Holds values for next layer of neural network
        let mut current = inputs;

        self.data = vec![current.clone()];

        // Iterate over all layers
        for i in 0..self.layers.len() - 1 {
            // Apply feed forward algorithm
            current = self.weights[i]
            .dot_multiply(&current)
            .add(&self.biases[i]).map(self.activation.function);

            // Return output of network
            self.data.push(current.clone());
        }
    }

    pub fn back_propagate(&mut self, inputs: Matrix, targetrs: Matrix) {
        let mut errors = targets.subtract(&inputs);

        // Determine which parts of the network are responsible for errors by iterating over each layer
        // Need to multiply activation function by its derivative to access weights and biases
        // Then, multiply neurons by our error to update them proportionately.
        // (The higher the gradient, the more responsible that neuron is for the incorrect output)
        for i in (0..self.layers.len() - 1).rev() {
            gradients = gradients.elementwise_multiply(&errors).map(|x| x * 0.5);

            self.weights[i] = self.weights[i].add(&gradients.dot_multiply(&self.data[i].transpose()));
            self.biases[i] = self.biases[i].add(&gradients);

            errors = self.weights[i].transpose().dot_multiply(&errors);
            gradients = self.data[i].map(self.activation.derivative);
        }
    }

    pub fn train(&mut self, inputs: Vec<Vec<f64>>, targets: Vec<Vec<f64>>, epochs: u32) {
        for i in 1..=epochs {
            if epochs < 100 || i % (epochs / 100) == 0 {    
                println!("Epoch {} of {}", i, epochs);
            }
            for j in 0..inputs.len() {
                let outputs = self.feed_forward(Matrix::from(inputs[j].clone()));
                self.back_propagate(outputs, Matrix::from(targets[j].clone()));
            }
        }
    }
}

pub fn main() {
    // XOR POC
    let inputs = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ];

    let targets = vec![vec![0.0], vec![1.0], vec![1.0], vec![0.0]];

    let mut nnetwork = NNetwork::new(vec![2, 3, 1], SIGMOID, 0.5);

    nnetwork.train(inputs, targets, 10000);

    println!("{:?}", nnetwork.feed_forward(Matrix::from(vec![0.0, 0.0])));
	println!("{:?}", nnetwork.feed_forward(Matrix::from(vec![0.0, 1.0])));
	println!("{:?}", nnetwork.feed_forward(Matrix::from(vec![1.0, 0.0])));
	println!("{:?}", nnetwork.feed_forward(Matrix::from(vec![1.0, 1.0])));
}