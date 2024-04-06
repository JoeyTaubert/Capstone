use chrono::NaiveDateTime;
use futures_util::TryStreamExt;
use mongodb::{
    bson::{doc, Document},
    Client, Collection,
    {options::IndexOptions, IndexModel},
};
use std::io::{self};

// ------------------------
/// More versatile function for creating indexes
///
/// # Arguements
/// * field: String - A field in MongoDB
/// * ascent: i64 - Either 1 (ascending order) or -1, (descending order)
///
/// # Returns
/// * Result<(), String>
///     * () - Returns as Ok()
///     * String - Returns an Err enum with string value for handling
///
pub async fn create_index(field: String, ascend: i64) -> Result<(), String> {
    // Define variables needed to interact with MongoDB
    let client = Client::with_uri_str("mongodb://127.0.0.1:27017")
        .await
        .map_err(|e| format!("[-]ERROR: Failed to connect to MongoDB: {}", e))?;

    let database = client.database("captures");
    let table: Collection<Document> = database.collection("packets");

    // Check to make sure we have a valid value for ascend/descend sorting
    if ascend != 1 && ascend != -1 {
        let errormessage: String = String::from("[-]ERROR: ascend value not set to 1 or -1");
        return Err(errormessage);
    }

    // Check to make sure we have a valid value for field
    // may need to trim_end()
    if field != "_id"
        && field != "number"
        && field != "timestamp"
        && field != "protocol"
        && field != "source_mac"
        && field != "source_ip"
        && field != "source_port"
        && field != "dest_mac"
        && field != "dest_ip"
        && field != "dest_port"
        && field != "length"
        && field != "payload"
    {
        let errormessage: String = String::from("[-]ERROR: field value not set to a valid field");
        return Err(errormessage);
    }

    // Create the index model using the builder, pass in field and ascend parameters
    let index_model = IndexModel::builder()
        .keys(doc! { field: ascend })
        .options(Some(IndexOptions::builder().build()))
        .build();

    // Create the index by passing the model previously built
    table
        .create_index(index_model, None)
        .await
        .map_err(|e| format!("[-]ERROR: Failed to create index in MongoDB: {}", e))?;

    Ok(()) // Returns Ok if successful
}

// ------------------------
/// Creates an index on the timestamp field
///
///
///
pub async fn create_timestamp_index() -> Result<(), String> {
    // Define variables needed to interact with MongoDB
    let client = Client::with_uri_str("mongodb://127.0.0.1:27017")
        .await
        .map_err(|e| format!("[-]ERROR: Failed to connect to MongoDB: {}", e))?;

    let database = client.database("captures");
    let table: Collection<Document> = database.collection("packets");

    // Create the index model using the builder
    let index_model = IndexModel::builder()
        .keys(doc! { "timestamp": 1 })
        .options(Some(IndexOptions::builder().build()))
        .build();

    // Create the index by passing the model previously built
    table
        .create_index(index_model, None)
        .await
        .map_err(|e| format!("[-]ERROR: Failed to create index in MongoDB: {}", e))?;

    Ok(()) // Returns Ok if successful
}

// ------------------------
/// Grab start/end timestamps
///
/// Timestamps are in format:
///
/// YYYY-MM-DD HH:MM:SS.SSSSSSSSS
///
/// Ex. 2024-03-18 11:00:00.000000000
///
/// # Arguements
/// * None
///
/// # Returns
/// * (String, String)
///     * start_timestamp: String
///     * end_timestamp: String
///
pub fn get_timestamps() -> (String, String) {
    // Initialize variables
    let mut start_timestamp = String::new();
    let mut end_timestamp = String::new();

    // Prompt for start timestamp
    println!("Start timestamp: (YYYY-MM-DD HH:MM:SS.SSSSSSSSS): ");
    io::stdin()
        .read_line(&mut start_timestamp)
        .expect("[-]ERROR: Failed to read line");

    // Check if a valid timestamp was supplied
    match NaiveDateTime::parse_from_str(start_timestamp.trim(), "%Y-%m-%d %H:%M:%S%.f") {
        Ok(_) => {
            let mut start_timestamp_trimmed = start_timestamp.trim_end().to_string();
            start_timestamp_trimmed.push_str(" UTC");
        }
        Err(e) => {
            println!("[-]ERROR: Incorrect start timestamp format: {}", e);
            start_timestamp = String::from("");
            end_timestamp = String::from("");
            return (start_timestamp, end_timestamp);
        }
    }

    // Prompt for end timestamp
    println!("End timestamp: (YYYY-MM-DD HH:MM:SS.SSSSSSSSS): ");
    io::stdin()
        .read_line(&mut end_timestamp)
        .expect("[-]ERROR: Failed to read line");

    // Check if a valid timestamp was supplied
    match NaiveDateTime::parse_from_str(end_timestamp.trim(), "%Y-%m-%d %H:%M:%S%.f") {
        Ok(_) => {
            let mut end_timestamp_trimmed = end_timestamp.trim_end().to_string();
            end_timestamp_trimmed.push_str(" UTC");
        }
        Err(e) => {
            println!("[-]ERROR: Incorrect end timestamp format: {}", e);
            start_timestamp = String::from("");
            end_timestamp = String::from("");
            return (start_timestamp, end_timestamp);
        }
    }

    (start_timestamp, end_timestamp)
}

