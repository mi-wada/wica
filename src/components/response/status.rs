use crate::components::ComponentState;

use anyhow::Result;
use reqwest::StatusCode;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph},
    Frame,
};

pub struct Status {
    data: Option<StatusCode>,
}

impl Default for Status {
    fn default() -> Self {
        Self { data: None }
    }
}

impl Status {
    pub fn set_data(&mut self, data: StatusCode) {
        self.data = Some(data);
    }

    pub fn render<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        _parent_state: ComponentState,
    ) -> Result<()> {
        let widget = match self.data {
            None => Paragraph::new("STATUS: ".to_string()).block(Block::default()),
            Some(data) => {
                let paragraph =
                    Paragraph::new(format!("{} {}", "STATUS:", data)).block(Block::default());

                if data.is_success() {
                    paragraph.style(Style::default().fg(Color::Green))
                } else if data.is_client_error() {
                    paragraph.style(Style::default().fg(Color::Red))
                } else {
                    paragraph.style(Style::default())
                }
            }
        };

        f.render_widget(widget, area);

        Ok(())
    }
}
