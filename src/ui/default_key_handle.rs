use anyhow::Result;
use termion::event::Key;

use crate::{
    components::{response, ComponentPosition},
    event::{Event, EventSender},
};

pub fn default_key_handle(k: Key, event_sender: EventSender) -> Result<()> {
    match k {
        Key::Ctrl('c') => event_sender.send(Event::Quit),
        Key::Ctrl('s') => event_sender.send(Event::Request),
        Key::Char('m') => {
            event_sender.send(Event::ChangeFocus(ComponentPosition::RequestMethod));
        }
        Key::Char('u') => {
            event_sender.send(Event::ChangeFocus(ComponentPosition::RequestUrl));
        }
        Key::Char('q') => {
            event_sender.send(Event::ChangeFocus(ComponentPosition::RequestQuery));
        }
        Key::Char('r') => {
            event_sender.send(Event::ChangeFocus(ComponentPosition::RequestBody));
        }
        Key::Char('b') => {
            event_sender.send(Event::ChangeFocus(ComponentPosition::Response(
                response::ResponseComponents::Body,
            )));
        }
        Key::Char('h') => {
            event_sender.send(Event::ChangeFocus(ComponentPosition::Response(
                response::ResponseComponents::Header,
            )));
        }
        _ => {}
    }
    Ok(())
}
