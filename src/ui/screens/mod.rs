pub mod theme;
mod generator;
mod input;
mod main_menu;
mod result;

use ratatui::{
    layout::{ Alignment, Constraint, Direction, Layout, Rect },
    text::{ Line, Span },
    widgets::{ Block, Borders, Paragraph },
    Frame,
};

use crate::ui::app::{ App, Screen };

pub fn centered_area(area: Rect) -> Rect {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(area)[1]
}

pub fn draw(frame: &mut Frame, app: &App) {
    match app.screen {
        Screen::MainMenu => main_menu::draw(frame, app),
        Screen::Generator => generator::draw(frame, app),
        Screen::GenWordCount => input::draw_word_count(frame, app),
        Screen::GenLength => input::draw_length(frame, app),
        Screen::GenSpecials => input::draw_specials(frame, app),
        Screen::Strengthener => input::draw_strengthener(frame, app),
        Screen::Evaluator => input::draw_evaluator(frame, app),
        Screen::Result => result::draw(frame, app),
    }
}

pub fn outer_layout(area: Rect) -> [Rect; 3] {
    let area = centered_area(area);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(1), Constraint::Length(2)])
        .split(area);
    [chunks[0], chunks[1], chunks[2]]
}

pub fn draw_choice(
    frame: &mut Frame,
    area: Rect,
    question: &str,
    subtitle: &str,
    options: &[(&str, &str)],
    selected: usize
) {
    let sub_h = u16::from(!subtitle.is_empty());

    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(1), // question
            Constraint::Length(sub_h), // subtitle
            Constraint::Length(1), // gap
            Constraint::Length(3), // buttons
            Constraint::Length(1), // description of selected option
            Constraint::Fill(1),
        ])
        .split(area);

    frame.render_widget(
        Paragraph::new(question).alignment(Alignment::Center).style(theme::bright()),
        v[1]
    );

    if !subtitle.is_empty() {
        frame.render_widget(
            Paragraph::new(subtitle).alignment(Alignment::Center).style(theme::muted()),
            v[2]
        );
    }

    draw_buttons(frame, v[4], options, selected);

    if let Some((_, desc)) = options.get(selected) {
        if !desc.is_empty() {
            frame.render_widget(
                Paragraph::new(*desc).alignment(Alignment::Center).style(theme::muted()),
                v[5]
            );
        }
    }
}

pub fn draw_buttons(frame: &mut Frame, area: Rect, options: &[(&str, &str)], selected: usize) {
    const GAP: u16 = 3;
    const PAD: u16 = 2;

    let btn_widths: Vec<u16> = options
        .iter()
        .map(|(label, _)| (label.chars().count() as u16) + PAD * 2 + 2)
        .collect();

    let total_width: u16 =
        btn_widths.iter().sum::<u16>() + GAP * (options.len().saturating_sub(1) as u16);

    let center = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Length(total_width), Constraint::Fill(1)])
        .split(area)[1];

    let mut constraints: Vec<Constraint> = Vec::new();
    for (i, &w) in btn_widths.iter().enumerate() {
        constraints.push(Constraint::Length(w));
        if i < options.len() - 1 {
            constraints.push(Constraint::Length(GAP));
        }
    }

    let btn_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(center);

    for (i, (label, _)) in options.iter().enumerate() {
        let btn_rect = btn_areas[i * 2];
        let style = if i == selected { theme::bold_accent() } else { theme::muted() };

        frame.render_widget(
            Paragraph::new(format!("  {label}  "))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).border_style(style))
                .style(style),
            btn_rect
        );
    }
}

pub fn draw_header(frame: &mut Frame, area: Rect, breadcrumb: &str) {
    frame.render_widget(
        Paragraph::new(
            Line::from(
                vec![
                    Span::raw(" "),
                    Span::styled("◆", theme::bold_accent()),
                    Span::raw(" "),
                    Span::styled("PassVault", theme::bold_white()),
                    Span::styled("  ›  ", theme::muted()),
                    Span::styled(breadcrumb, theme::bright())
                ]
            )
        ).block(Block::default().borders(Borders::BOTTOM).border_style(theme::subtle())),
        area
    );
}

pub fn draw_help(frame: &mut Frame, area: Rect, bindings: &[(&str, &str)]) {
    let mut spans: Vec<Span> = Vec::new();
    for (i, (key, desc)) in bindings.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled("    ", theme::muted()));
        }
        spans.push(Span::styled(format!("[{key}]"), theme::accent()));
        spans.push(Span::raw(" "));
        spans.push(Span::styled(*desc, theme::muted()));
    }
    frame.render_widget(
        Paragraph::new(Line::from(spans))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::TOP).border_style(theme::subtle())),
        area
    );
}
