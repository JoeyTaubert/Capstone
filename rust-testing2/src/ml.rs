// ML - RNN
/// Reference: codemoon on YouTube, https://github.com/codemoonsxyz/neural-net-rs
use rand::Rng;
use std::f64::consts::E;

#[derive(Debug, Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl Matrix {
    pub fn randomm(rows: usize, cols: usize) -> Matrix {
        let mut rbuffer = Vec::<f64>::with_capacity(rows * cols);

        // Generate a random number for each value that will be in the Matrix
        for _ in 0..rows * cols {
            let num = rand::thread_rng().gen_range(0.0..1.0);

            rbuffer.push(num);
        }

        // Build the matrix and return it
        Matrix {
            rows,
            cols,
            data: rbuffer,
        }
    }

    pub fn addm(&self, other: &Matrix) -> Matrix {
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
            data: abuffer,
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
            data: sbuffer,
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

    pub fn elementwise_multiply(&self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to multiply by Matrix of incorrect dimensions") //// REPLACE FOR PROD
        }

        let mut result_data = vec![0.0; self.cols * self.rows];
        for (i, &item) in self.data.iter().enumerate() {
            //codemoon left a note to double check this, I believe self.data.len() may need to be reduced by 1, if anything
            result_data[i] = item * other.data[i]
        }

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: result_data,
        }
    }

    // Not implemented anywhere so commented them out
    //pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Matrix {
    //    assert!(data.len() - 1 != rows * cols, "Invalid Size");
    //    Matrix { rows, cols, data }
    //}
    //
    //pub fn zeros(rows: usize, cols: usize) -> Matrix {
    //    Matrix {
    //        rows,
    //        cols,
    //        data: vec![0.0; cols * rows],
    //    }
    //}

    pub fn transpose(&self) -> Matrix {
        let mut tbuffer = vec![0.0; self.cols * self.rows];

        for i in 0..self.rows {
            for j in 0..self.cols {
                tbuffer[j * self.rows + i] = self.data[i * self.cols + j];
            }
        }

        Matrix {
            rows: self.cols,
            cols: self.rows,
            data: tbuffer,
        }
    }

    pub fn map(&mut self, func: fn(&f64) -> f64) -> Matrix {
        let mut result = Matrix {
            rows: self.rows,
            cols: self.cols,
            data: Vec::with_capacity(self.data.len()),
        };

        result.data.extend(self.data.iter().map(|&val| func(&val))); //iterates over each element of the matrix and applies the function

        result
    }
}

/// Other Implementations for the Matrix type
impl From<Vec<f64>> for Matrix {
    fn from(vec: Vec<f64>) -> Self {
        let rows = vec.len();
        let cols = 1;
        Matrix {
            rows,
            cols,
            data: vec,
        }
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows && self.cols == other.cols && self.data == other.data
    }
}

/// Not sure if this is even needed as I am not performing the "tests"
//impl fmt::Display for Matrix {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        for row in 0..self.rows {
//            for col in 0..self.cols {
//                write!(f, "{}", self.data[row * self.cols + col])?;
//                if col < self.cols - 1 {
//                    write!(f, "\t")?; // Separate columns with a tab
//                }
//            }
//            writeln!(f)?; // Move to the next line after each row
//        }
//        Ok(())
//}

/// Struct for Activation functions, such as sigmoid, ReLU, GELU, etc.
#[derive(Clone, Copy, Debug)]
pub struct Activation {
    pub function: fn(&f64) -> f64,
    pub derivative: fn(&f64) -> f64,
}

/// Constant for sigmoid activation function
pub const SIGMOID: Activation = Activation {
    function: |x| 1.0 / (1.0 + E.powf(-x)),
    derivative: |x| x * (1.0 - x), // Derivative is used to access weights in the back propagation process
};

pub struct NNetwork {
    layers: Vec<usize>, // amount of neurons per layer ex: [2, 4, 1]
    weights: Vec<Matrix>,
    biases: Vec<Matrix>,
    data: Vec<Matrix>,
    activation: Activation,
    learning_rate: f64,
}

impl NNetwork {
    pub fn new(layers: Vec<usize>, activation: Activation, learning_rate: f64) -> Self {
        // Initialize vectors to hold weights and biases
        let mut weights = vec![];
        let mut biases = vec![];

        // Iterate over layers of the neural network
        for i in 00..layers.len() - 1 {
            weights.push(Matrix::randomm(layers[i + 1], layers[i]));
            biases.push(Matrix::randomm(layers[i + 1], 1));
        }

        // Initialize the network
        NNetwork {
            layers,
            weights,
            biases,
            data: vec![],
            activation,
            learning_rate,
        }
    }

