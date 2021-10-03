mod body;
mod method;
mod query;
mod url;
use body::Body;
use method::Method;
use query::Query;
use url::Url;

use tui::layout::{Constraint, Direction, Layout};

use crate::components::{Component, ComponentState};
use crate::event::EventSender;

use anyhow::Result;
use termion::event::Key;
use tui::{backend::Backend, layout::Rect, Frame};

pub struct Request {
    pub method: Method,
    pub url: Url,
    pub query: Query,
    pub body: Body,
    state: ComponentState,
}

impl<'a> Default for Request {
    fn default() -> Self {
        Self {
            method: Method::default(),
            url: Url::default(),
            query: Query::default(),
            body: Body::default(),
            state: ComponentState::Focused,
        }
    }
}

impl Request {
    pub fn is_focused(&self) -> bool {
        self.state.is_focused()
    }

    pub fn unfocused(&mut self) {
        self.set_state(ComponentState::UnFocused);
        self.url.set_state(ComponentState::UnFocused);
        self.method.set_state(ComponentState::UnFocused);
        self.query.set_state(ComponentState::UnFocused);
        self.body.set_state(ComponentState::UnFocused);
    }

    pub fn get_method(&self) -> reqwest::Method {
        self.method.get_data()
    }

    pub fn get_url(&self) -> String {
        self.url.get_data()
    }

    pub fn get_body(&self) -> String {
        self.body.get_data()
    }
}

impl Component for Request {
    fn key_handle(&mut self, k: Key, event_sender: EventSender) -> Result<()> {
        if self.method.is_focused() {
            self.method.key_handle(k, event_sender)?;
        } else if self.url.is_focused() {
            self.url.key_handle(k, event_sender)?;
        } else if self.query.is_focused() {
            self.query.key_handle(k, event_sender)?;
        } else if self.body.is_focused() {
            self.body.key_handle(k, event_sender)?;
        }

        Ok(())
    }

    fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) -> Result<()> {
        let (method_and_url_area, query_and_body_area) = {
            let chunks = Layout::default()
                .margin(0)
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
                .split(area);
            (chunks[0], chunks[1])
        };

        let (method_area, url_area) = {
            let chunks = Layout::default()
                .margin(0)
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(15), Constraint::Min(1)].as_ref())
                .split(method_and_url_area);
            (chunks[0], chunks[1])
        };

        let (query_area, body_area) = {
            let chunks = Layout::default()
                .margin(0)
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(query_and_body_area);
            (chunks[0], chunks[1])
        };

        self.url.render(f, url_area, self.state)?;
        self.method.render(f, method_area, self.state)?;
        self.query.render(f, query_area, self.state)?;
        self.body.render(f, body_area, self.state)?;

        Ok(())
    }

    fn set_state(&mut self, state: ComponentState) {
        self.state = state;
    }
}
