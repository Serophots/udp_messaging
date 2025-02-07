use std::{
    net::{SocketAddr, UdpSocket},
    sync::Arc,
};

use crate::Message;

pub struct Receiver {
    socket: Arc<UdpSocket>,
    send: std::sync::mpsc::Sender<Message>,
}
impl Receiver {
    pub fn init(socket: Arc<UdpSocket>, send: std::sync::mpsc::Sender<Message>) -> Self {
        Self { socket, send }
    }

    pub fn run(&mut self) {
        let mut buf = [0; 4096];

        loop {
            let (size, addr) = self
                .socket
                .recv_from(&mut buf)
                .expect("failed to recv_from on socket");

            if let Ok(msg) = String::from_utf8(Vec::from(&buf[0..size])) {
                if let SocketAddr::V4(addr) = addr {
                    self.send
                        .send((msg, addr))
                        .expect("receiver failed to send message to channel");
                }
            }
        }
    }
}
