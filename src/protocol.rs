use std::{io::{self, Error, ErrorKind}, time::Instant};

pub enum SrtPacket {
    Handshake(HandshakePacket),
    KeepAlive
}

pub enum PacketType {
    Data,
    Control
}

pub struct PacketHeader {
    /// Defines whether a packet is a Data packet or a Control packet.
    pub packet_type: PacketType,
    pub timestamp: Instant,
    pub dest_id: u32
}

impl Default for PacketHeader {
    fn default() -> Self {
        Self { 
            packet_type: PacketType::Data,
            timestamp: Instant::now(), 
            dest_id: Default::default() 
        }
    }
}

pub struct HandshakePacket {
    /// Header of the packet
    pub header: PacketHeader,
    /// Handshake version number. Currently used values are 4 and 5.
    /// Values greater than 5 are reserved for future use.
    pub version: u32,
    pub encryption_field: u16,
    pub extension_field: u16,
}

impl HandshakePacket { 
    fn new(buffer: &[u8], header: PacketHeader) -> Self {
        todo!()
    }
}

pub fn to_packet(buffer: &[u8]) -> io::Result<SrtPacket> {
    if buffer.len() < 16 {
        return Err(Error::new(ErrorKind::InvalidData, "Packet size too small"));
    }
    let mut header = PacketHeader::default();
    if buffer[0] & 128 == 0 { // Data packet
        header.packet_type = PacketType::Data;
    } else { // Control packet
        header.packet_type = PacketType::Control;
        let control_type = u16::from_be_bytes([buffer[0] - 128, buffer[1]]);
        match control_type {
            // Handshake
            0 => {
                let packet = HandshakePacket::new(buffer, header);
            }
            // Keep alive
            1 => {

            }
            _ => {
                return Err(Error::new(ErrorKind::Unsupported, "Invalid control packet type"));
            }
        }
    }
    todo!()
}