use mongodb::{bson::{doc, Document}, options::IndexOptions, Client, Collection, IndexModel};

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