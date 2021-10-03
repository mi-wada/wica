use anyhow::Result;
use termion::event::Key;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use unicode_width::UnicodeWidthChar;

use crate::{
    components::ComponentState,
    event::{Event, EventSender},
    ui::default_key_handle,
};

pub struct Url {
    data: Vec<char>,
    x_data_editing_at: usize,
    state: ComponentState,
}

impl Default for Url {
    fn default() -> Self {
        Self {
            data: vec![],
            x_data_editing_at: 0,
            state: ComponentState::Focused,
        }
    }
}

impl Url {
    pub fn key_handle(&mut self, k: Key, event_sender: EventSender) -> Result<()> {
        match self.state {
            ComponentState::Focused => match k {
                Key::Char('\n') => {
                    self.state = ComponentState::Editing;
                }
                _ => default_key_handle(k, event_sender)?,
            },
            ComponentState::Editing => match k {
                Key::Char(c) if !c.is_ascii_control() => {
                    self.data.insert(self.x_data_editing_at, c);
                    self.x_data_editing_at += 1;
                    self.send_set_query_event(event_sender);
                }
                Key::Backspace => {
                    if self.x_data_editing_at > 0 {
                        self.data.remove(self.x_data_editing_at - 1);
                        self.x_data_editing_at -= 1;
                        self.send_set_query_event(event_sender);
                    }
                }
                Key::Left => {
                    if self.x_data_editing_at > 0 {
                        self.x_data_editing_at -= 1;
                    }
                }
                Key::Right => {
                    if self.x_data_editing_at < self.data.len() {
                        self.x_data_editing_at += 1;
                    }
                }
                Key::Esc => {
                    self.state = ComponentState::Focused;
                }
                _ => default_key_handle(k, event_sender)?,
            },
            _ => {}
        }

        Ok(())
    }

    fn x_cursor_postion(&self) -> u16 {
        self.data[..self.x_data_editing_at]
            .iter()
            .fold(0, |acc, c| acc + c.width().unwrap() as u16)
    }

    pub fn render<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        _parent_state: ComponentState,
    ) -> Result<()> {
        let widget = Paragraph::new(self.data.clone().into_iter().collect::<String>())
            .style(match self.state {
                ComponentState::Editing => Style::default().fg(Color::LightGreen),
                ComponentState::Focused => Style::default().fg(Color::Green),
                _ => Style::default(),
            })
            .block(Block::default().borders(Borders::ALL).title("[U]URL"));

        f.render_widget(widget, area);

        match self.state {
            ComponentState::Editing => {
                f.set_cursor(area.x + 1 + self.x_cursor_postion(), area.y + 1)
            }
            _ => {}
        }
        Ok(())
    }

    pub fn get_query(&self) -> Option<Vec<char>> {
        let split_at = match self.data.iter().position(|&c| c == '?') {
            None => return None,
            Some(i) => i,
        };
        let (_, query) = self.data.split_at(split_at);
        if query.len() == 1 {
            Some(vec![])
        } else {
            Some(query[1..].to_vec())
        }
    }

    fn send_set_query_event(&self, event_sender: EventSender) {
        if let Some(query) = self.get_query() {
            event_sender.send(Event::SetQuery(query));
        }
    }

    pub fn set_query(&mut self, query: &Vec<char>) {
        if let Some(old_query) = self.get_query() {
            if &old_query == query {
                return;
            }
        }
        self.data.truncate(
            self.data
                .iter()
                .position(|&c| c == '?')
                .unwrap_or(self.data.len()),
        );
        self.data.extend(vec!['?']);
        self.data.extend(query);

        self.x_data_editing_at = self.data.len();
    }

    pub fn get_data(&self) -> String {
        self.data.iter().collect()
    }

    pub fn set_state(&mut self, state: ComponentState) {
        self.state = state;
    }

    pub fn is_focused(&self) -> bool {
        self.state.is_focused()
    }
}
