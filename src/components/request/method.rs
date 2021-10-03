use crate::{components::ComponentState, event::EventSender, ui::default_key_handle};
use anyhow::Result;
use termion::event::Key;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

const METHODS: [reqwest::Method; 4] = [
    reqwest::Method::GET,
    reqwest::Method::POST,
    reqwest::Method::PUT,
    reqwest::Method::DELETE,
];

pub struct Method {
    data: &'static reqwest::Method,
    selected_method_index: usize,
    state: ComponentState,
}

impl Default for Method {
    fn default() -> Self {
        Self {
            data: &METHODS[0],
            selected_method_index: 0,
            state: ComponentState::UnFocused,
        }
    }
}

impl Method {
    pub fn key_handle(&mut self, k: Key, event_sender: EventSender) -> Result<()> {
        match self.state {
            ComponentState::Focused => match k {
                Key::Char('\n') => {
                    self.selected_method_index = (self.selected_method_index + 1) % METHODS.len();
                    self.data = &METHODS[self.selected_method_index];
                }
                _ => default_key_handle(k, event_sender)?,
            },
            ComponentState::Editing => match k {
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
        let widget = Paragraph::new(self.data.as_str())
            .style(match self.state {
                ComponentState::Editing => Style::default().fg(Color::LightGreen),
                ComponentState::Focused => Style::default().fg(Color::Green),
                _ => Style::default(),
            })
            .block(Block::default().borders(Borders::ALL).title("[M]Method"));

        f.render_widget(widget, area);

        Ok(())
    }

    pub fn set_state(&mut self, state: ComponentState) {
        self.state = state;
    }

    pub fn is_focused(&self) -> bool {
        self.state.is_focused()
    }

    pub fn get_data(&self) -> reqwest::Method {
        self.data.to_owned()
    }
}
