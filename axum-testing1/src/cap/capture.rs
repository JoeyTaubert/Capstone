use chrono::{DateTime, Utc};
use mongodb::{
    bson::{doc, Bson, Document},
    Client, Collection,
};
use pnet::datalink::{self, Channel, NetworkInterface};
use pnet::{
    packet::{
        arp::ArpPacket, ethernet::EthernetPacket, ip::IpNextHeaderProtocols, ipv4::Ipv4Packet,
        ipv6::Ipv6Packet, tcp::TcpPacket, udp::UdpPacket, Packet,
    },
    util::MacAddr,
};
use std::net::{IpAddr, Ipv4Addr};

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
pub async fn start_capture(interface: String, num_of_packets: u32) {
    println!(
        "\n[+]INFO: Capturing {} packets on {}...\n",
        num_of_packets, interface
    );

    let calc_num_of_packets = num_of_packets - 1;

    let interfaces = datalink::interfaces();
    let mut number = 0;
    // Get the network interface as &NetworkInterface type
    let interface_dl = interfaces
        .into_iter()
        .find(|iface: &NetworkInterface| iface.name == interface)
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
                        let packet_data: super::PacketStruct::PacketStruct =
                            parse_packet(&packet, number);

                        // Send to MongoDB using a separate async task
                        //// For each packet captured, this will create a database interaction. I want to combine these into batches to increase efficiency
                        tokio::spawn(async move {
                            insert_packet_to_mongo(packet_data).await.expect(
                                "[-]ERROR: Failed to start insert_packet_to_mongo function.",
                            );
                        });
                        //// Add error handling since insert_packet_to_mongo() returns a Result type
                    }
                    // If there is an error accessing the next ethernet frame, print an error to the error log
                    Err(e) => {
                        eprintln!("[-]ERROR: An error occured while reading {}", e);
                    }
                }
            }
        }
        // If a packet comes in with an unexpected header layout, print an error to the error log
        Ok(_) => {
            eprintln!("[-]ERROR: Unsupported channel type for packet, aka not Ethernet channel")
        }
        // Handles generic Err
        Err(e) => eprintln!(
            "[-]ERROR: An error occured while creating the datalink channel: {}",
            e
        ),
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
pub fn parse_packet(
    packet_data: &EthernetPacket,
    number: u32,
) -> super::PacketStruct::PacketStruct {
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
                source_ip = IpAddr::V4(header.get_source());
                dest_ip = IpAddr::V4(header.get_destination()); //create a function convert_ipv4_to_ip to convert Ipv4Addr to IpAddr type
                match header.get_next_level_protocol() {
                    // TCP
                    IpNextHeaderProtocols::Tcp => {
                        // Can further inspect TCP traffic for Client Hello for TLS
                        if let Some(tcp) = TcpPacket::new(header.payload()) {
                            // Grab source/destination TCP ports
                            source_port = tcp.get_source();
                            dest_port = tcp.get_destination();
                            protocol = String::from("TCP");
                        } else {
                            //Do nothing
                        }
                    }
                    // UDP
                    IpNextHeaderProtocols::Udp => {
                        // Can further inspect UDP traffic to see if it follows the same format as QUIC
                        if let Some(udp) = UdpPacket::new(header.payload()) {
                            source_port = udp.get_source();
                            dest_port = udp.get_destination();
                            protocol = String::from("UDP");
                        } else {
                            //Do nothing
                        }
                    }
                    // ICMP (ICMPv4)
                    IpNextHeaderProtocols::Icmp => {
                        // ICMP is a layer 3 protocol and does not have a port to extract
                        protocol = String::from("ICMP");
                    }
                    // HOPOPT, "Hop-by-Hop" IPv6 extension header
                    IpNextHeaderProtocols::Hopopt => protocol = String::from("HOPOPT"),
                    // For any other 'match' condition, print an error to the error log
                    _ => {
                        eprintln!(
                            "[-]ERROR: Unsupported next level protocol: {}",
                            header.get_next_level_protocol()
                        );
                    }
                }
            } else {
                // Do nothing
            }
        }
        pnet::packet::ethernet::EtherTypes::Ipv6 => {
            if let Some(header) = Ipv6Packet::new(packet_data.payload()) {
                // Grab source/destination IPv6
                source_ip = IpAddr::V6(header.get_source());
                dest_ip = IpAddr::V6(header.get_source());
                match header.get_next_header() {
                    // TCP
                    IpNextHeaderProtocols::Tcp => {
                        // Need to account for Udp, QUIC, TLS
                        if let Some(tcp) = TcpPacket::new(header.payload()) {
                            // Grab source/destination ports
                            source_port = tcp.get_source();
                            dest_port = tcp.get_destination();
                        } else {
                            //Do nothing
                        }
                    }
                    // UDP
                    IpNextHeaderProtocols::Udp => {
                        // Can further inspect UDP traffic to see if it follows the same format as QUIC
                        if let Some(udp) = UdpPacket::new(header.payload()) {
                            source_port = udp.get_source();
                            dest_port = udp.get_destination();
                            protocol = String::from("UDP");
                        } else {
                            //Do nothing
                        }
                    }
                    // ICMPv6
                    IpNextHeaderProtocols::Icmpv6 => {
                        protocol = String::from("ICMPv6");
                    }
                    _ => {
                        eprintln!(
                            "[-]ERROR: Unsupported next level protocol: {}",
                            header.get_next_header()
                        );
                    }
                }
            } else {
                //Do nothing
            }
        }
        pnet::packet::ethernet::EtherTypes::Arp => {
            if let Some(_arp) = ArpPacket::new(packet_data.payload()) {
                protocol = String::from("ARP");
            } else {
                //Do nothing
            }
        }
        _ => {
            eprintln!(
                "[-]ERROR: Unsupported ethertype: {:?}",
                packet_data.get_ethertype()
            );
        }
    };

    //Format:
    //println!("Number: {} | Time: {} | Protocol: {} | Source MAC: {} | Destination MAC: {} | Source IP: {} | Source Port: {} | Destination IP: {} | Destination Port: {} | Length: {} | Payload: {:?}\n", &number, &timestamp, &protocol, &source_mac, &dest_mac, &source_ip, &source_port, &dest_ip, &dest_port, &length, &ppayload);

    // Return an instance of PacketStruct so that the packet can be written to a file
    super::PacketStruct::PacketStruct::new(
        number,
        timestamp,
        protocol,
        source_mac,
        source_ip,
        source_port,
        dest_mac,
        dest_ip,
        dest_port,
        length,
        ppayload,
    )
}

pub async fn insert_packet_to_mongo(
    packet_data: super::PacketStruct::PacketStruct,
) -> Result<(), String> {
    let client = Client::with_uri_str("mongodb://127.0.0.1:27017")
        .await
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

    table
        .insert_one(new_doc, None)
        .await
        .map_err(|e| format!("[-]ERROR: Failed to insert document into MongoDB: {}", e))?;

    Ok(())
}
