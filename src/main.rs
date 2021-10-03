mod app;
mod cli;
mod components;
mod event;
mod http_request;
mod ui;

use app::App;
use event::Event;

use anyhow::Result;
use std::io;
use structopt::StructOpt;
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};

#[tokio::main]
async fn main() -> Result<()> {
    let _cmd_args = crate::cli::Cli::from_args();

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = App::default();

    loop {
        terminal.draw(|f| {
            app.render(f).unwrap();
        })?;

        match app.events.next()? {
            Event::KeyInput(input) => {
                app.key_handle(input)?;
            }
            Event::Request => match app.request_handle().await {
                Ok(_) => {}
                // TODO: response fieldにerror_messageを表示する
                Err(_e) => {}
            },
            Event::Response(resp) => {
                app.response_handle(resp);
            }
            Event::SetQuery(query) => {
                app.set_query_handle(query);
            }
            Event::ChangeFocus(position) => {
                app.change_focus(position);
            }
            Event::Tick => continue,
            Event::Quit => break,
        }
    }

    Ok(())
}
