#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::io;
use std::{
    collections::HashMap,
    io::Write,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use tui::backend::CrosstermBackend;
use tui::Terminal;

use config;
use crossterm::{
    event::{self, DisableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
};
use reqwest::Url;
mod app;
mod asset;
mod ui;
mod util;

extern crate chrono;

enum Event<I> {
    Input(I),
    Tick,
}

fn main() -> Result<(), io::Error> {
    setup_logger().expect("unable to set up logger");
    info!("starting up");

    let config = get_config();

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear().expect("Could not clear terminal");
    terminal.show_cursor().expect("Could not show cursor");
    enable_raw_mode().unwrap();

    // Setup input handling
    let (tx, rx) = mpsc::channel();

    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            // poll for tick rate duration, if no events, sent tick event.
            if event::poll(tick_rate - last_tick.elapsed()).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    tx.send(Event::Input(key)).unwrap();
                }
            }
            if last_tick.elapsed() >= tick_rate {
                tx.send(Event::Tick).unwrap();
                last_tick = Instant::now();
            }
        }
    });

    let mut app = app::App::new("Stonks", String::from("MSFT"), config);

    // Live prices websocket
    let (wstx, wsrx) = mpsc::channel();
    let api_key = app.config.get("api_key");
    if let Some(key) = api_key {
        thread::spawn(move || {
            let (mut socket, response) = tungstenite::connect(
                Url::parse(&format!("wss://ws.finnhub.io?token={}", key)).unwrap(),
            )
            .expect("cannot connect");
            loop {
                // poll for tick rate duration, if no events, sent tick event.
                wstx.send(1.0).unwrap();
            }
        });
    }

    terminal.clear()?;

    loop {
        terminal.draw(|mut f| ui::draw(&mut f, &mut app))?;
        terminal.set_cursor(20, 20).unwrap();
        match rx.recv().unwrap() {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode().unwrap();
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    )
                    .unwrap();
                    terminal.show_cursor()?;
                    break;
                }
                KeyCode::Char(c) => app.on_key(c),
                KeyCode::Left => app.on_left(),
                KeyCode::Up => app.on_up(),
                KeyCode::Right => app.on_right(),
                KeyCode::Down => app.on_down(),
                _ => {}
            },
            Event::Tick => {
                app.on_tick();
            }
        }
        if app.should_quit {
            break;
        }
    }
    terminal.clear()?;
    Ok(())
}

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

fn get_config() -> HashMap<String, String> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("config")).unwrap();
    settings.try_into::<HashMap<String, String>>().unwrap()
}
