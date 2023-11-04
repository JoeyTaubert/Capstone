use pnet::{packet::{ipv4::Ipv4Packet, ipv6::Ipv6Packet, tcp::TcpPacket, udp::UdpPacket, Packet, ethernet::EthernetPacket, ip::IpNextHeaderProtocols}, util::MacAddr};
use pnet::datalink::{self, NetworkInterface, Channel};
use std::{io, collections::HashMap, net::{IpAddr, Ipv4Addr, Ipv6Addr}};

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

    // Initialize int_safe for use in the loop
    let mut int_safe = String::new();

    // Loop through all interfaces
    loop {
        // If the input matches a interface in the list, write it to int_safe and break
        if all_interfaces.contains(&interface_input) {
            int_safe = interface_input;
            break;
        // else, try again
        } else {
            println!("Interface '{}' not found, please try again.", &interface_input);
            interface_input = choose_int();
        }
    }
    return int_safe;
}


// ------------------------
/// Starts a network capture
/// 
/// # Arguments
/// * interface: String - 
/// 
/// # Returns
/// N/A
fn capture(interface: String) {
    println!("Capturing on {}...\n", interface);
    
    let interfaces = datalink::interfaces();
    // Get the network interface as &NetworkInterface type
    let interface_dl = interfaces.into_iter()
                                                .filter(|iface: &NetworkInterface| iface.name == interface)
                                                .next()
                                                .expect("Error getting interface");

    // Handling packets so that only packets with Ethernet frames are processed further
    match datalink::channel(&interface_dl, Default::default()) {
        Ok(Channel::Ethernet(_, mut rx)) => {
            loop {
                // Calls the next ethernet frame
                match rx.next() {
                    Ok(packet) => {
                        // Store the etherenet frame in variable
                        let packet = EthernetPacket::new(packet).unwrap();
                        // Pass the packet data to the parse_packet()
                        parse_packet(&packet);
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
fn parse_packet(packet_data: &EthernetPacket) {
    // Initialize all needed fields
    let source_mac: MacAddr = packet_data.get_source(); // We already have direct access to layer 2 info, so assign these variables
    let dest_mac: MacAddr = packet_data.get_destination();
    let mut source_ip: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let mut dest_ip: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let mut source_port: u16 = 0;
    let mut dest_port: u16 = 0;

    // 'match' statement to differentiate between IPv4 header and IPv6
    match packet_data.get_ethertype() {
        pnet::packet::ethernet::EtherTypes::Ipv4 => {
            if let Some(header) = Ipv4Packet::new(packet_data.payload()) {
                // Grab source/destination IPv4s
                source_ip = IpAddr::V4(Ipv4Addr::from(header.get_source()));
                dest_ip = IpAddr::V4(Ipv4Addr::from(header.get_destination())); //create a function convert_ipv4_to_ip to convert Ipv4Addr to IpAddr type
                match header.get_next_level_protocol() {
                    IpNextHeaderProtocols::Tcp => { // Need to account for Udp, QUIC, TLS, etc.
                        if let Some(tcp) = TcpPacket::new(header.payload()) {
                            // Grab source/destination TCP ports
                            source_port = tcp.get_source();
                            dest_port = tcp.get_destination(); 
                        } else {
                            //Do nothing
                        }
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
                    IpNextHeaderProtocols::Tcp => { // Need to account for Udp, QUIC, TLS
                            if let Some(tcp) = TcpPacket::new(header.payload()) {
                            // Grab source/destination ports
                            source_port = tcp.get_source();
                            dest_port = tcp.get_destination();
                            } else {
                                //Do nothing
                            }
                    },
                    _ => {
                        eprintln!("Error");
                    }
                }
            } else {
                //Do nothing
            }
        },
        _ => {
            eprintln!("[+]INFO: Unsupported ethertype: {:?}", packet_data.get_ethertype());
        }
    };
    println!("Source MAC: {} | Destination MAC: {} | Source IP: {} | Source Port: {} | Destination IP: {} | Destination Port: {}", source_mac, dest_mac, source_ip, source_port, dest_ip, dest_port);
}

// ------------------------
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
    pub info: String
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
        info: String
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
                info,
            }
        }
}

fn main() {
    // Call interface_fn() and assign to variable
    let interface_checked = interface_fn();
    
    // Call capture() passing the interface
    capture(interface_checked);
}




