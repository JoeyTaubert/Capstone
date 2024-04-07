use std::net::IpAddr;

use chrono::{DateTime, Utc};
use pnet::util::MacAddr;

// ------------------------
/// The "Packet" struct represents the critical data within a single packet of network traffic
///
///
/// * new() - Takes all fields as parameters, returns a PacketStruct type. Used to create a new instance of the struct.
pub struct PacketStruct {
    // Could implement lifetimes here if I would like to take in references, like &i32 and &DateTime<Utc>
    pub number: u32,
    pub time: DateTime<Utc>,
    pub protocol: String,
    pub source_mac: MacAddr,
    pub source_ip: IpAddr,
    pub source_port: u16,
    pub dest_mac: MacAddr,
    pub dest_ip: IpAddr,
    pub dest_port: u16,
    pub length: usize,
    pub payload: Vec<u8>,
}

/// Constructor for 'PacketStruct'
/// https://rust-lang.github.io/rust-clippy/master/index.html#/too_many_arguments
impl PacketStruct {
    pub fn new(
        number: u32,
        time: DateTime<Utc>,
        protocol: String,
        source_mac: MacAddr,
        source_ip: IpAddr,
        source_port: u16,
        dest_mac: MacAddr,
        dest_ip: IpAddr,
        dest_port: u16,
        length: usize,
        payload: Vec<u8>,
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
