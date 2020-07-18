use tui::{
    Frame,
    symbols,
    backend::{Backend},
    style::{Color, Modifier, Style},
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{
        Text,
        Axis, Block, Borders, Chart, Dataset, Paragraph,
    },
};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use num_format::{Locale, ToFormattedString};
use crate::app::{InputMode, App};
use crate::asset;
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
    draw_text(f, chunks[1], &app);
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
        let mut data =  Vec::<(f64, f64)>::new();
        data.push((1.0, 217.68));
        data.push((2.0, 222.49));
        data.push((3.0, 217.19));

        match &app.company {
            None => { 
                let api_key = app.config.get("api_key").expect("No API Key in config");
                match asset::get_equity(api_key, "TSLA") {
                    Err(err) => println!("Error getting {}",err),
                    Ok(company) => app.company = Some(company),
                }
            },
            Some(_) => {}
        }        
        
        let datasets = [
            Dataset::default()
                .name("data2")
                .marker(symbols::Marker::Dot)
                .style(Style::default().fg(Color::Cyan))
                .data(&data),
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
                    .bounds([200.0, 250.0])
                    .labels(&["0.0", "125", "250"]),
            )
            .datasets(&datasets);
        f.render_widget(chart, chunks[1]);
    }
}

fn draw_text<B>(f: &mut Frame<B>, area: Rect, app: &App)
where
    B: Backend,
{
    let mut text = vec![];
    
    match &app.company
    {
        Some(company) => assemble_company_info(company, &mut text),
        None => {}
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Footer")
        .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::BOLD));
    let paragraph = Paragraph::new(text.iter())
        .block(block);
        //.wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn assemble_company_info<'a,'b>(company: &'a asset::Company, text: &'b mut Vec<Text<'a>> ){
    
    text.push(Text::styled("Name: ", Style::default().fg(Color::Blue)));
    text.push(Text::raw(format!("{}",company.name)));
    text.push(Text::styled("\nTicker: ", Style::default().fg(Color::Blue)));
    text.push(Text::raw(format!("{}",company.ticker)));
    text.push(Text::styled("\nCountry: ", Style::default().fg(Color::Blue)));
    text.push(Text::raw(format!("{}",company.country)));
    text.push(Text::styled("\nExchange: ", Style::default().fg(Color::Blue)));
    text.push(Text::raw(format!("{}",company.exchange)));
    text.push(Text::styled("\nMarket Cap: ", Style::default().fg(Color::Blue)));
    let market_cap = company.market_capitalization as u64;    
    text.push(Text::raw(format!("${}", market_cap.to_formatted_string(&Locale::en))));
    text.push(Text::styled("\nCurrency: ", Style::default().fg(Color::Blue)));
    text.push(Text::raw(format!("{}",company.currency)));
    text.push(Text::styled("\nIndustry: ", Style::default().fg(Color::Blue)));
    text.push(Text::raw(format!("{}",company.industry)));

}

