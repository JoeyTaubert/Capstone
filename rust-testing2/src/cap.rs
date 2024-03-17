use pnet::{packet::{ipv4::Ipv4Packet, ipv6::Ipv6Packet, tcp::TcpPacket, udp::UdpPacket, Packet, ethernet::EthernetPacket, ip::IpNextHeaderProtocols, arp::ArpPacket}, util::MacAddr};
use pnet::datalink::{self, NetworkInterface, Channel};
use std::{io::Write, net::{IpAddr, Ipv4Addr, Ipv6Addr}, fs::OpenOptions};
use chrono::{Utc, DateTime};
use mongodb::{bson::{doc, Bson, Document}, Client, Collection};
use tokio;

// ------------------------
/// Lists available network interfaces
/// 
/// # Arguments
/// None
/// 
/// # Return 
/// * interfaces_vec: Vec<String> - Network interface list in a vector.
/// 
pub fn interface_list() -> Vec<String> {
    // Grab network interfaces
    let devices = pcap::Device::list().expect("[-]ERROR: Failed to grab network interfaces");
    
    // Print network interfaces
    let mut interfaces_vec: Vec<String> = Vec::new();
    
    println!("\n-=-=-=-=-=-=-=-=-=-=-=-");
    println!("    Interface List");
    println!("-=-=-=-=-=-=-=-=-=-=-=-");

    for device in devices{
        interfaces_vec.push(device.name.clone());
        println!("{}", device.name.clone());
    }
    println!("-=-=-=-=-=-=-=-=-=-=-=-\n");

    interfaces_vec //Implicit return for interfaces_vec
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
pub fn choose_int() -> String {
    // Call interface_list()
    interface_list();

    // Get the choice of interface
    let mut int_choice = String::new();
    println!("[+] Interface you would like to capture on:");

    // Record user input
    std::io::stdin().read_line(&mut int_choice).expect("[-]ERROR: Error, no valid interface selected");

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
/// * interface_input: String - The system interface that should be captured on
/// 
pub fn interface_fn() -> String {
    // Calls choose_int() and interface_list()
    let mut interface_input: String = choose_int();
    
    let devices = pcap::Device::list().expect("[-]ERROR: Failed to grab network interfaces");

    let mut all_interfaces = Vec::new();

    for device in devices{
        all_interfaces.push(device.name.clone());
    }

    // Begin loop, this will continue until user input matches a interface
    loop {
        match all_interfaces.contains(&interface_input) {
            true => break, // If the interface is found, break the loop
            false => {
                // If not found, print a message and ask for input again
                println!("ERROR[-]: Interface '{}' not found, please try again.", &interface_input);
                // I'm not sure if this is how I should be calling choose_int() or if I need to return differently
                interface_input = choose_int();
            }
        }
    }
    return interface_input;
}


// ------------------------
/// Starts a network capture
/// 
/// # Arguments
/// * interface: String - The selected interface (see main())
/// * num_of_packets: i32 - The provided number of packets that should be captured
/// 
/// # Returns
/// N/A
/// 
/// * Outputs a file name YYYY-MM-DD-HH-MM-SS-Capture.txt
pub fn capture(interface: String, num_of_packets: i32) {
    println!("\n[+]INFO: Capturing {} packets on {}...\n", num_of_packets, interface);

    let calc_num_of_packets = num_of_packets - 1;

    // Set up timestamp for file creation
    let rnow = Utc::now();
    let rnowformatted = rnow.format("%Y-%m-%d_%H-%M-%S").to_string();

    //// Perform a check if the file exists (is this the right place?) if not, create the file and write the header.

    // Grab file handle
    //// Use Path/PathBuf for this? 
    //let mut cfile = match OpenOptions::new()
    //.append(true)
    //.create(true)
    //.open(format!("caps/{}-Capture.txt", rnowformatted)) {
    //    Ok(file) => file,
    //    Err(e) => {
    //        eprintln!("[-]ERROR: An error occured while trying to acquire the file handle. Maybe the directory 'caps' is missing? {}", e);
    //        return;
    //    },
    //}; 
    
    let interfaces = datalink::interfaces();
    let mut number = 0;
    // Get the network interface as &NetworkInterface type
    let interface_dl = interfaces.into_iter()
                                                .filter(|iface: &NetworkInterface| iface.name == interface)
                                                .next()
                                                .expect("[-]ERROR: Error getting interface");

    // Handling packets so that only packets with Ethernet frames are processed further
    match datalink::channel(&interface_dl, Default::default()) {
        Ok(Channel::Ethernet(_, mut rx)) => {
            while calc_num_of_packets >= number {
                number += 1;
                // Calls the next ethernet frame
                match rx.next() {
                    Ok(packet) => {
                        // Store the etherenet frame in variable
                        let packet = EthernetPacket::new(packet).unwrap();
                        // Pass the packet data to the parse_packet()
                        let packet_data: PacketStruct = parse_packet(&packet, number);

                        // Send to MongoDB
                        //// For each packet captured, this will create a database interaction. I want to combine these into batches to increase efficiency
                        let future = async {
                            insert_packet_to_mongo(packet_data).await.expect("[-]ERROR: Failed to start insert_packet_to_mongo function.");
                        };

                        // Write to the file
                        //let data_string = format!("Number: {} | Time: {} | Protocol: {} | Source MAC: {} | Destination MAC: {} | Source IP: {} | Source Port: {} | Destination IP: {} | Destination Port: {} | Length: {} | Payload: {:?}", packet_data.number, packet_data.time, packet_data.protocol, packet_data.source_mac, packet_data.dest_mac, packet_data.source_ip, packet_data.source_port, packet_data.dest_ip, packet_data.dest_port, packet_data.length, packet_data.payload);
                        //writeln!(cfile, "{}", data_string)
                        //    .expect("[-]ERROR: Error writing to file.");

                    },
                    // If there is an error accessing the next ethernet frame, print an error to the error log
                    Err(e) => {
                        eprintln!("[-]ERROR: An error occured while reading {}", e);
                    }
                }
            }
        },
        // If a packet comes in with an unexpected header layout, print an error to the error log
        Ok(_) => eprintln!("[-]ERROR: Unsupported channel type for packet, aka not Ethernet channel"),
        // Handles generic Err
        Err(e) => eprintln!("[-]ERROR: An error occured while creating the datalink channel: {}", e),
        }

        println!("[+]INFO: Finished capturing {num_of_packets} packets!")
    }


// ------------------------
/// Parses each packet of the capture and grabs critical information
/// 
/// # Arguments
/// packet_data: &EthernetPacket - A reference to packet data from the pnet::EthernetPacket method
/// number - Usually comes from an outer loop where each iteration increments the number variable by one
/// 
/// # Returns
/// N/A
pub fn parse_packet(packet_data: &EthernetPacket, number: i32) -> PacketStruct {
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
                    // HOPOPT, "Hop-by-Hop" IPv6 extension header
                    IpNextHeaderProtocols::Hopopt => {
                        protocol = String::from("HOPOPT")
                    },
                    // For any other 'match' condition, print an error to the error log
                    _ => {
                        eprintln!("[-]ERROR: Unsupported next level protocol: {}", header.get_next_level_protocol());
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
                        eprintln!("[-]ERROR: Unsupported next level protocol: {}", header.get_next_header());
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
            eprintln!("[-]ERROR: Unsupported ethertype: {:?}", packet_data.get_ethertype());
        }
    };
    
    //Format: 
    println!("Number: {} | Time: {} | Protocol: {} | Source MAC: {} | Destination MAC: {} | Source IP: {} | Source Port: {} | Destination IP: {} | Destination Port: {} | Length: {} | Payload: {:?}\n", &number, &timestamp, &protocol, &source_mac, &dest_mac, &source_ip, &source_port, &dest_ip, &dest_port, &length, &ppayload);

    // Return an instance of PacketStruct so that the packet can be written to a file
    return PacketStruct::new(number, timestamp, protocol, source_mac,  source_ip, source_port, dest_mac, dest_ip, dest_port, length, ppayload);
    
}


pub async fn insert_packet_to_mongo(packet_data: PacketStruct) -> Result<(), String> {
    let client = Client::with_uri_str("mongodb://127.0.0.1:27017").await
        .map_err(|e| format!("[-]ERROR: Failed to connect to MongoDB: {}", e))?;
    let database = client.database("captures");
    let table: Collection<Document> = database.collection("packets");

    let new_doc = doc! {
        "number": packet_data.number,
        "timestamp": &packet_data.time.to_string(),
        "protocol": &packet_data.protocol,
        "source_mac": &packet_data.source_mac.to_string(),
        "source_ip": &packet_data.source_ip.to_string(),
        "source_port": packet_data.source_port.to_string(),
        "dest_mac": &packet_data.dest_mac.to_string(),
        "dest_ip": &packet_data.dest_ip.to_string(),
        "dest_port": packet_data.dest_port.to_string(),
        "length": packet_data.length.to_string(),
        "payload": packet_data.payload.iter().map(|&byte| Bson::Int32(byte as i32)).collect::<Vec<Bson>>(),
    };

    table.insert_one(new_doc, None).await
        .map_err(|e| format!("[-]ERROR: Failed to insert document into MongoDB: {}", e))?;

    Ok(())
}

// ------------------------
/// The "Packet" struct represents the critical data within a single packet of network traffic
/// 
/// # Fields
/// 
/// * number - Packet number in the capture
/// * time - Time from the start of the capture
/// * protocol - The highest level protocol used for the packet
/// * source_mac - Source MAC address
/// * source_ip - Source IP address
/// * source_port - Source port
/// * dest_mac - Destination MAC address
/// * dest_ip - Destintation IP address
/// * dest_port - Destination port
/// * length - Size of the packet, in bytes
/// * payload - Summary of the fields of the highest layer protocol
/// 
/// # impl's
/// 
/// * new() - Takes all fields as parameters, returns a PacketStruct type. Used to create a new instance of the struct.
pub struct PacketStruct {
    // Could implement lifetimes here if I would like to take in references, like &i32 and &DateTime<Utc>
    pub number: i32,
    pub time: DateTime<Utc>,
    pub protocol: String,
    pub source_mac: MacAddr,
    pub source_ip: IpAddr,
    pub source_port: u16,
    pub dest_mac: MacAddr,
    pub dest_ip: IpAddr,
    pub dest_port: u16,
    pub length: usize,
    pub payload: Vec<u8>
}

/// Constructor for 'PacketStruct'
impl PacketStruct {
    pub fn new(
        number: i32, 
        time: DateTime<Utc>, 
        protocol: String, 
        source_mac: MacAddr,
        source_ip: IpAddr, 
        source_port: u16, 
        dest_mac: MacAddr,
        dest_ip: IpAddr, 
        dest_port: u16,
        length: usize, 
        payload: Vec<u8>
        ) -> Self {
            PacketStruct {
                number, 
                time, 
                protocol, 
                source_mac,
                source_ip, 
                source_port,
                dest_mac, 
                dest_ip, 
                dest_port, 
                length, 
                payload,
            }
        }
}

#[tokio::main]
pub async fn main() {
    // Call interface_fn() and assign to variable
    let interface_checked = interface_fn();

    let mut packet_choice_i32: i32 = 0;
    // How many packets to be captured
    loop {
        let mut packet_choice_string = String::new();
        println!("\n[+] Numer of packets to be captured: ");
        std::io::stdin()
            .read_line(&mut packet_choice_string)
            .expect("[-]ERROR: Invalid input for packet number");

        // Check if user input is an integer, continue
        match packet_choice_string.trim().parse::<i32>(){
            Ok(num) => {
                packet_choice_i32 = num;
                break
            }
            Err(_) => {
                eprintln!("\n[-]ERROR: Invalid input for number of packets, '{}' not an integer.", packet_choice_string.trim());
            },
        };
    }

    // Call capture() passing the interface
    capture(interface_checked, packet_choice_i32);
}
