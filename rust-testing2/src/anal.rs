use pnet::{packet::{ipv4::Ipv4Packet, ipv6::Ipv6Packet, tcp::TcpPacket, udp::UdpPacket, Packet, ethernet::EthernetPacket, ip::IpNextHeaderProtocols, arp::ArpPacket}, util::MacAddr};
use pnet::datalink::{self, NetworkInterface, Channel};
use std::{io::{Write, self, BufRead, BufReader}, net::{IpAddr, Ipv4Addr, Ipv6Addr}, fs::{OpenOptions, File}};
use chrono::{Utc, DateTime};

///
///
pub fn open_file() -> Result<File, io::Error> {

    //Get the filename from the user
    println!("\nWhat is the filename? (YYYY-MM-DD-HH-MM-SS-Capture.txt) ");

    let mut filename = String::new();
    std::io::stdin().read_line(&mut filename)
        .expect("[-]ERROR: Failed to read line.");

    let filename = filename.trim();

    // Build the path to the file
    let dir = "caps/";
    let filepath = format!("{}{}", dir, filename);
    
    // Obtain File Handle
    File::open(filepath) // Implicit return
}

/// Parse for data that will be used to compute metrics
/// 
/// Metrics:
/// Bandwidth Usage - Total Packet Size (bits) / Time (s/m/h)
/// Throughput - Total Successful Packet Size (bits) / Time (s/m/h)
/// TCP Retransmission Rate - It has the same SEQ and ACK values as the lost packet, but a different IP ID (ip.id) in the IP header.
/// Connection Establishment Time -  ACK Timestamp - SYN Timestamp 
/// Response Times?
/// Types of traffic and distribution of traffic - Port/Transport Protocol (53/udp)
/// IP Addresses that were communicating (resolve to FQDN?)
/// 
pub fn parsing(file_handle: File) -> Vec<String> {
    let mut cap_data: Vec<String> = Vec::new();
    let breader = BufReader::new(file_handle);

    /*for line in breader.lines() {
        match line {
            Ok(l) => cap_data.push(l + "\n"),
            Err(e) => eprintln!("[-]ERROR: Failed to read a line from file: {}", e),
        }*/

    return cap_data

}

pub fn choose() {
    println!("\n[+] Source data from a file (f) or timeframe (t): ");

    let mut dchoice = String::new();
    std::io::stdin().read_line(&mut dchoice).expect("[-]ERROR: Error reading user input.");

    
    if dchoice.trim().starts_with("f") {
        let mut data_from_file: Vec<String> = Vec::new(); 
        match open_file() {
            Ok(fhandle) => {
                data_from_file = parsing(fhandle)
            },
            Err(e) => {
                eprintln!("[-]ERROR: Failed to open file; {}", e);
                return 
            },
        };
    // Option for parsing via timeframe. This will take user input for start time/date and end time/date (UTC), and
    // open all files that occur within the timeframe, it will then parse through all packets with a timestamp 
    // within the range. For now I will just be using the file option for testing. 
    } else if dchoice.trim().starts_with("t") {
        println!("\nUTC Start Date (YYYY-MM-DD): ");
        let mut date1 = String::new(); 
        std::io::stdin().read_line(&mut date1).expect("[-]ERROR: Error reading user input ");

        println!("\nUTC Start Time (HH[00-24]:MM[00-59]:SS[00-59]:MS[000000000-999999999])");
        let mut time1 = String::new();
        std::io::stdin().read_line(&mut time1).expect("[-]ERROR: Error reading user input");

        println!("\nUTC End Date (YYYY-MM-DD): ");
        let mut date2 = String::new(); 
        std::io::stdin().read_line(&mut date2).expect("[-]ERROR: Error reading user input ");

        println!("\nUTC End Time (HH[00-24]:MM[00-59]:SS[00-59]:MS[000000000-999999999])");
        let mut time2 = String::new();
        std::io::stdin().read_line(&mut time2).expect("[-]ERROR: Error reading user input");

        // Code for recursing through files with appropriate timestamps. Likely, I will find the first 
        // applicable packet in the first file (this file name will not be in the range, but the next chronological file
        // would be in the range) and the last applicable packet in the last file (last file where the name is in the
        // range).  
        
    } else {
        eprintln!("[-]ERROR: Invalid input for data source")
    }
}

pub fn main() {
    choose();
}
