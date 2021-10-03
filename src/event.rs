use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;

use crate::components::ComponentPosition;
use crate::http_request;

pub enum Event<I> {
    KeyInput(I),
    Tick,
    Quit,
    SetQuery(Vec<char>),
    Request,
    Response(http_request::Response), // TODO: Option<http_request::Response>に変更する
    ChangeFocus(ComponentPosition),
}

pub struct Events {
    tx: mpsc::Sender<Event<Key>>,
    rx: mpsc::Receiver<Event<Key>>,
    #[allow(dead_code)]
    input_handle: thread::JoinHandle<()>,
    #[allow(dead_code)]
    tick_handle: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl Events {
    pub fn new() -> Events {
        Events::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> Events {
        let (tx, rx) = mpsc::channel();
        let input_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.keys() {
                    if let Ok(key) = evt {
                        if let Err(err) = tx.send(Event::KeyInput(key)) {
                            eprintln!("{}", err);
                            return;
                        }
                    }
                }
            })
        };
        let tick_handle = {
            let tx = tx.clone();
            thread::spawn(move || loop {
                if let Err(err) = tx.send(Event::Tick) {
                    eprintln!("{}", err);
                    break;
                }
                thread::sleep(config.tick_rate);
            })
        };
        Events {
            tx,
            rx,
            input_handle,
            tick_handle,
        }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }

    pub fn sender(&self) -> EventSender {
        EventSender::new(&self.tx)
    }
}

pub struct EventSender<'a> {
    tx: &'a mpsc::Sender<Event<Key>>,
}

impl<'a> EventSender<'a> {
    pub fn new(tx: &mpsc::Sender<Event<Key>>) -> EventSender {
        EventSender { tx }
    }

    pub fn send(&self, event: Event<Key>) {
        if let Err(err) = self.tx.send(event) {
            eprintln!("{}", err);
            return;
        }
    }
}
