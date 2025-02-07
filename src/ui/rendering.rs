use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Styled},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListDirection, ListItem, Paragraph, StatefulWidget, Widget},
};

use super::{App, AppFocus};

//App rendering
impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [content_area, instructions_area, input_area] = Layout::vertical([
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(3),
        ])
        .areas(area);

        let [messages_area, contacts_area] =
            Layout::horizontal([Constraint::Min(1), Constraint::Max(16)]).areas(content_area);

        self.render_messages(messages_area, buf);
        self.render_contacts(contacts_area, buf);
        self.render_instructions(instructions_area, buf);
        self.render_input(input_area, buf);
    }
}
impl App {
    fn render_contacts(&mut self, area: Rect, buf: &mut Buffer) {
        //Render contacts
        let contacts = self
            .settings
            .contacts
            .contacts
            .keys()
            .map(|c| ListItem::new(vec![Line::from(Span::raw(c))]))
            .collect::<Vec<_>>();

        let contacts = List::new(contacts)
            .direction(ListDirection::TopToBottom)
            .block(Block::default().borders(Borders::ALL).title("Contacts"))
            .style(match self.focus {
                AppFocus::Contacts => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .highlight_symbol(">")
            .highlight_style(match self.focus {
                AppFocus::Contacts => Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::RAPID_BLINK),
                _ => Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Yellow),
            });

        StatefulWidget::render(contacts, area, buf, &mut self.settings.contacts.state);
    }

    fn render_messages(&self, area: Rect, buf: &mut Buffer) {
        let messages = self
            .messages
            .iter()
            .map(|m| ListItem::new(vec![Line::from(Span::raw(m))]))
            .collect::<Vec<_>>();

        let messages = List::new(messages)
            .direction(ListDirection::BottomToTop)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Listening on {}", self.localhost)),
            );

        Widget::render(messages, area, buf);
    }

    fn render_instructions(&self, area: Rect, buf: &mut Buffer) {
        let instruction = Paragraph::new(Text::from(Line::from(vec![
            Span::raw("Press "),
            Span::styled("Tab", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" and "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to navigate, "),
            Span::styled("Ctrl + C", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit."),
        ])))
        .set_style(Style::default());

        Widget::render(instruction, area, buf);
    }

    fn render_input(&mut self, area: Rect, buf: &mut Buffer) {
        // let width = area.width.max(3) - 3;
        let input = Paragraph::new(self.input.value())
            .style(match self.focus {
                AppFocus::MessageInput => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Send a message"),
            );
        Widget::render(input, area, buf);
    }
}
