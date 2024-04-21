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
        // Initialize our data variable
        let mut rbuffer = Vec::<f64>::with_capacity(rows * cols);

        // Generate a random number for each value that will be in the Matrix
        for _ in 0..rows * cols {
            let num = rand::thread_rng().gen_range(0.0..1.0);

            // Push each randomized number to the list
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
        // Check if we can add these Matrices
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to add matrix of incorrect dimensions") //// REPLACE THIS FOR PRODUCTION
        }

        // Initialize data buffer for resulting Matrix
        let mut abuffer = Vec::<f64>::with_capacity(self.rows * self.cols);

        // Do the addition
        for i in 0..self.data.len() {
            let result = self.data[i] + other.data[i];

            abuffer.push(result)
        }

        // Build the matrix and return it
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

        // Iterate over the rows of matrix A
        for i in 0..self.rows {
            // Iterate over the columns of matrix B
            for j in 0..other.cols {
                let mut sum = 0.0; // Initialize our sum float and reset each iteration
                                   // Sum the elements of the current row of Matrix A and the corresponding column of Matrix B
                for k in 0..self.cols {
                    sum += self.data[i * self.cols + k] * other.data[k * other.cols + j];
                }
                // Store the calculated dot product in its respective position in the resulting matrix
                result_data[i * other.cols + j] = sum;
            }
        }

        // Build and return the resulting matrix
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

        // Initialize resulting vector to hold data
        let mut result_data = vec![0.0; self.cols * self.rows];

        // Multiply each element of matrix A against its corresponding element of matrix B
        for (i, &item) in self.data.iter().enumerate() {
            result_data[i] = item * other.data[i]
        }

        // Build and return the resulting matrix
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
        // Initialize a matrix of 0's, dimensions based on self
        let mut tbuffer = vec![0.0; self.cols * self.rows];

        // Iterate over the rows
        for i in 0..self.rows {
            // Iterate over the columns
            for j in 0..self.cols {
                // Perform the transposition
                tbuffer[j * self.rows + i] = self.data[i * self.cols + j];
            }
        }

        // Build the new matrix to be returned
        Matrix {
            rows: self.cols,
            cols: self.rows,
            data: tbuffer,
        }
    }

    pub fn map(&mut self, func: fn(&f64) -> f64) -> Matrix {
        // Initialize resulting matrix
        let mut result = Matrix {
            rows: self.rows,
            cols: self.cols,
            data: Vec::with_capacity(self.data.len()),
        };

        // Iterate over each element of the matrix and apply the provided function
        result.data.extend(self.data.iter().map(|&val| func(&val)));

        // Returns the matrix
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
            // Initialize all weights with a random value
            weights.push(Matrix::randomm(layers[i + 1], layers[i]));
            // Initialize all biases with a random value
            biases.push(Matrix::randomm(layers[i + 1], 1));
        }

        // Return the network
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
        //Check to see if we have enough neurons in the first layer to accept all inputs
        assert!(
            self.layers[0] == inputs.data.len(),
            "Invalid number of inputs to feed forward"
        );

        // Grab a mutable version of inputs
        let mut current = inputs;

        // Pass the inputs to the data field of our neural network
        self.data = vec![current.clone()];

        // Iterate over the layers of the network
        for i in 0..self.layers.len() - 1 {
            // Apply feed forward algorithm and reassign output to current
            current = self.weights[i] // Access the weights of the current layer
                // Multiply the inputs/datapoints by the weights
                .dot_multiply(&current)
                // Add the biases
                .addm(&self.biases[i])
                // Use the activation function and return floats as outputs
                .map(self.activation.function);

            // Return output of network
            self.data.push(current.clone());
        }
        // return the outputs, this will be a vector of f64's
        // If the last layer has one neuron, one f64 value will be in the vector
        current
    }

    pub fn back_propagate(&mut self, inputs: Matrix, targets: Matrix) {
        // Initialize the gradient matrix with the model's outputs, and undo the feed forward process
        let mut gradients = inputs.clone().map(self.activation.derivative);
        // Initialize the errors matrix by finding the difference between the outputs and the target values
        let mut errors = targets.subtractm(&inputs);

        // Iterate over the layers of the network in reverse
        for i in (0..self.layers.len() - 1).rev() {
            // Calculate the error gradient
            gradients = gradients.elementwise_multiply(&errors);

            // Apply the learning rate to the gradients matrix
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
        // Iterate over each epoch
        for i in 1..=epochs {
            if epochs < 100 || i % (epochs / 100) == 0 {
                println!("Epoch {} of {}", i, epochs);
            }
            // Iterate over each input vector
            for j in 0..inputs.len() {
                // Feed forward with current input vector
                let outputs = self.feed_forward(Matrix::from(inputs[j].clone()));
                // Tune weights and biases based on the current target
                self.back_propagate(outputs, Matrix::from(targets[j].clone()));
            }
        }
    }
}

