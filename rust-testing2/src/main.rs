use pnet::{packet::{ipv4::Ipv4Packet, ipv6::Ipv6Packet, tcp::TcpPacket, udp::UdpPacket, Packet, ethernet::EthernetPacket, ip::IpNextHeaderProtocols, icmp::IcmpPacket, icmpv6::Icmpv6Packet, arp::ArpPacket}, util::MacAddr};
use pnet::datalink::{self, NetworkInterface, Channel};
use std::{io, collections::HashMap, net::{IpAddr, Ipv4Addr, Ipv6Addr}};
use chrono::{Utc, Local, DateTime};

// ------------------------
/// Lists available network interfaces
/// 
/// # Arguments
/// None
/// 
/// # Return 
/// * interfaces_vec: Vec<String> - Network interface list in a vector.
/// 
fn interface_list() -> Vec<String> {
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

    interfaces_vec //Returns interfaces_vec
}


// ------------------------
/// Prompts user for the desired network interface, stores that info
/// 
/// # Arguments
/// None
/// 
/// # Return 
/// * int_choice: String - The user input for the network interface as 'String' 
/// 
fn choose_int() -> String {
    // Call interface_list()
    interface_list();
    // Get the choice of interface
    let mut int_choice = String::new();
    println!("Interface you would like to capture on:");
    // Record user input
    io::stdin().read_line(&mut int_choice).expect("Error, no valid interface selected");

    // Some type-casting
    let int_choice2: &str = int_choice.as_str().trim_end();
    let int_choice = int_choice2.to_string();

    return int_choice;
}


// ------------------------
/// Compares user input for interface against the list.
/// 
/// # Arguments
/// * None
/// 
/// # Returns
/// * int_safe: String - The system interface that should be captured on
/// 
fn interface_fn() -> String {
    // Calls choose_int() and interface_list()
    let mut interface_input: String = choose_int();
    let all_interfaces = interface_list();

    // Loop through all interfaces
    loop {
        // If the input matches a interface in the list, write it to int_safe and break
        if all_interfaces.contains(&interface_input) {
            break;
        // else, try again
        } else {
            println!("Interface '{}' not found, please try again.", &interface_input);
            interface_input = choose_int();
        }
    }
    return interface_input;
}


// ------------------------
/// Starts a network capture
/// 
/// # Arguments
/// * interface: String - 
/// 
/// # Returns
/// N/A
fn capture(interface: String, number: &mut i32) {
    println!("Capturing on {}...\n", interface);
    
    let interfaces = datalink::interfaces();
    let mut number = 0;
    // Get the network interface as &NetworkInterface type
    let interface_dl = interfaces.into_iter()
                                                .filter(|iface: &NetworkInterface| iface.name == interface)
                                                .next()
                                                .expect("Error getting interface");

    // Handling packets so that only packets with Ethernet frames are processed further
    match datalink::channel(&interface_dl, Default::default()) {
        Ok(Channel::Ethernet(_, mut rx)) => {
            loop {
                number += 1;
                // Calls the next ethernet frame
                match rx.next() {
                    Ok(packet) => {
                        // Store the etherenet frame in variable
                        let packet = EthernetPacket::new(packet).unwrap();
                        // Pass the packet data to the parse_packet()
                        parse_packet(&packet, &mut number);
                    },
                    // If there is an error accessing the next ethernet frame, print an error to the error log
                    Err(e) => {
                        eprintln!("[+]INFO: An error occured while reading {}", e);
                    }
                }
            }
        },
        // If a packet comes in with an unexpected header layout, print an error to the error log
        Ok(_) => eprintln!("[+]INFO: Unsupported channel type for packet, aka not Ethernet channel"),
        // Handles generic Err
        Err(e) => eprintln!("[+]INFO: An error occured while creating the datalink channel: {}", e),
        }
    }


