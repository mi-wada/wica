mod body;
mod delay;
mod header;
mod status;
mod tab;
use body::Body;
use delay::Delay;
use header::Header;
use status::Status;
use tab::Tab;
use tui::layout::{Constraint, Direction, Layout};

use crate::components::{Component, ComponentState};
use crate::event::EventSender;
use crate::http_request;

use anyhow::Result;
use termion::event::Key;
use tui::{backend::Backend, layout::Rect, Frame};

#[derive(Clone, Copy)]
pub enum ResponseComponents {
    Body,
    Header,
}

pub struct Response {
    tab: Tab,
    pub body: Body,
    pub header: Header,
    pub status: Status,
    pub delay: Delay,
    state: ComponentState,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            tab: Tab::default(),
            body: Body::default(),
            header: Header::default(),
            status: Status::default(),
            delay: Delay::default(),
            state: ComponentState::UnFocused,
        }
    }
}

impl Response {
    pub fn set_data(&mut self, resp: http_request::Response) {
        self.body.set_data(resp.body);
        self.header.set_data(resp.header);
        self.status.set_data(resp.status);
        self.delay.set_data(resp.delay);
    }

    pub fn is_focused(&self) -> bool {
        self.state.is_focused()
    }

    pub fn unfocused(&mut self) {
        self.set_state(ComponentState::UnFocused);
        self.body.set_state(ComponentState::UnFocused);
        self.header.set_state(ComponentState::UnFocused);
    }
}

impl Component for Response {
    fn key_handle(&mut self, k: Key, event_sender: EventSender) -> Result<()> {
        if self.body.is_focused() {
            self.body.key_handle(k, event_sender)?;
        } else if self.header.is_focused() {
            self.header.key_handle(k, event_sender)?;
        }

        Ok(())
    }

    fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) -> Result<()> {
        let (tab_area, main_area, status_area, delay_area) = {
            let chunks = Layout::default()
                .margin(0)
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(2),
                        Constraint::Min(1),
                        Constraint::Length(2),
                    ]
                    .as_ref(),
                )
                .split(area);

            let status_delay_area = Layout::default()
                .margin(0)
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(30)].as_ref())
                .split(chunks[2]);
            (
                chunks[0],
                chunks[1],
                status_delay_area[0],
                status_delay_area[1],
            )
        };

        if self.body.is_focused() {
            self.body.render(f, main_area, self.state)?;
            self.tab
                .render(f, tab_area, self.state, ResponseComponents::Body)?;
        } else if self.header.is_focused() {
            self.header.render(f, main_area, self.state)?;
            self.tab
                .render(f, tab_area, self.state, ResponseComponents::Header)?;
        }
        self.status.render(f, status_area, self.state)?;
        self.delay.render(f, delay_area, self.state)?;

        Ok(())
    }

    fn set_state(&mut self, state: ComponentState) {
        self.state = state;
    }
}
