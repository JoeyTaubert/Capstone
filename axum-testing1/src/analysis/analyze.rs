use futures_util::TryStreamExt;
use mongodb::{bson::{doc, Document}, Client, Collection};

/// MongoDB insertion function
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

// COMPUTATION FUNCTIONS 

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
