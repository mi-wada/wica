use crate::components::{response::ResponseComponents, ComponentState};

use anyhow::Result;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Tab {}

impl Default for Tab {
    fn default() -> Self {
        Self {}
    }
}

impl Tab {
    pub fn render<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
        parent_state: ComponentState,
        selected_tab: ResponseComponents,
    ) -> Result<()> {
        let mut body_tab = Paragraph::new("[B]Body")
            .style(Style::default().fg(Color::DarkGray))
            .block(
                Block::default()
                    .borders(Borders::TOP.union(Borders::LEFT.union(Borders::RIGHT)))
                    .border_style(Style::default().fg(Color::DarkGray)),
            );
        let mut header_tab = Paragraph::new("[H]Header")
            .style(Style::default().fg(Color::DarkGray))
            .block(
                Block::default()
                    .borders(Borders::TOP.union(Borders::LEFT.union(Borders::RIGHT)))
                    .border_style(Style::default().fg(Color::DarkGray)),
            );
        match selected_tab {
            ResponseComponents::Body => {
                body_tab = body_tab.style(Style::default()).block(
                    Block::default()
                        .borders(Borders::TOP.union(Borders::LEFT.union(Borders::RIGHT))),
                )
            }
            ResponseComponents::Header => {
                header_tab = header_tab.style(Style::default()).block(
                    Block::default()
                        .borders(Borders::TOP.union(Borders::LEFT.union(Borders::RIGHT))),
                )
            }
        }

        match parent_state {
            ComponentState::Focused => match selected_tab {
                ResponseComponents::Body => {
                    body_tab = body_tab.style(Style::default().fg(Color::Green));
                }
                ResponseComponents::Header => {
                    header_tab = header_tab.style(Style::default().fg(Color::Green));
                }
            },
            _ => {}
        }

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(10),
                    Constraint::Length(13),
                    Constraint::Min(1),
                ]
                .as_ref(),
            )
            .split(area);

        f.render_widget(body_tab, chunks[0]);
        f.render_widget(header_tab, chunks[1]);

        Ok(())
    }
}
