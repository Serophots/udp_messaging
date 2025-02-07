use std::{
    collections::{HashMap, VecDeque},
    io,
    net::{Ipv4Addr, SocketAddrV4, UdpSocket},
    sync::Arc,
};

use ratatui::{
    crossterm::{
        event::EnableMouseCapture,
        execute,
        terminal::{enable_raw_mode, EnterAlternateScreen},
    },
    widgets::ListState,
};
use serde::{Deserialize, Serialize};
use tui_input::Input;

use crate::{settings::Settings, Message};

mod logic;
mod rendering;

pub fn blocking_ui(app: App) -> anyhow::Result<Settings> {
    // setup terminal
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    // run app
    let terminal = ratatui::init();
    let app_result = app.run(terminal);

    // finish
    ratatui::restore();
    app_result
}

pub struct App {
    socket: Arc<UdpSocket>,
    recv: std::sync::mpsc::Receiver<Message>,
    localhost: SocketAddrV4,

    settings: Settings,

    exit: bool,
    focus: AppFocus,

    input: Input,
    messages: VecDeque<String>,
}
impl App {
    pub fn init(
        socket: Arc<UdpSocket>,
        recv: std::sync::mpsc::Receiver<Message>,
        localhost: SocketAddrV4,
        settings: Settings,
    ) -> Self {
        Self {
            socket,
            recv,
            localhost,

            settings,

            exit: false,
            focus: AppFocus::default(),

            input: Input::default(),
            messages: VecDeque::default(),
        }
    }
}

enum AppFocus {
    MessageInput,
    Contacts,
}
impl Default for AppFocus {
    fn default() -> Self {
        Self::Contacts
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactsList {
    contacts: HashMap<String, Ipv4Addr>,
    state: ListState,
}
impl Default for ContactsList {
    fn default() -> Self {
        let mut contacts = HashMap::new();
        contacts.insert(String::from("test1"), Ipv4Addr::new(0, 0, 0, 0));
        contacts.insert(String::from("test2"), Ipv4Addr::new(0, 0, 0, 0));
        contacts.insert(String::from("test3"), Ipv4Addr::new(0, 0, 0, 0));
        contacts.insert(String::from("test4"), Ipv4Addr::new(0, 0, 0, 0));
        Self {
            contacts,
            state: ListState::default(),
        }
    }
}
