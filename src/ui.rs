use crate::app::{App, InputMode};
use crate::asset;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use num_format::{Locale, ToFormattedString};
#[allow(unused_imports)]
use std::{sync::mpsc::Receiver, time::Duration};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    widgets::{Axis, Block, Borders, Chart, Dataset, Paragraph, Text},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App, wsrx: &Receiver<(String, f64)>) {
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Percentage(3),
                Constraint::Percentage(7),
                Constraint::Percentage(90),
            ]
            .as_ref(),
        )
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
        0 => draw_first_tab(f, app, chunks[2], &wsrx),
        _ => {}
    };
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect, wsrx: &Receiver<(String, f64)>)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(60), Constraint::Length(7)].as_ref())
        .split(area);
    //draw_gauges(f, app, chunks[0]);
    draw_charts(f, app, chunks[0]);
    draw_text(f, chunks[1], app, &wsrx);
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
        match &app.company {
            None => {
                let api_key = app.config.get("api_key").expect("No API Key in config");
                match asset::get_equity(api_key, &app.symbol) {
                    Err(err) => println!("Error getting {}", err),
                    Ok(mut company) => {
                        match asset::get_price_history(api_key, &app.symbol, "D") {
                            Ok(res) => company.prices = res,
                            _ => {}
                        }
                        app.company = Some(company);
                    }
                }
            }
            Some(_) => {}
        }
        let mut data = Vec::<(f64, f64)>::new();
        data.push((1.0, 217.68));
        data.push((2.0, 222.49));
        data.push((3.0, 217.19));

        let mut min = 0.0;
        let mut max = 300.0;
        if let Some(company) = &app.company {
            //info!("{:#?}",company);
            data = label_data(&company.prices.close);
            let price_range = get_range(&company.prices.close);
            min = price_range.0;
            max = price_range.1;
        }
        let x_labels = [
            format!("{}", 0),
            format!("{}", data.len() / 10 * 1),
            format!("{}", data.len() / 10 * 2),
            format!("{}", data.len() / 10 * 3),
            format!("{}", data.len() / 10 * 4),
            format!("{}", data.len() / 10 * 5),
            format!("{}", data.len() / 10 * 6),
            format!("{}", data.len() / 10 * 7),
            format!("{}", data.len() / 10 * 8),
            format!("{}", data.len() / 10 * 9),
            format!("{}", data.len()),
        ];

        let datasets = [
            Dataset::default()
                .name("data2")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Cyan))
                .data(&data),
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
                    .bounds([0.0, data.len() as f64])
                    .labels(&x_labels),
            )
            .y_axis(
                Axis::default()
                    .title("Y Axis")
                    .style(Style::default().fg(Color::Gray))
                    .labels_style(Style::default().modifier(Modifier::ITALIC))
                    .bounds([min, max])
                    .labels(&["min", "mid", "max"]),
            )
            .datasets(&datasets);
        f.render_widget(chart, chunks[1]);
    }
}

fn draw_text<B>(f: &mut Frame<B>, area: Rect, app: &mut App, wsrx: &Receiver<(String, f64)>)
where
    B: Backend,
{
    let mut text = vec![];

    match &mut app.company {
        Some(company) => assemble_company_info(company, &mut text, wsrx),
        None => {}
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Footer")
        .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::BOLD));
    let paragraph = Paragraph::new(text.iter()).block(block);
    //.wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn assemble_company_info<'a, 'b>(
    company: &'a mut asset::Company,
    text: &'b mut Vec<Text<'a>>,
    wsrx: &Receiver<(String, f64)>,
) {
    text.push(Text::styled("Name: ", Style::default().fg(Color::Blue)));
    text.push(Text::raw(format!("{}", company.name)));
    text.push(Text::styled("\nPrice: ", Style::default().fg(Color::Blue)));
    if company.prices.close.len() > 0 {
        text.push(Text::raw(format!(
            "{}",
            company.prices.close[company.prices.close.len() - 1]
        )));
    }
    text.push(Text::styled("\nTicker: ", Style::default().fg(Color::Blue)));
    text.push(Text::raw(format!("{}", company.ticker)));
    text.push(Text::styled(
        "\nCountry: ",
        Style::default().fg(Color::Blue),
    ));
    text.push(Text::raw(format!("{}", company.country)));
    text.push(Text::styled(
        "\nExchange: ",
        Style::default().fg(Color::Blue),
    ));
    text.push(Text::raw(format!("{}", company.exchange)));
    text.push(Text::styled(
        "\nMarket Cap: ",
        Style::default().fg(Color::Blue),
    ));
    let market_cap = company.market_capitalization as u64;
    text.push(Text::raw(format!(
        "${}",
        market_cap.to_formatted_string(&Locale::en)
    )));
    text.push(Text::styled(
        "\nCurrency: ",
        Style::default().fg(Color::Blue),
    ));
    text.push(Text::raw(format!("{}", company.currency)));
    text.push(Text::styled(
        "\nIndustry: ",
        Style::default().fg(Color::Blue),
    ));
    text.push(Text::raw(format!("{}", company.industry)));

    live_price_text(text, company, wsrx);

}

fn live_price_text(text: &mut Vec<Text>, company: &mut asset::Company, wsrx: &Receiver<(String, f64)>) {
    let d = Duration::from_millis(0);
    let res = wsrx.recv();// recv_timeout(d);
    match res {
        Ok(msg) => {
            let (symbol, price) = msg;
            text.push(Text::styled(
                format!("\nLive - {} ", symbol),
                Style::default().fg(Color::Blue),
            ));
            if price > company.prices.live_price {                
                text.push(Text::styled(
                    format!("▲ {}", price),
                    Style::default().fg(Color::Green),
                ));
                company.prices.movement_indicator = String::from("▲");
            } else {
                text.push(Text::styled(
                    format!("▼ {}", price),
                    Style::default().fg(Color::Red),
                ));
                company.prices.movement_indicator = String::from("▼");
            }
            company.prices.live_price = price;
        }
        Err(e) => {
            text.push(Text::styled(
                format!("\nLive - {} ", company.ticker),
                Style::default().fg(Color::Blue),
            ));
            let mut color = Color::Red;
            if company.prices.movement_indicator == "▲" {
                color = Color::Green;
            }

            text.push(Text::styled(
                format!("{} {}", company.prices.movement_indicator, company.prices.live_price),
                Style::default().fg(color),
            ));
            //error!("{}", e);
        }
    }
}

fn label_data(prices: &Vec<f64>) -> Vec<(f64, f64)> {
    let mut data: Vec<(f64, f64)> = Vec::new();

    for (i, price) in prices.iter().enumerate() {
        data.push((i as f64, *price));
    }
    data
}

fn get_range(prices: &Vec<f64>) -> (f64, f64) {
    let mut min = std::u32::MAX as f64;
    let mut max = 0.0;

    for price in prices {
        if *price > max {
            max = *price;
        };
        if *price < min {
            min = *price;
        }
    }

    (min, max)
}
