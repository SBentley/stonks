use tui::{
    Frame,
    symbols,
    backend::{Backend, CrosstermBackend},
    style::{Color, Modifier, Style},
    layout::{Constraint, Direction, Layout, Rect},
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{
        Tabs, Text,
        Axis, Block, Borders, Chart, Dataset, Paragraph, Row, Sparkline,
    },
};

use crate::app::{InputMode, App};
use unicode_width::UnicodeWidthStr;
//use crate::demo::App;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(3), Constraint::Percentage(7), Constraint::Percentage(90)].as_ref())
        .split(f.size());


    let msg = match app.input_mode {
        InputMode::Normal => "Search for an asset",
        InputMode::Editing => "Press Esc to stop editing, Enter to record the message",
    };
    let text = [Text::raw(msg)];
    let help_message = Paragraph::new(text.iter());
    f.render_widget(help_message, chunks[0]);

    let text = [Text::raw(&app.input)];
    let input = Paragraph::new(text.iter())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Search"))
        .style(Style::default().fg(Color::Red));
    f.render_widget(input, chunks[1]);
    
    
    match app.tabs.index {
        0 => draw_first_tab(f, app, chunks[2]),
        _ => {}
    };
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [                
                Constraint::Percentage(60),
                Constraint::Length(7),
            ]
            .as_ref(),
        )
        .split(area);
    //draw_gauges(f, app, chunks[0]);    
    draw_charts(f, app, chunks[0]);
    draw_text(f, chunks[1]);
}

fn draw_charts<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let constraints = if app.show_chart {
        vec![Constraint::Percentage(0), Constraint::Percentage(50)]
    } else {
        vec![Constraint::Percentage(100)]
    };
    let chunks = Layout::default()
        .constraints(constraints)
        .direction(Direction::Horizontal)
        .split(area);

    if app.show_chart {
        let x_labels = [
            format!("{}", app.signals.window[0]),
            format!("{}", (app.signals.window[0] + app.signals.window[1]) / 2.0),
            format!("{}", app.signals.window[1]),
        ];
        let datasets = [
            Dataset::default()
                .name("data2")
                .marker(symbols::Marker::Dot)
                .style(Style::default().fg(Color::Cyan))
                .data(&app.signals.sin1.points),
            Dataset::default()
                .name("data3")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::LightGreen))
                .data(&app.signals.sin2.points),
        ];
        let chart = Chart::default()
            .block(
                Block::default()
                    .title("Chart")
                    .title_style(Style::default().fg(Color::Cyan).modifier(Modifier::BOLD))
                    .borders(Borders::ALL),
            )
            .x_axis(
                Axis::default()
                    .title("X Axis")
                    .style(Style::default().fg(Color::Gray))
                    .labels_style(Style::default().modifier(Modifier::ITALIC))
                    .bounds(app.signals.window)
                    .labels(&x_labels),
            )
            .y_axis(
                Axis::default()
                    .title("Y Axis")
                    .style(Style::default().fg(Color::Gray))
                    .labels_style(Style::default().modifier(Modifier::ITALIC))
                    .bounds([-20.0, 20.0])
                    .labels(&["-20", "0", "20"]),
            )
            .datasets(&datasets);
        f.render_widget(chart, chunks[1]);
    }
}

fn draw_text<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let text = [
        Text::raw("This is a paragraph with several lines. You can change style your text the way you want.\n\nFor example: "),
        Text::styled("under", Style::default().fg(Color::Red)),
        Text::raw(" "),
        Text::styled("the", Style::default().fg(Color::Green)),
        Text::raw(" "),
        Text::styled("rainbow", Style::default().fg(Color::Blue)),
        Text::raw(".\nOh and if you didn't "),
        Text::styled("notice", Style::default().modifier(Modifier::ITALIC)),
        Text::raw(" you can "),
        Text::styled("automatically", Style::default().modifier(Modifier::BOLD)),
        Text::raw(" "),
        Text::styled("wrap", Style::default().modifier(Modifier::REVERSED)),
        Text::raw(" your "),
        Text::styled("text", Style::default().modifier(Modifier::UNDERLINED)),
        Text::raw(".\nOne more thing is that it should display unicode characters: 10€")
    ];
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Footer")
        .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::BOLD));
    let paragraph = Paragraph::new(text.iter())
        .block(block);
        //.wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

