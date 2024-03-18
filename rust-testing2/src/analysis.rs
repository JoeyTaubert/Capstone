use pnet::{packet::{ipv4::Ipv4Packet, ipv6::Ipv6Packet, tcp::TcpPacket, udp::UdpPacket, Packet, ethernet::EthernetPacket, ip::IpNextHeaderProtocols, arp::ArpPacket}, util::MacAddr};
use pnet::datalink::{self, NetworkInterface, Channel};
use rand::Error;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use chrono::{Utc, DateTime};
use mongodb::{bson::{doc, Bson, Document}, Client, Collection, {options::IndexOptions, IndexModel}};
use tokio;

///
///
///
///
pub async fn create_index() -> Result<(), String> {
    // Define variables needed to interact with MongoDB
    let client = Client::with_uri_str("mongodb://127.0.0.1:27017").await
        .map_err(|e| format!("[-]ERROR: Failed to connect to MongoDB: {}", e))?;
    
    let database = client.database("captures");
    let table: Collection<Document> = database.collection("packets");

    // Create the index model using the builder
    let index_model = IndexModel::builder()
        .keys(doc! { "timestamp": 1 }) // 1 for ascending order, -1 for descending
        .options(Some(IndexOptions::builder().build()))
        .build();

    // Create the index by passing the model previously built
    table.create_index(index_model, None).await
        .map_err(|e| format!("[-]ERROR: Failed to create index in MongoDB: {}", e))?;

    Ok(()) // Returns Ok if successful
}

///
///
///
///
pub fn get_timestamps() -> (String, String) {
    
    
}

///
///
///
///
pub async fn compute_total_size(start_timestamp: String, end_timestamp: String) -> Result<(), String>{
    // Define variables needed to interact with MongoDB
    let client = Client::with_uri_str("mongodb://127.0.0.1:27017").await
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
            "$gte": start_timestamp,
            "$lte": end_timestamp
        }
    };

    // Query the database for all documents within the timestamp
    let mut cursor = table.find(query, None).await
        .map_err(|e| format!("[-]ERROR: Failed to query database: {}", e))?;



    Ok(())
}


#[tokio::main]
pub async fn main() {
    // May need to add a step here where we reorganize the data in MongoDB based on timestamp. 
    // OR, when we query, just sort based on timestamp
    
    // Create the index for timestamp, this is a no-op for MongoDB if it is already existing, which uses very minimal resources
    match create_index().await {
        Ok(_) => println!("[+]INFO: Index created or already exists"),
        Err(e) => println!("[-]ERROR: Failed to create index: {}", e),
    }

    let (start_timestamp, end_timestamp) = get_timestamps();

    // Compute Key Metrics

    // Total Size of Packets in x timeframe
    match compute_total_size(start_timestamp, end_timestamp).await {
        Ok(_) => println!("[+]INFO: Successfully queried database"),
        Err(e) => println!("[-]ERROR: Failed to query database: {}", e),
    }

}
