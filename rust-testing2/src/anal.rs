use pnet::{packet::{ipv4::Ipv4Packet, ipv6::Ipv6Packet, tcp::TcpPacket, udp::UdpPacket, Packet, ethernet::EthernetPacket, ip::IpNextHeaderProtocols, arp::ArpPacket}, util::MacAddr};
use pnet::datalink::{self, NetworkInterface, Channel};
use std::{io::{Write, self, BufRead, BufReader}, net::{IpAddr, Ipv4Addr, Ipv6Addr}, fs::{OpenOptions, File}};
use chrono::{Utc, DateTime};
mod cap;

pub fn open_file() -> io::Resutlt<()> {

    println!("What is the filename? (YYYY-MM-DD-HH-MM-SS-Capture.txt) ");
    let mut filename = std::io::stdin().read_line(&mut filename)
        .expect("[-]ERROR: Failed to read line.");
    let mut filepath = String::new();
    let dir = String::from("caps/");

    filepath = format!("{}{}", dir, filename);
    
    let cap_data = fs::read_to_string(filepath).expect("[-]ERROR: Could not read file to string.")?;

    println!("File contents:\n()", cap_data);

    Ok(())
}

pub fn main() {
    let data = open_file();
    data;
}
