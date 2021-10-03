use std::str::FromStr;

use crate::components::request::Request;
use crate::components::response::{Response, ResponseComponents};
use crate::components::ComponentPosition;
use crate::components::{help_message::HelpMessage, Component, ComponentState};
use crate::event::{Event, Events};
use crate::http_request;

use anyhow::Result;
use termion::event::Key;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame,
};

pub struct App {
    pub events: Events,
    help_message: HelpMessage,
    request: Request,
    response: Response,
}

impl Default for App {
    fn default() -> Self {
        App {
            help_message: HelpMessage {},
            request: Request::default(),
            response: Response::default(),
            events: Events::new(),
        }
    }
}

impl App {
    pub fn render<B: Backend>(&mut self, f: &mut Frame<'_, B>) -> Result<()> {
        let (help_message_area, request_area, response_area) = {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Percentage(40),
                        Constraint::Min(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());
            (chunks[0], chunks[1], chunks[2])
        };
        self.help_message.render(f, help_message_area)?;
        self.request.render(f, request_area)?;
        self.response.render(f, response_area)?;

        Ok(())
    }

    pub fn key_handle(&mut self, k: Key) -> Result<()> {
        if self.request.is_focused() {
            self.request.key_handle(k, self.events.sender())?;
        } else if self.response.is_focused() {
            self.response.key_handle(k, self.events.sender())?;
        }

        Ok(())
    }

    pub fn change_focus(&mut self, position: ComponentPosition) {
        self.response.set_state(ComponentState::UnFocused);
        self.request.unfocused();
        match position {
            ComponentPosition::RequestMethod => {
                self.request.set_state(ComponentState::Focused);
                self.request.method.set_state(ComponentState::Focused);
            }
            ComponentPosition::RequestUrl => {
                self.request.set_state(ComponentState::Focused);
                self.request.url.set_state(ComponentState::Focused);
            }
            ComponentPosition::RequestQuery => {
                self.request.set_state(ComponentState::Focused);
                self.request.query.set_state(ComponentState::Focused);
            }
            ComponentPosition::RequestBody => {
                self.request.set_state(ComponentState::Focused);
                self.request.body.set_state(ComponentState::Focused);
            }
            ComponentPosition::Response(response_component) => {
                self.response.unfocused();
                self.response.set_state(ComponentState::Focused);
                match response_component {
                    ResponseComponents::Body => {
                        self.response.body.set_state(ComponentState::Focused);
                    }
                    ResponseComponents::Header => {
                        self.response.header.set_state(ComponentState::Focused);
                    }
                }
            }
        }
    }

    pub fn set_query_handle(&mut self, query: Vec<char>) {
        self.request.query.set_data(&query);
        self.request.url.set_query(&query);
    }

    pub async fn request_handle(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match http_request::request(
            &reqwest::Request::new(
                self.request.get_method(),
                reqwest::Url::from_str(&self.request.get_url())?,
            ),
            self.request.get_body(),
        )
        .await?
        {
            Some(resp) => self.events.sender().send(Event::Response(resp)),
            None => {}
        }
        Ok(())
    }

    pub fn response_handle(&mut self, resp: http_request::Response) {
        self.response.set_data(resp);
    }
}
