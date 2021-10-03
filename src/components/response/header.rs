use crate::{components::ComponentState, event::EventSender, ui::default_key_handle};

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

pub struct Header {
    data: Vec<(String, String)>,
    data_display_from: usize,
    state: ComponentState,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            data: vec![],
            data_display_from: 0,
            state: ComponentState::UnFocused,
        }
    }
}

impl Header {
    pub fn set_data(&mut self, data: Vec<(String, String)>) {
        self.data = data;
    }

    pub fn key_handle(&mut self, k: Key, event_sender: EventSender) -> Result<()> {
        match k {
            Key::Char('j') => {
                if self.data_display_from + 1 < self.data.len() {
                    self.data_display_from += 1;
                }
            }
            Key::Char('k') => {
                if self.data_display_from > 0 {
                    self.data_display_from -= 1;
                }
            }
            _ => default_key_handle(k, event_sender)?,
        }

        Ok(())
    }

    pub fn render<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        state: ComponentState,
    ) -> Result<()> {
        let widget = List::new(
            self.data[self.data_display_from..]
                .iter()
                .map(|d| {
                    let content = vec![Spans::from(Span::raw(format!("{}: {}", d.0, d.1)))];
                    ListItem::new(content)
                })
                .collect::<Vec<ListItem>>(),
        )
        .block(Block::default().borders(Borders::ALL));

        let widget = match state {
            ComponentState::Focused => widget.style(Style::default().fg(Color::Green)),
            _ => widget,
        };

        f.render_widget(widget, area);

        Ok(())
    }

    pub fn set_state(&mut self, state: ComponentState) {
        self.state = state;
    }

    pub fn is_focused(&self) -> bool {
        self.state.is_focused()
    }
}
