use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Content (list + details)
                Constraint::Length(3), // Footer
            ]
            .as_ref(),
        )
        .split(frame.area());

    // Header
    let header = Paragraph::new("SpaceX Data in the Terminal")
        .style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().title(" Aurora ").borders(Borders::ALL));
    frame.render_widget(header, chunks[0]);

    // Split content area into left and right
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(30), // Left: Launch list
                Constraint::Percentage(70), // Right: Details
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    // Left: List of launches
    let items: Vec<_> = app
        .spacex_data
        .iter()
        .enumerate()
        .map(|(index, launch)| {
            let status_color = match launch.success {
                Some(true) => Color::Green,
                Some(false) => Color::Red,
                None => Color::Yellow,
            };
            ListItem::new(Span::styled(
                launch.name.clone(),
                Style::default()
                    .fg(status_color)
                    .add_modifier(if app.selected_index == index {
                        Modifier::BOLD
                    } else {
                        Modifier::empty()
                    }),
            ))
        })
        .collect();
    let list = List::new(items)
        .block(Block::default().title(" Launches ").borders(Borders::ALL))
        .highlight_symbol(">>")
        .highlight_style(Style::default().fg(Color::Yellow));
    frame.render_stateful_widget(list, content_chunks[0], &mut app.list_state);

    // Right: Launch details
    let details = if let Some(launch) = app.spacex_data.get(app.selected_index) {
        vec![
            format!("Mission: {}", launch.name),
            format!("Date: {}", launch.date_utc),
            format!(
                "Success: {}",
                match launch.success {
                    Some(true) => "Yes",
                    Some(false) => "No",
                    None => "Unknown",
                }
            ),
            format!("Rocket: {}", launch.rocket),
            format!("Launchpad: {}", launch.launchpad),
        ]
        .join("\n")
    } else {
        "No launch selected.".to_string()
    };

    let paragraph = Paragraph::new(details)
        .style(Style::default().fg(Color::White))
        .block(Block::default().title(" Details ").borders(Borders::ALL));
    frame.render_widget(paragraph, content_chunks[1]);

    // Footer
    let footer = Paragraph::new("[q] Quit | [Up/Down] Navigate | [/] Search")
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, chunks[2]);
}
