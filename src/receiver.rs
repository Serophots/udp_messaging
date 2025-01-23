use std::sync::Arc;

use tokio::net::UdpSocket;

use crate::friends::Friends;

pub struct Receiver {
    pub socket: Arc<UdpSocket>,
    pub friends: Friends,
}
impl Receiver {
    pub async fn run(&mut self) {
        //Receiver
        let mut buf = [0; 4096];

        loop {
            //Receive a message
            let (size, addr) = self
                .socket
                .recv_from(&mut buf)
                .await
                .expect("failed to recv");

            if let Ok(msg) = String::from_utf8(Vec::from(&buf[0..size])) {
                let name = match addr.ip() {
                    std::net::IpAddr::V4(addr) => self.friends.lookup_addr(addr),
                    std::net::IpAddr::V6(_) => None,
                };

                println!(
                    "Recevied from {} message {}",
                    name.unwrap_or(addr.to_string()),
                    msg
                );
            } else {
                eprintln!(
                    "Received {} bytes from {} buffer {:?}",
                    size,
                    addr,
                    &buf[0..size]
                );
            }
        }
    }
}