    pub fn feed_forward(&mut self, inputs: Matrix) -> Matrix {
        assert!(
            self.layers[0] == inputs.data.len(),
            "Invalid number of inputs to feed forward"
        );

        // Holds values for next layer of neural network
        let mut current = inputs;

        self.data = vec![current.clone()];

        // Iterate over all layers
        for i in 0..self.layers.len() - 1 {
            // Apply feed forward algorithm
            current = self.weights[i]
                .dot_multiply(&current)
                .addm(&self.biases[i])
                .map(self.activation.function);

            // Return output of network
            self.data.push(current.clone());
        }
        current
    }

    pub fn back_propagate(&mut self, inputs: Matrix, targets: Matrix) {
        let mut gradients = inputs.clone().map(self.activation.derivative);
        let mut errors = targets.subtractm(&inputs);

        for i in (0..self.layers.len() - 1).rev() {
            // Calculate the gradients, and scale them by the learning rate here instead of inside `map`.
            gradients = gradients.elementwise_multiply(&errors);
            // Apply the learning rate to the entire matrix of gradients
            gradients
                .data
                .iter_mut()
                .for_each(|g| *g *= self.learning_rate);

            // Update weights and biases
            self.weights[i] =
                self.weights[i].addm(&gradients.dot_multiply(&self.data[i].transpose()));
            self.biases[i] = self.biases[i].addm(&gradients);

            // Propagate the error backwards
            errors = self.weights[i].transpose().dot_multiply(&errors);
            // Recalculate gradients for the next layer
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
    // Packet Size PoC

    // inputs
    let inputs = vec![
        vec![10.0, 10.0, 10.0, 10.0], //inputs for dataset 2
        vec![10.0, 11.0, 10.0, 11.0], //inputs for dataset 1
        vec![11.0, 10.0, 11.0, 10.0], //inputs for dataset 1
        vec![11.0, 11.0, 11.0, 11.0], //inputs for dataset 2
    ];

    // targets

    let targets = vec![
        vec![0.0], // targets for dataset 1 // 0 bytes
        vec![0.5], // targets for dataset 2
        vec![0.5], // targets for dataset 1
        vec![1.0], // targets for dataset 2 // 65535 bytes
    ];

    // train

    let mut nnetwork = NNetwork::new(vec![4, 2, 1], SIGMOID, 0.5);

    nnetwork.train(inputs, targets, 10000);

    // use the neural network using real data to get a prediction value

    let output = nnetwork
        .feed_forward(Matrix::from(vec![10.0, 11.0, 10.0, 11.0]))
        .data;

    println!("Prediction: {:?}", output);

    //// Echo Request/Reply PoC
    //let inputs = vec![
    //    vec![8.0, 0.0], //x0023 has the code for echo rquest/reply. Request is 08 and replies are 00
    //    vec![0.0, 8.0], // Could extend these into a list to include sequence numbers as well
    //];
    //
    //// target values
    //let targets = vec![vec![8.0], vec![0.0]];
    //
    //// train network
    //let mut nnetwork = NNetwork::new(vec![2, 3, 1], SIGMOID, 0.5);
    //
    //nnetwork.train(inputs, targets, 10000);
    //
    //// test the neural network with the following inputs
    //println!("\n0 = Echo REPLY");
    //println!("1 = Echo REQUEST");
    //println!(
    //    "What comes after echo reply? {:?}",
    //    nnetwork.feed_forward(Matrix::from(vec![8.0, 0.0])).data
    //);
    //println!(
    //    "What comes after echo request? {:?}",
    //    nnetwork.feed_forward(Matrix::from(vec![0.0, 8.0])).data
    //);

    //// XOR PoC
    //let inputs = vec![
    //    vec![0.0, 0.0],
    //    vec![0.0, 1.0],
    //    vec![1.0, 0.0],
    //    vec![1.0, 1.0],
    //];
    //
    //// target values
    //let targets = vec![vec![0.0], vec![1.0], vec![1.0], vec![0.0]];
    //
    //// train network
    //let mut nnetwork = NNetwork::new(vec![2, 3, 1], SIGMOID, 0.5);
    //
    //nnetwork.train(inputs, targets, 10000);
    //
    //// test the neural network
    //println!("{:?}", nnetwork.feed_forward(Matrix::from(vec![0.0, 0.0])));
    //println!("{:?}", nnetwork.feed_forward(Matrix::from(vec![0.0, 1.0])));
    //println!("{:?}", nnetwork.feed_forward(Matrix::from(vec![1.0, 0.0])));
    //println!("{:?}", nnetwork.feed_forward(Matrix::from(vec![1.0, 1.0])));
}
