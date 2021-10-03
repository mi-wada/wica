use std::time::Duration;

use crate::components::ComponentState;

use anyhow::Result;
use tui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, Paragraph},
    Frame,
};

pub struct Delay {
    data: Option<Duration>,
}

impl Default for Delay {
    fn default() -> Self {
        Self { data: None }
    }
}

impl Delay {
    pub fn set_data(&mut self, data: Duration) {
        self.data = Some(data);
    }

    pub fn render<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        _parent_state: ComponentState,
    ) -> Result<()> {
        let widget = match self.data {
            None => Paragraph::new("RESPONSE TIME:".to_string()).block(Block::default()),
            Some(data) => Paragraph::new(format!(
                "{} {}{}",
                "RESPONSE TIME:",
                data.as_secs_f64(),
                "s"
            ))
            .block(Block::default()),
        };

        f.render_widget(widget, area);

        Ok(())
    }
}