// ------------------------
/// Parses each packet of the capture and grabs critical information
/// 
/// # Arguments
/// packet_data: &EthernetPacket - 
/// 
/// # Returns
/// N/A
fn parse_packet(packet_data: &EthernetPacket, number: &mut i32) {
    // Initialize all needed fields
    let source_mac: MacAddr = packet_data.get_source(); // We already have direct access to layer 2 info, so assign these variables
    let dest_mac: MacAddr = packet_data.get_destination();
    let mut source_ip: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let mut dest_ip: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let mut source_port: u16 = 0;
    let mut dest_port: u16 = 0;
    let mut protocol = String::new();
    let length = packet_data.packet().len();
    let timestamp = Utc::now();
    let ppayload: Vec<u8> = packet_data.payload().to_vec();

    // 'match' statement to differentiate between IPv4 header and IPv6
    match packet_data.get_ethertype() {
        pnet::packet::ethernet::EtherTypes::Ipv4 => {
            if let Some(header) = Ipv4Packet::new(packet_data.payload()) {
                // Grab source/destination IPv4s
                source_ip = IpAddr::V4(Ipv4Addr::from(header.get_source()));
                dest_ip = IpAddr::V4(Ipv4Addr::from(header.get_destination())); //create a function convert_ipv4_to_ip to convert Ipv4Addr to IpAddr type
                match header.get_next_level_protocol() {
                    // TCP
                    IpNextHeaderProtocols::Tcp => { // Can further inspect TCP traffic for Client Hello for TLS
                        if let Some(tcp) = TcpPacket::new(header.payload()) {
                            // Grab source/destination TCP ports
                            source_port = tcp.get_source();
                            dest_port = tcp.get_destination();
                            protocol = String::from("TCP"); 
                        } else {
                            //Do nothing
                        }
                    },
                    // UDP
                    IpNextHeaderProtocols::Udp => { // Can further inspect UDP traffic to see if it follows the same format as QUIC 
                        if let Some(udp) = UdpPacket::new(header.payload()) {
                            source_port = udp.get_source();
                            dest_port = udp.get_destination();
                            protocol = String::from("UDP"); 
                        } else {
                            //Do nothing
                        }
                    },
                    // ICMP (ICMPv4)
                    IpNextHeaderProtocols::Icmp => {
                            // ICMP is a layer 3 protocol and does not have a port to extract
                            protocol = String::from("ICMP"); 
                    },
                    // For any other 'match' condition, print an error to the error log
                    _ => {
                        eprintln!("[+]INFO: Unsupported next level protocol: {}", header.get_next_level_protocol());
                    }
                }
            } else {
                // Do nothing
            }
        },
        pnet::packet::ethernet::EtherTypes::Ipv6 => {
            if let Some(header) = Ipv6Packet::new(packet_data.payload()) {
                // Grab source/destination IPv6
                source_ip = IpAddr::V6(Ipv6Addr::from(header.get_source()));
                dest_ip = IpAddr::V6(Ipv6Addr::from(header.get_source()));
                match header.get_next_header(){
                    // TCP
                    IpNextHeaderProtocols::Tcp => { // Need to account for Udp, QUIC, TLS
                            if let Some(tcp) = TcpPacket::new(header.payload()) {
                            // Grab source/destination ports
                            source_port = tcp.get_source();
                            dest_port = tcp.get_destination();
                            } else {
                                //Do nothing
                            }
                    },
                    // UDP
                    IpNextHeaderProtocols::Udp => { // Can further inspect UDP traffic to see if it follows the same format as QUIC 
                        if let Some(udp) = UdpPacket::new(header.payload()) {
                            source_port = udp.get_source();
                            dest_port = udp.get_destination();
                            protocol = String::from("UDP"); 
                        } else {
                            //Do nothing
                        }
                    },
                    // ICMPv6
                    IpNextHeaderProtocols::Icmpv6 => {
                        protocol = String::from("ICMPv6"); 
                    },
                    _ => {
                        eprintln!("[+]INFO: Unsupported next level protocol: {}", header.get_next_header());
                    }
                }
            } else {
                //Do nothing
            }
        },
        pnet::packet::ethernet::EtherTypes::Arp => {
            if let Some(_arp) = ArpPacket::new(packet_data.payload()) {
                protocol = String::from("ARP");
            } else {
                //Do nothing
            }
        },
        _ => {
            eprintln!("[+]INFO: Unsupported ethertype: {:?}", packet_data.get_ethertype());
        }
    };
    println!("Number: {} | Time: {} | Protocol: {} | Source MAC: {} | Destination MAC: {} | Source IP: {} | Source Port: {} | Destination IP: {} | Destination Port: {} | Length: {} | Payload: {:?}\n", 
            &number, &timestamp, &protocol, &source_mac, &dest_mac, &source_ip, 
            &source_port, &dest_ip, &dest_port, &length, &ppayload);
}

// ------------------------
/// The "Packet" struct represents the critical data within a single packet of network traffic
/// 
/// # Fields
/// 
/// * number - Packet number in the capture
/// * time - Time from the start of the capture
/// * source_mac - Source MAC address
/// * source_ip - Source IP address
/// * source_port - Source port
/// * dest_mac - Destination MAC address
/// * dest_ip - Destintation IP address
/// * dest_port - Destination port
/// * protocol - The highest level protocol used for the packet
/// * length - Size of the packet, in bytes
/// * payload - Summary of the fields of the highest layer protocol
/// 
/// # impl's
/// 
/// * new() - Takes all fields as parameters, returns a PacketStruct type. Used to create a new instance of the struct.
pub struct PacketStruct {
    pub number: u32,
    pub time: String,
    pub source_mac: MacAddr,
    pub source_ip: IpAddr,
    pub source_port: u16,
    pub dest_mac: MacAddr,
    pub dest_ip: IpAddr,
    pub dest_port: u16,
    pub protocol: String,
    pub length: u32,
    pub payload: String
}

/// Constructor for 'PacketStruct'
impl PacketStruct {
    pub fn new(
        number: u32, 
        time: String, 
        source_mac: MacAddr,
        source_ip: IpAddr, 
        source_port: u16, 
        dest_mac: MacAddr,
        dest_ip: IpAddr, 
        dest_port: u16,
        protocol: String, 
        length: u32, 
        payload: String
        ) -> Self {
            PacketStruct {
                number, 
                time, 
                source_mac,
                source_ip, 
                source_port,
                dest_mac, 
                dest_ip, 
                dest_port, 
                protocol, 
                length, 
                payload,
            }
        }
}

fn main() {
    // Call interface_fn() and assign to variable
    let interface_checked = interface_fn();
    
    // Initialize variables that need a globalized scope
    let mut number = 0;

    // Call capture() passing the interface
    capture(interface_checked, &mut number);
}




