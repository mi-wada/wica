pub mod help_message;
pub mod request;
pub mod response;

use crate::event::EventSender;

use anyhow::Result;
use termion::event::Key;
use tui::{backend::Backend, layout::Rect, Frame};

pub trait Component {
    fn key_handle(&mut self, k: Key, event_sender: EventSender) -> Result<()>;
    fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) -> Result<()>
    where
        Self: Sized;
    fn set_state(&mut self, state: ComponentState);
}

#[derive(Clone, Copy)]
pub enum ComponentState {
    UnFocused,
    Focused,
    Editing,
}

impl ComponentState {
    pub fn is_focused(&self) -> bool {
        match *self {
            ComponentState::UnFocused => false,
            _ => true,
        }
    }
}

#[derive(Clone, Copy)]
pub enum ComponentPosition {
    RequestMethod,
    RequestUrl,
    RequestQuery,
    RequestBody,
    Response(response::ResponseComponents),
}

impl ComponentPosition {
    pub fn position(&self) -> (isize, isize) {
        match *self {
            ComponentPosition::RequestMethod => (0, 0),
            ComponentPosition::RequestUrl => (1, 0),
            ComponentPosition::RequestQuery => (0, 1),
            ComponentPosition::RequestBody => (1, 1),
            ComponentPosition::Response(response_component) => match response_component {
                response::ResponseComponents::Body => (0, 2),
                response::ResponseComponents::Header => (1, 2),
            },
        }
    }

    pub fn from_position(coordinate: (isize, isize)) -> Option<ComponentPosition> {
        match coordinate {
            (0, 0) => Some(ComponentPosition::RequestMethod),
            (1, 0) => Some(ComponentPosition::RequestUrl),
            (0, 1) => Some(ComponentPosition::RequestQuery),
            (1, 1) => Some(ComponentPosition::RequestBody),
            (0, 2) => Some(ComponentPosition::Response(
                response::ResponseComponents::Body,
            )),
            (1, 2) => Some(ComponentPosition::Response(
                response::ResponseComponents::Header,
            )),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn up(&self) -> ComponentPosition {
        let current_position = self.position();
        match ComponentPosition::from_position((current_position.0, current_position.1 - 1)) {
            Some(position) => position,
            None => *self,
        }
    }

    #[allow(dead_code)]
    pub fn down(&self) -> ComponentPosition {
        let current_position = self.position();
        match ComponentPosition::from_position((current_position.0, current_position.1 + 1)) {
            Some(position) => position,
            None => *self,
        }
    }

    #[allow(dead_code)]
    pub fn right(&self) -> ComponentPosition {
        let current_position = self.position();
        match ComponentPosition::from_position((current_position.0 + 1, current_position.1)) {
            Some(position) => position,
            None => *self,
        }
    }

    #[allow(dead_code)]
    pub fn left(&self) -> ComponentPosition {
        let current_position = self.position();
        match ComponentPosition::from_position((current_position.0 - 1, current_position.1)) {
            Some(position) => position,
            None => *self,
        }
    }
}
