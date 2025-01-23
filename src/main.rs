use std::io::stdin;
use std::net::{IpAddr, SocketAddrV4};
use std::sync::Arc;

use clap::Parser;
use friends::Friends;
use local_ip_address::local_ip;
use receiver::Receiver;
use tokio::net::UdpSocket;

mod commands;
mod friends;
mod receiver;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    port: Option<u16>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let port = args.port.unwrap_or(8080);

    println!("Top of the morning to you sir!");
    println!("Welcome to tangerines.cool v2\n\n");
    println!("Brief tutorial:");
    println!(" - ctrl + c to exit");
    println!(" - /send <address> to switch to a new address to send messages to");
    println!(" - /friend <name> <address> to store a friends address under a name");
    println!(" - listening to messages from all receivers at all times");

    let localhost = match local_ip().expect("Failed to retreive your local network address") {
        IpAddr::V4(addr) => addr,
        IpAddr::V6(addr) => panic!("Local network address was ipv6 {}", addr),
    };

    let socket = Arc::new(
        UdpSocket::bind(SocketAddrV4::new(localhost, port))
            .await
            .expect("Failed to bind to socket"),
    );
    let mut friends = Friends::new(localhost);
    println!("Established on {}\n\n", localhost);

    //Receiver
    let mut receiver = Receiver {
        socket: socket.clone(),
        friends: friends.clone(),
    };

    tokio::spawn(async move {
        receiver.run().await;
    });

    //Sender
    let mut target = localhost;

    loop {
        let mut message = String::new();
        stdin()
            .read_line(&mut message)
            .expect("failed to read_line stdin");
        let msg = message.trim();

        if msg.starts_with("/") {
            let args = msg.split(" ").collect::<Vec<_>>();
            match args[0] {
                "/ping" => {
                    println!("ping command")
                }
                "/send" => {
                    if let Some(to) = args.get(1) {
                        if let Some(addr) = friends.parse_address(to) {
                            println!("Sending messages to {}", addr);
                            target = addr;
                        } else {
                            eprintln!("Failed to parse address and no friend under that name")
                        }
                    } else {
                        eprintln!("/send <address>")
                    }
                }
                "/friend" => {
                    if let Some(name) = args.get(1) {
                        if let Some(addr) = args.get(2) {
                            match friends.new_friend(name, addr) {
                                Some(addr) => {
                                    println!("Added friend {} with address {}", name, addr);
                                }
                                None => {
                                    eprintln!("Failed to add friend");
                                }
                            };
                        } else {
                            eprintln!("/friend <name> <addr>")
                        }
                    } else {
                        eprintln!("/friend <name> <addr>")
                    }
                }
                _ => {}
            }
        } else {
            //Sending a message

            socket
                .send_to(&msg.as_bytes(), SocketAddrV4::new(target, port))
                .await
                .expect("failed to send");
        }
    }
}
