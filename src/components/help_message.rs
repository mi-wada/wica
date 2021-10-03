use crate::components::{Component, ComponentState};
use crate::event::EventSender;

use anyhow::Result;
use termion::event::Key;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Modifier, Style},
    text::{Span, Spans, Text},
    widgets::Paragraph,
    Frame,
};

pub struct HelpMessage {}

impl Component for HelpMessage {
    fn key_handle(&mut self, _k: Key, _: EventSender) -> Result<()> {
        Ok(())
    }

    fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) -> Result<()> {
        let (msg, style) = (
            vec![
                Span::styled("Ctrl + s", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(": send request"),
            ],
            Style::default(),
        );
        let mut text = Text::from(Spans::from(msg));
        text.patch_style(style);
        let help_message = Paragraph::new(text);
        f.render_widget(help_message, area);
        Ok(())
    }

    fn set_state(&mut self, _state: ComponentState) {}
}
