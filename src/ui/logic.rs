use std::{
    net::{Ipv4Addr, SocketAddrV4},
    sync::mpsc::TryRecvError,
};

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    DefaultTerminal,
};
use tui_input::backend::crossterm::EventHandler;

use crate::settings::Settings;

use super::{App, AppFocus};

//App logic
impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> anyhow::Result<Settings> {
        while !self.exit {
            match self.recv.try_recv() {
                Ok(msg) => self.messages.push_back(msg.0),
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {
                    return Err(anyhow::anyhow!("Receiver channel disconnected"));
                }
            }

            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            match event::read()? {
                // event::Event::FocusGained => todo!(),
                // event::Event::FocusLost => todo!(),
                event::Event::Key(key) => self.handle_key(key),
                // event::Event::Mouse(mouse_event) => todo!(),
                // event::Event::Paste(_) => todo!(),
                // event::Event::Resize(_, _) => todo!(),
                _ => {}
            }
        }
        Ok(self.settings)
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('c') => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.exit = true;
                    return;
                }
            }
            KeyCode::Tab => {
                match self.focus {
                    AppFocus::Contacts => self.focus = AppFocus::MessageInput,
                    AppFocus::MessageInput => self.focus = AppFocus::Contacts,
                }
                return;
            }
            _ => {}
        }
        match self.focus {
            AppFocus::Contacts => match key.code {
                KeyCode::Up => {
                    self.settings.contacts.state.select_previous();
                }
                KeyCode::PageUp => {
                    self.settings.contacts.state.select_first();
                }
                KeyCode::Down => {
                    self.settings.contacts.state.select_next();
                }
                KeyCode::PageDown => {
                    self.settings.contacts.state.select_last();
                }
                _ => {}
            },
            AppFocus::MessageInput => match key.code {
                KeyCode::Enter => {
                    let msg: String = self.input.value().into();

                    self.socket
                        .send_to(
                            &msg.as_bytes(),
                            SocketAddrV4::new(Ipv4Addr::new(192, 168, 1, 119), 7070),
                        )
                        .expect("Failed to send message");
                    self.messages.push_front(msg);

                    self.input.reset();
                }
                _ => {
                    self.input.handle_event(&Event::Key(key));
                }
            },
        }
    }
}