pub fn main() {
    // DNS Exfil Detection PoC

    // inputs (training on good DNS traffic, bad DNS traffic)
    let inputs = vec![
        //   length, entropy of domain, TCP or UDP
        vec![0.01, 0.01, 0.0], // small length, low entropy, UDP
        vec![0.01, 0.5, 0.0],
        vec![0.01, 0.6, 0.0],
        vec![0.01, 1.0, 0.0], // small length, high entropy, UDP
        vec![0.5, 0.01, 0.0], // med legnth, low entropy, UDP
        vec![0.5, 0.5, 0.0],
        vec![0.5, 1.0, 0.0], // med legnth, high entropy, UDP
        vec![0.7, 0.01, 0.0],
        vec![0.7, 1.0, 0.0],
        vec![1.0, 0.01, 0.0],  // high length, low entopy, UDP
        vec![1.0, 1.0, 0.0],   // high length, high entropy, UDP
        vec![0.01, 0.01, 1.0], // small length, low entropy, TCP
        vec![0.01, 0.5, 1.0],  // small length, med entropy, TCP
        vec![0.01, 1.0, 1.0],  // small length, high entropy, TCP
        vec![1.0, 0.01, 1.0],  // high length, low entropy, TCP
        vec![1.0, 1.0, 1.0],   // high length, high entropy, TCP
    ];

    // targets (0 for good traffic, 1 for bad traffic)
    let targets = vec![
        vec![0.0],
        vec![0.10],
        vec![0.25],
        vec![1.0],
        vec![0.1],
        vec![0.25],
        vec![1.0],
        vec![0.5],
        vec![1.0],
        vec![1.0],
        vec![1.0],
        vec![0.5],
        vec![1.0],
        vec![1.0],
        vec![1.0],
        vec![1.0],
    ];

    // train
    let mut nnetwork = NNetwork::new(vec![3, 6, 3, 1], SIGMOID, 1.0);
    nnetwork.train(inputs, targets, 10000);

    // feed forward and display results
    println!(
        "Suspicion rating: {:?}",
        nnetwork
            .feed_forward(Matrix::from(vec![0.586, 0.10, 0.0]))
            .data
    );

    /*
    // Packet Size PoC

    // inputs
    // these are calculated size of packets (five 10 second intervals)
    let inputs = vec![
        vec![5124.0, 5487.0, 4806.0, 4968.0, 5082.0],
        vec![4968.0, 3672.0, 5070.0, 4968.0, 3312.0],
        vec![4968.0, 6058.0, 5180.0, 5290.0, 3312.0],
        vec![4968.0, 3612.0, 4968.0, 4968.0, 3312.0],
        vec![4968.0, 3714.0, 4968.0, 4968.0, 11808.0],
    ];

    // Define scale
    //let max_val = 11808.0;
    //let min_val = 3312.0;
    let max_val = 12000.0;
    let min_val = 0.0;

    // Scale inputs
    let mut scaled_inputs = vec![];

    for i in &inputs {
        let mut temp_vec = vec![];
        for j in i {
            let scaled_j = (j - min_val) / (max_val - min_val);
            temp_vec.push(scaled_j);
        }
        scaled_inputs.push(temp_vec);
    }

    // targets
    // This is the target value of the dataset (the sixth 10 second interval)
    let targets = vec![
        vec![3938.0],
        vec![4968.0],
        vec![4968.0],
        vec![4968.0],
        vec![6384.0],
    ];

    // scale targets
    let mut scaled_targets = vec![];

    for i in &targets {
        let mut temp_vec2 = vec![];
        for j in i {
            let scaled_j = (j - min_val) / (max_val - min_val);
            temp_vec2.push(scaled_j);
        }
        scaled_targets.push(temp_vec2);
    }

    // initialize the RNN
    let mut nnetwork = NNetwork::new(vec![5, 10, 10, 10, 5, 2, 1], SIGMOID, 1.0);

    // train
    nnetwork.train(scaled_inputs, scaled_targets, 10000);

    // feed forward with real inputs (five 10 second intervals)
    // output is predicting the sixth
    let ff_inputs = vec![4968.0, 3714.0, 4968.0, 4968.0, 3938.0];

    // scale inputs
    let mut ff_inputs_scaled = vec![];

    for i in &ff_inputs {
        let scaled_i = (i - min_val) / (max_val - min_val);
        ff_inputs_scaled.push(scaled_i);
    }

    let output = nnetwork.feed_forward(Matrix::from(ff_inputs_scaled)).data;

    // result
    println!("Prediction: {:?}", output);
    println!("Actual: 0.413");
    println!(
        "Prediction (Bytes): {}",
        min_val + output[0] * (max_val - min_val)
    );
    println!("Actual: 4968.0");
    */

    /*
    Numbers PoC

    // inputs
    let inputs = vec![
        vec![1.0, 1.0], //inputs for dataset 2
        vec![2.0, 2.0], //inputs for dataset 1
        vec![3.0, 3.0], //inputs for dataset 1
        vec![4.0, 4.0], //inputs for dataset 2
        vec![5.0, 5.0],
    ];

    //targets

    let targets = vec![
        vec![0.0],  // targets for dataset 1 // 0 bytes
        vec![0.25], // targets for dataset 2
        vec![0.5],  // targets for dataset 1
        vec![0.75], // targets for dataset 2 // 65535 bytes
        vec![1.0],
    ];

    // train

    let mut nnetwork = NNetwork::new(vec![2, 2, 1], SIGMOID, 0.5);

    nnetwork.train(inputs, targets, 10000);

    // use the neural network using real data to get a prediction value

    let output = nnetwork.feed_forward(Matrix::from(vec![3.0, 3.0])).data;

    println!("Prediction: {:?}", output);
    */

    /*
    // XOR PoC
    let inputs = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ];

    // target values
    let targets = vec![vec![0.0], vec![1.0], vec![1.0], vec![0.0]];

    // train network
    let mut nnetwork = NNetwork::new(vec![2, 3, 1], SIGMOID, 0.5);

    nnetwork.train(inputs, targets, 10000);

    // test the neural network
    println!(
        "0 XOR 0 = {:?}",
        nnetwork.feed_forward(Matrix::from(vec![0.0, 0.0])).data
    );
    println!(
        "0 XOR 1 = {:?}",
        nnetwork.feed_forward(Matrix::from(vec![0.0, 1.0])).data
    );
    println!(
        "1 XOR 0 = {:?}",
        nnetwork.feed_forward(Matrix::from(vec![1.0, 0.0])).data
    );
    println!(
        "1 XOR 1 = {:?}",
        nnetwork.feed_forward(Matrix::from(vec![1.0, 1.0])).data
    );
    */
}
