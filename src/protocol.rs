use std::{io::{self, Error, ErrorKind}, time::Instant};

use crate::protocol::handshake::HandshakePacket;

pub(crate) mod handshake;

pub(crate) enum SrtPacket {
    Handshake(HandshakePacket),
    KeepAlive
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) enum PacketType {
    Data,
    Control
}

#[derive(Debug)]
pub(crate) struct PacketHeader {
    /// Defines whether a packet is a Data packet or a Control packet.
    pub packet_type: PacketType,
    pub timestamp: Instant,
    /// Marks which SRT socket ID is the destination ID for this packet.
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

/// Converts an array of bytes received into an SrtPacket.
/// 
/// Returns an error if the given byte array doesn't correspond to a valid SRT packet. 
pub(crate) fn to_packet(buffer: &[u8]) -> io::Result<SrtPacket> {
    if buffer.len() < 16 {
        return Err(Error::new(ErrorKind::InvalidData, "Packet size too small"));
    }

    let packet_type = {
        if buffer[0] & 128 == 0 {
            PacketType::Data
        } else {
            PacketType::Control
        }
    };
    let timestamp_value = u32::from_be_bytes([buffer[8], buffer[9], buffer[10], buffer[11]]);
    let dest_id = u32::from_be_bytes([buffer[12], buffer[13], buffer[14], buffer[15]]);

    let header = PacketHeader {
        packet_type,
        timestamp: Instant::now(), // PLACEHOLDER
        dest_id
    };

    if packet_type == PacketType::Control {
        let control_type = u16::from_be_bytes([buffer[0] - 128, buffer[1]]);
        let packet_buffer = &buffer[16..];
        match control_type {
            // Handshake
            0 => {
                let packet = HandshakePacket::new(packet_buffer, header)?;
                return Ok(SrtPacket::Handshake(packet));
            }
            // Keep alive
            1 => {

            }
            _ => {
                return Err(Error::new(ErrorKind::Unsupported, "Invalid control packet type"));
            }
        }
    } else {

    }
    todo!()
}