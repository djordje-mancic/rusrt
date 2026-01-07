use std::{io::{self, Error, Read, Write}, net::{SocketAddr, ToSocketAddrs, UdpSocket}};

use crate::protocol::{handshake::HandshakePacket, SrtPacket, to_packet};

mod protocol;

pub struct SrtStream {
    socket: UdpSocket
}

impl SrtStream {
    pub fn connect<A: ToSocketAddrs>(peer_address: A) {

    }

    fn new(socket: UdpSocket) -> Self {
        SrtStream {  
            socket
        }
    }
}

impl Read for SrtStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        todo!()
    }
}

impl Write for SrtStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> io::Result<()> {
        todo!()
    }
}

pub struct SrtListener {
    socket: UdpSocket
}

impl SrtListener {
    pub fn bind<A: ToSocketAddrs>(address: A) -> io::Result<Self> {
        let udp_socket = UdpSocket::bind(address)?;

        let listener = SrtListener {
            socket: udp_socket
        };

        Ok(listener)
    }

    pub fn accept(&self) -> io::Result<(SrtStream, SocketAddr)> {
        let mut buf = [0u8; 65535];
        let result = self.socket.recv_from(&mut buf)?;
        println!("{:?}, {}", &buf[..result.0], result.1);
        let packet = to_packet(&buf)?;
        let SrtPacket::Handshake(handshake_recv) = packet else {
            todo!()
        };
        println!("{:?}", handshake_recv);

        todo!()
    }
}