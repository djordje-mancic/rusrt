use std::{io::{self, Error, ErrorKind, Read}, net::{IpAddr}};

use crate::protocol::PacketHeader;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) enum HandshakeVersion {
    V4,
    V5
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) enum HandshakeEncryption {
    AES128,
    AES192,
    AES256
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) enum HandshakeExtensionType {
    HandshakeRequest,
    HandshakeResponse,
    KeyMaterialRequest,
    KeyMaterialResponse,
    StreamID,
    Congestion,
    Filter,
    Group
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) enum HandshakeType {
    Done,
    Agreement,
    Conclusion,
    WaveAHand,
    Induction
}

#[derive(Debug)]
pub(crate) struct HandshakeExtension {
    pub extension_type: HandshakeExtensionType
}

#[derive(Debug)]
pub(crate) struct HandshakePacket {
    /// Header of the packet
    pub header: PacketHeader,

    /// Handshake version number. Currently used values are 4 and 5.
    /// Values greater than 5 are reserved for future use.
    pub version: HandshakeVersion,
    pub encryption_field: Option<HandshakeEncryption>,
    pub extension_field: u16,
    pub init_packet_seq_num: u32,

    /// Maximum Transmission Unit (MTU) size in bytes.
    /// 
    /// Usually set to 1500 bytes, as is the default MTU size
    /// for Ethernet, but can be set to a different size.
    pub mtu: u32,

    /// Maximum Flow Window size specified in the number of packets.
    /// 
    /// Indicates the maximum number of data packets allowed to be
    /// "in flight".
    pub max_flow_window: u32,

    /// Indicates the type of Handshake packet
    pub hs_type: HandshakeType,
    pub socket_id: u32,
    pub syn_cookie: u32,
    pub peer_address: IpAddr,
    pub extension: Option<HandshakeExtension>
}

impl HandshakePacket { 
    pub fn new(buffer: &[u8], header: PacketHeader) -> io::Result<Self> {
        let mut read_buffer = buffer;

        let version = {
            let mut version_bytes = [0u8; 4];
            read_buffer.read_exact(&mut version_bytes)?;
            match u32::from_be_bytes(version_bytes) {
                4 =>  HandshakeVersion::V4,
                5 =>  HandshakeVersion::V5,
                unknown => {
                    return Err(Error::new(ErrorKind::InvalidInput, format!("Unknown handshake version number encountered ({unknown})")));
                }
            }
        };

        let encryption_field = {
            let mut encryption_bytes = [0u8; 2];
            read_buffer.read_exact(&mut encryption_bytes)?;
            match u16::from_be_bytes(encryption_bytes) {
                0 =>  None, // No encryption cipher advertised
                2 =>  Some(HandshakeEncryption::AES128),
                3 =>  Some(HandshakeEncryption::AES192),
                4 =>  Some(HandshakeEncryption::AES256),
                unknown => {
                    return Err(Error::new(ErrorKind::InvalidInput, format!("Unknown encryption number encountered ({unknown})")));
                }
            }
        };

        let extension_field = {
            let mut received_bytes = [0u8; 2];
            read_buffer.read_exact(&mut received_bytes)?;
            u16::from_be_bytes(received_bytes)
        };

        let init_packet_seq_num = {
            let mut received_bytes = [0u8; 4];
            read_buffer.read_exact(&mut received_bytes)?;
            u32::from_be_bytes(received_bytes)
        };

        let mtu = {
            let mut received_bytes = [0u8; 4];
            read_buffer.read_exact(&mut received_bytes)?;
            u32::from_be_bytes(received_bytes)
        };

        let max_flow_window = {
            let mut received_bytes = [0u8; 4];
            read_buffer.read_exact(&mut received_bytes)?;
            u32::from_be_bytes(received_bytes)
        };

        let hs_type = {
            let mut hs_bytes = [0u8; 4];
            read_buffer.read_exact(&mut hs_bytes)?;
            match u32::from_be_bytes(hs_bytes) {
                0 => HandshakeType::WaveAHand,
                1 => HandshakeType::Induction,
                0xFFFFFFFF => HandshakeType::Conclusion,
                0xFFFFFFFE => HandshakeType::Agreement,
                0xFFFFFFFD => HandshakeType::Done,
                unknown => {
                    return Err(Error::new(ErrorKind::InvalidInput, format!("Unknown handshake type number encountered ({unknown})")));
                }
            }
        };

        let socket_id = {
            let mut received_bytes = [0u8; 4];
            read_buffer.read_exact(&mut received_bytes)?;
            u32::from_be_bytes(received_bytes)
        };

        let syn_cookie = {
            let mut received_bytes = [0u8; 4];
            read_buffer.read_exact(&mut received_bytes)?;
            u32::from_be_bytes(received_bytes)
        };

        let peer_address = {
            let mut ip_address_bytes = [0u8; 16];
            read_buffer.read_exact(&mut ip_address_bytes)?;
            
            for i in 0..4 {
                ip_address_bytes[i*4..i*4+4].reverse();
            }

            let mut is_ipv6 = false;
            for byte in &ip_address_bytes[4..] {
                if *byte != 0 {
                    is_ipv6 = true;
                    break;
                }
            }

            if is_ipv6 {
                IpAddr::from(ip_address_bytes)
            } else {
                IpAddr::from([
                    ip_address_bytes[0], ip_address_bytes[1], ip_address_bytes[2], ip_address_bytes[3]
                ])
            }
        };

        let extension = {
            let extension_type_option = {
                let mut extension_bytes = [0u8; 2];
                read_buffer.read_exact(&mut extension_bytes)?;
                match u16::from_be_bytes(extension_bytes) {
                    0 => None,
                    1 => Some(HandshakeExtensionType::HandshakeRequest),
                    2 => Some(HandshakeExtensionType::HandshakeResponse),
                    3 => Some(HandshakeExtensionType::KeyMaterialRequest),
                    4 => Some(HandshakeExtensionType::KeyMaterialResponse),
                    5 => Some(HandshakeExtensionType::StreamID),
                    6 => Some(HandshakeExtensionType::Congestion),
                    7 => Some(HandshakeExtensionType::Filter),
                    8 => Some(HandshakeExtensionType::Group),
                    unknown => {
                        return Err(Error::new(ErrorKind::InvalidInput, format!("Unknown handshake type number encountered ({unknown})")));
                    }
                }
            };

            if let Some(extension_type) = extension_type_option {
                Some(HandshakeExtension {
                    extension_type
                })
            } else {
                None
            }
        };
        
        Ok(HandshakePacket {
            header,
            version,
            encryption_field,
            extension_field,
            init_packet_seq_num,
            mtu,
            max_flow_window,
            hs_type,
            socket_id,
            syn_cookie,
            peer_address,
            extension
        })
    }
}