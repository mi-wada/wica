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

use crate::{components::ComponentState, event::EventSender, ui::default_key_handle};

pub struct Body {
    data: Vec<String>,
    data_display_from: usize,
    state: ComponentState,
}

impl Default for Body {
    fn default() -> Self {
        Self {
            data: vec![],
            data_display_from: 0,
            state: ComponentState::Focused,
        }
    }
}

impl Body {
    pub fn set_data(&mut self, data: Vec<String>) {
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
        parent_state: ComponentState,
    ) -> Result<()> {
        let widget = List::new(
            self.data[self.data_display_from..]
                .iter()
                .map(|d| {
                    let content = vec![Spans::from(Span::raw(d))];
                    ListItem::new(content)
                })
                .collect::<Vec<ListItem>>(),
        )
        .block(Block::default().borders(Borders::ALL));

        let widget = match parent_state {
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
