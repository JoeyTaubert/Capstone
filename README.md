# run.bash
Used to compile the binary with necessary privileges to initiate a packet capture. 

# rust-testing2
Project folder that holds the main code that was worked on. 

## src/
### main.rs
Main program flow
### cap.rs
Packet capture program. Parses out data into fields and inserts into MongoDB.
### analysis.rs
Computes key metrics on packets based on provided timestamps. Pulls data from MongoDB via query, and inserts computed results into MongoDB.
### ml.rs
Main file containing the Feedforward Neural Network (FNN) code. Proof of concepts are located in the `main()` function.

## data/
Contians a text file describing the test dataset used for packet size proof of concept

## caps/
Old directory used to hold packet capture data before MongoDB was set up

# axum-testing1
Project folder for WebApp development.
## src/
### main.rs
Main WebApp program. 
### cap
Capture module imported from `rust-testing2`
### analysis
Analysis module imported from `rust-testing2`
## static/html
Contains html files and handlebars files for dynamic webpage rendering. 