// ------------------------
///
/// # Arguements
/// * start_timestamp: &String - Start timestamp
/// * end_timestamp: &String - End timestamp
///
/// # Returns
/// * Result<i64, String>
///     * total_size: i64 - Total size, in bytes of all packets matching query
///     * Error: String
///
pub async fn compute_total_size(
    start_timestamp: &String,
    end_timestamp: &String,
) -> Result<i64, String> {
    // Define variables needed to interact with MongoDB
    let client = Client::with_uri_str("mongodb://127.0.0.1:27017")
        .await
        .map_err(|e| format!("[-]ERROR: Failed to connect to MongoDB: {}", e))?;
    let database = client.database("captures");
    let table: Collection<Document> = database.collection("packets");

    // Define start and end timestamp
    // I want this to be handled by a nice GUI interface
    //let start_timestamp = DateTime::from_millis();
    //let end_timestamp = DateTime::from_millis();

    // Build the query filter
    let query = doc! {
            "timestamp": {
                "$gte": &start_timestamp,
                "$lte": &end_timestamp
        }
    };

    // Query the database for all documents within the timestamp
    let mut cursor = table
        .find(query, None)
        .await
        .map_err(|e| format!("[-]ERROR: Failed to query database: {}", e))?;

    let mut total_size: i64 = 0;

    // Iterate over the returned results, add each "length" field to total_size
    while let Some(document) = cursor
        .try_next()
        .await
        .map_err(|e| format!("[-]ERROR: Failed to fetch document: {}", e))?
    {
        if let Ok(length_str) = document.get_str("length") {
            if let Ok(length) = length_str.parse::<i64>() {
                total_size += length;
            }
        }
    }

    let insert_table = String::from("size");

    match insert_result(insert_table, start_timestamp, end_timestamp, total_size).await {
        Ok(_) => Ok(total_size),
        Err(e) => Err(e),
    }
}

pub async fn insert_result(
    table: String,
    start_timestamp: &String,
    end_timestamp: &String,
    data: i64,
) -> Result<(), String> {
    let client = Client::with_uri_str("mongodb://127.0.0.1:27017")
        .await
        .map_err(|e| format!("[-]ERROR: Failed to connect to MongoDB: {}", e))?;
    let database = client.database("metrics");
    let collection: Collection<Document> = database.collection(&table);

    let new_doc = doc! {
        "start_timestamp": &start_timestamp,
        "end_timestamp": &end_timestamp,
        "size": &data,
    };

    collection
        .insert_one(new_doc, None)
        .await
        .expect("[-] ERROR: Failed to insert analysis data into MongoDB");

    Ok(())
}

// ------------------------
/// Main function for controlling analysis/computation of key metrics
///
/// # Arguements
/// * None
///
/// # Returns
/// * None (Key metrics are stored in DB)
///
#[tokio::main]
pub async fn main() {
    // May need to add a step here where we reorganize the data in MongoDB based on timestamp.
    // OR, when we query, just sort based on timestamp

    // Create the index for timestamp, this is a no-op for MongoDB if it is already existing, which uses very minimal resources
    match create_timestamp_index().await {
        Ok(_) => println!("[+]INFO: Index created or already exists"),
        Err(e) => println!("[-]ERROR: Failed to create index: {}", e),
    }

    // Initialize timestamps so we can send them to the function in a tuple
    let mut start_timestamp = String::from("");
    let mut end_timestamp = String::from("");

    // While the timestamps are "", get timestamps
    while start_timestamp.is_empty() || end_timestamp.is_empty() {
        println!("");
        (start_timestamp, end_timestamp) = get_timestamps();
    }

    // Compute Key Metrics

    // Total Size of Packets in x timeframe
    match compute_total_size(&start_timestamp, &end_timestamp).await {
        Ok(total_size) => println!("{} inserted into MongoDB", total_size),
        Err(e) => println!(
            "[-] ERROR: Failed to compute total size of data range: {}",
            e
        ),
    };
}
