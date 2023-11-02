use pcap::{Device, Capture};
use std::{io, env};
use std::collections::HashMap;
use std::net::IpAddr;


// ------------------------
/// Lists available network interfaces
/// 
/// # Arguments
/// None
/// 
/// # Examples
/// ```
/// interface_list();
/// ```
/// # Return 
/// N/A
/// 
fn interface_list() {
    // Grab network interfaces
    let devices = pcap::Device::list().expect("Failed to grab network interfaces");
    
    // Print network interfaces
    //let mut numbering = 1;
    let mut interfaces_vec: Vec<String> = Vec::new();
    
    println!("\n-=-=-=-=-=-=-=-=-=-=-=-");
    println!("    Interface List");
    println!("-=-=-=-=-=-=-=-=-=-=-=-");

    for device in devices{
        interfaces_vec.push(device.name.clone());

        println!("{}", device.name.clone());
        //println!("{}. {}", &mut numbering, device.name.clone());
        //numbering += 1;
    }
    println!("-=-=-=-=-=-=-=-=-=-=-=-\n");
}


// ------------------------
/// Prompts user for the desired network interface, stores that info
/// 
/// # Arguments
/// None
/// 
/// # Examples
/// ```
/// choose_int();
/// ```
/// # Return 
/// The desired network interface as 'String' 
/// 
fn choose_int() -> String {
    interface_list();
    // Get the choice of interface
    let mut int_choice = String::new();
    println!("Interface you would like to capture on:");
    io::stdin().read_line(&mut int_choice).expect("Error, no valid interface selected");

    return int_choice;
}


// ------------------------
/// Starts the packet capture
/// 
/// # Arguments
/// * None
/// 
/// # Examples
/// ```
/// capture();
/// ```
/// 
/// # Returns
/// * N/A
/// 
fn capture() {
    let int_choice = choose_int();

    // Create HashMap and packet number variable
    let mut packetmap: HashMap<u32, Packet> = HashMap::new();
    let mut packet_number = 0u32;

    // Create Capture
    let mut cap = pcap::Capture::from_device(int_choice.as_str().trim_end())
        .expect("Error getting device")
        .promisc(true)        // Passes packets from the interface to the CPU
        .snaplen(128)        // Set the capture length per packet (Would be cool to set this differently depending on protocol)
        .open()     // Activate the capture
        .expect("Error starting capture");    // Catches any error from .open()

    //Put data into hashmap
    while let Ok(packet) = cap.next_packet() {
        packet_number += 1;
        println!("Received packet #{}: {:?}", packet_number, packet);

    }
    

}


// ------------------------
/// A packet of network data
/// 
/// The "Packet" struct represents the critical data within a single packet of network traffic
/// 
/// # Fields
/// 
/// * number - Packet number in the capture
/// * time - Time from the start of the capture
/// * source_ip - Source IP address
/// * source_port - Source port
/// * destination_ip - Destintation IP
/// * destination_port - Destination port
/// * protocol - The highest level protocol used for the packet
/// * length - Size of the packet, in bytes
/// * info - Summary of the fields of the highest layer protocol
/// 
pub struct Packet {
    pub number: u32,
    pub time: String,
    pub source_ip: IpAddr,
    pub source_port: u32,
    pub destination_ip: IpAddr,
    pub destination_port: u32,
    pub protocol: String,
    pub length: u32,
    pub info: String
}

/// Constructor for 'Packet' struct
impl Packet {
    pub fn new(
        number: u32, 
        time: String, 
        source_ip: IpAddr, 
        source_port: u32, 
        destination_ip: IpAddr, 
        destination_port: u32,
        protocol: String, 
        length: u32, 
        info: String
        ) -> Self {
            Packet {
                number, 
                time, 
                source_ip, 
                source_port, 
                destination_ip, 
                destination_port, 
                protocol, 
                length, 
                info,
            }
        }

        // Getters and Setters, used in object-oriented programming. Rust uses 'pub' to simplify this process.
        // Still may use these for data validation
        //
        //// Getter format, one for each field
        //pub fn number(&self) -> &u32 {
        //    &self.number
        //}
        //pub fn time(&self) -> &u32 {
        //    &self.time
        //}
        //pub fn source_ip(&self) -> IpAddr {
        //    self.source_ip
        //}
        //pub fn source_port(&self) -> u32 {
        //    self.source_port
        //}
        //pub fn destination_ip(&self) -> IpAddr {
        //    self.destination_ip
        //}
        //pub fn destination_port(&self) -> u32 {
        //    self.destination_port
        //}
        //pub fn protocol(&self) -> &str {
        //    &self.protocol
        //}
        //pub fn length(&self) -> u32 {
        //    self.length
        //}
        //pub fn info(&self) -> &str {
        //    &self.info
        //}
        //
        //// Setter format, one is needed for each field
        //pub fn set_number(&mut self, number: u32) {
        //    self.number = number;
        //}
        //pub fn set_time(&mut self, time: u32) {
        //    self.time = time;
        //}
        //pub fn set_source_ip(&mut self, source_ip: IpAddr) {
        //    self.source_ip = source_ip;
        //}
        //pub fn set_source_port(&mut self, source_port: u32) {
        //    self.source_port = source_port;
        //}
        //pub fn set_destination_ip(&mut self, destination_ip: IpAddr) {
        //    self.destination_ip = destination_ip;
        //}
        //pub fn set_destination_port(&mut self, destination_port: u32) {
        //    self.destination_port = destination_port;
        //}
        //pub fn set_protocol(&mut self, protocol: String) {
        //    self.protocol = protocol;
        //}
        //pub fn set_length(&mut self, length: u32) {
        //    self.length = length;
        //}
        //pub fn set_info(&mut self, info: String) {
        //    self.info = info;
        //}


    
}

fn main() {
    capture();
}




