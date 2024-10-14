use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use super::app::{App, AppState, Log};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    // Create a layout with a horizontal 50/50 split
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(frame.area());

    render_menu(app, frame, chunks[0]);
    render_logs(app, frame, chunks[1]);
}

fn render_menu(app: &mut App, frame: &mut Frame, area: Rect) {
    match app.state {
        AppState::SlaveList => render_slave_list(app, frame, area),
        AppState::SlaveMenu => render_slave_menu(app, frame, area),
    }
}

fn render_slave_list(app: &mut App, frame: &mut Frame, area: Rect) {
    let slaves: Vec<ListItem> = app
        .slaves
        .iter()
        .map(|slave| ListItem::new(Span::raw(format!("{}", slave.to_string()))))
        .collect();

    let slaves_list = List::new(slaves)
        .block(
            Block::default()
                .title("Connected Slaves")
                .borders(Borders::ALL),
        )
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Yellow),
        );

    frame.render_stateful_widget(slaves_list, area, &mut app.slave_list_state);
}

fn render_slave_menu(app: &mut App, frame: &mut Frame, area: Rect) {
    let options = vec![
        ListItem::new("1. Take Screenshot"),
        ListItem::new("2. Show System Info"),
    ];

    let options_list = List::new(options)
        .block(Block::default().title("Slave Menu").borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Yellow),
        );

    frame.render_stateful_widget(options_list, area, &mut app.slave_menu_state);
}

fn render_logs(app: &App, frame: &mut Frame, area: Rect) {
    let logs: Vec<Line> = app
        .logs
        .iter()
        .map(|log| {
            let (content, style) = match log {
                Log::Result(msg) => (msg, Style::default().fg(Color::Green)),
                Log::Info(msg) => (msg, Style::default().fg(Color::Yellow)),
                Log::Error(msg) => (msg, Style::default().fg(Color::Red)),
            };
            Line::from(vec![Span::styled(content, style)])
        })
        .collect();

    let log_widget = Paragraph::new(Text::from(logs))
        .block(Block::default().title("Logs").borders(Borders::ALL))
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(log_widget, area);
}
