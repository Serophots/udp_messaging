use std::{
    net::{IpAddr, SocketAddrV4, UdpSocket},
    sync::Arc,
    thread,
};

use clap::{command, Parser};
use local_ip_address::local_ip;
use receiver::Receiver;
use settings::Settings;
use ui::{blocking_ui, App};

mod receiver;
mod settings;
mod ui;

type Message = (String, SocketAddrV4);

#[derive(Parser)]
#[command()]
struct Args {
    #[clap(short, long, default_value = "8080")]
    port: u16,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let Args { port } = args;
    let localhost = SocketAddrV4::new(
        match local_ip().expect("Failed to retreive your local network address") {
            IpAddr::V4(addr) => addr,
            IpAddr::V6(addr) => panic!("Local network address was ipv6 {}", addr),
        },
        port,
    );

    let settings = Settings::load()?;
    let socket = Arc::new(UdpSocket::bind(localhost).expect("Failed to bind to socket"));

    //Channels
    let (send_incoming_msg, recv_incoming_msg) = std::sync::mpsc::channel::<Message>();

    //Receiving
    let mut receiver = Receiver::init(socket.clone(), send_incoming_msg);
    thread::spawn(move || {
        receiver.run();
    });

    //Run UI
    let app = App::init(socket, recv_incoming_msg, localhost, settings);
    let settings = blocking_ui(app)?;

    //Save settings
    settings.save()?;

    Ok(())
}
