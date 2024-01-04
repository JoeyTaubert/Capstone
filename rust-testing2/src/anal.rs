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

///
/// 
pub fn parsing(file_handle: File) -> Vec<String> {
    let mut cap_data: Vec<String> = Vec::new();
    let breader = BufReader::new(file_handle);

    for line in breader.lines() {
        match line {
            Ok(l) => cap_data.push(l),
            Err(e) => eprintln!("[-]ERROR: Failed to read a line from file: {}", e),
        }
    }

    return cap_data
}

pub fn main() {

    let mut data: Vec<String> = Vec::new(); 

    match open_file() {
        Ok(fhandle) => data = parsing(fhandle),
        Err(e) => {
            eprintln!("[-]ERROR: Failed to open file; {}", e);
            return 
        },
    };

    println!("{:?}", data)
}
