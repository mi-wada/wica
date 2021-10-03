use anyhow::Result;
use termion::event::Key;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};
use unicode_width::UnicodeWidthChar;

use crate::{
    components::ComponentState,
    event::{Event, EventSender},
    ui::default_key_handle,
};

pub struct Query {
    data: Vec<Vec<char>>,
    x_data_editing_at: usize,
    y_data_editing_at: usize,
    state: ComponentState,
}

impl Default for Query {
    fn default() -> Self {
        Self {
            data: vec![vec![]],
            x_data_editing_at: 0,
            y_data_editing_at: 0,
            state: ComponentState::UnFocused,
        }
    }
}

impl Query {
    pub fn key_handle(&mut self, k: Key, event_sender: EventSender) -> Result<()> {
        match self.state {
            ComponentState::Focused => match k {
                Key::Char('\n') => {
                    self.state = ComponentState::Editing;
                }
                _ => default_key_handle(k, event_sender)?,
            },
            ComponentState::Editing => match k {
                Key::Char('\n') => {
                    self.y_data_editing_at += 1;
                    self.x_data_editing_at = 0;
                    self.data.push(vec![]);
                }
                Key::Char(c) => {
                    self.data[self.y_data_editing_at].insert(self.x_data_editing_at, c);
                    self.x_data_editing_at += 1;
                    self.send_set_query_event(event_sender);
                }
                Key::Backspace => {
                    if self.x_data_editing_at > 0 {
                        self.data[self.y_data_editing_at].remove(self.x_data_editing_at - 1);
                        self.x_data_editing_at -= 1;
                    } else if self.y_data_editing_at > 0 {
                        let removed = self.data.remove(self.y_data_editing_at);
                        self.x_data_editing_at = self.data[self.y_data_editing_at - 1].len();
                        self.data[self.y_data_editing_at - 1].extend(removed);
                        self.y_data_editing_at -= 1;
                    }
                    self.send_set_query_event(event_sender);
                }
                Key::Up => {
                    if self.y_data_editing_at > 0 {
                        self.y_data_editing_at -= 1;
                    }
                }
                Key::Down => {
                    if self.y_data_editing_at < self.data.len() - 1 {
                        self.y_data_editing_at += 1;
                    }
                }
                Key::Left => {
                    if self.x_data_editing_at > 0 {
                        self.x_data_editing_at -= 1;
                    }
                }
                Key::Right => {
                    if self.x_data_editing_at < self.data[self.y_data_editing_at].len() {
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

    pub fn render<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        _parent_state: ComponentState,
    ) -> Result<()> {
        let widget = List::new(
            self.data
                .iter()
                .map(|d| {
                    let content = vec![Spans::from(Span::raw(
                        d.clone().into_iter().collect::<String>(),
                    ))];
                    ListItem::new(content)
                })
                .collect::<Vec<ListItem>>(),
        )
        .style(match self.state {
            ComponentState::Editing => Style::default().fg(Color::LightGreen),
            ComponentState::Focused => Style::default().fg(Color::Green),
            _ => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title("[Q]QUERY"));

        f.render_widget(widget, area);

        match self.state {
            ComponentState::Editing => f.set_cursor(
                area.x + 1 + self.x_cursor_postion(),
                area.y + 1 + self.y_cursor_postion(),
            ),
            _ => {}
        }
        Ok(())
    }

    fn x_cursor_postion(&self) -> u16 {
        self.data[self.y_data_editing_at][..self.x_data_editing_at]
            .iter()
            .fold(0, |acc, c| acc + c.width().unwrap() as u16)
    }

    fn y_cursor_postion(&self) -> u16 {
        self.y_data_editing_at as u16
    }

    fn send_set_query_event(&self, event_sender: EventSender) {
        event_sender.send(Event::SetQuery(self.data.clone().join(&'&')));
    }

    pub fn set_state(&mut self, state: ComponentState) {
        self.state = state;
    }

    pub fn set_data(&mut self, data: &Vec<char>) {
        let new_data = data
            .split(|c| *c == '&')
            .map(|v| v.to_vec())
            .collect::<Vec<Vec<char>>>();
        if self.data != new_data {
            self.data = new_data;
            self.x_data_editing_at = 0;
            self.y_data_editing_at = 0;
        }
    }

    pub fn is_focused(&self) -> bool {
        self.state.is_focused()
    }
}
