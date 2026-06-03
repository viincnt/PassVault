use ratatui::{
    Frame,
    layout::{ Constraint, Direction, Layout, Margin, Position },
    widgets::{ Block, Borders, Paragraph },
};
use crate::ui::app::App;
use super::{ draw_header, draw_help, outer_layout, theme };

fn draw_input(
    frame: &mut Frame,
    breadcrumb: &str,
    label: &str,
    hint: &str,
    input: &str,
    error: Option<&str>,
    bindings: &[(&str, &str)]
) {
    let areas = outer_layout(frame.area());
    draw_header(frame, areas[0], breadcrumb);
    draw_help(frame, areas[2], bindings);

    let content = areas[1].inner(Margin { horizontal: 6, vertical: 0 });

    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .split(content);

    frame.render_widget(Paragraph::new(label).style(theme::bright()), v[1]);

    let box_style = if error.is_some() { theme::input_border_err() } else { theme::input_border() };

    let visible_width = v[2].width.saturating_sub(2) as usize;
    let char_count = input.chars().count();
    let display: String = if char_count > visible_width {
        input
            .chars()
            .skip(char_count - visible_width)
            .collect()
    } else {
        input.to_string()
    };

    frame.render_widget(
        Paragraph::new(display.as_str())
            .block(Block::default().borders(Borders::ALL).border_style(box_style))
            .style(theme::bold_white()),
        v[2]
    );

    frame.set_cursor_position(Position {
        x: v[2].x + 1 + (display.chars().count() as u16),
        y: v[2].y + 1,
    });

    if let Some(msg) = error {
        frame.render_widget(Paragraph::new(format!("⚠  {msg}")).style(theme::err()), v[3]);
    } else if !hint.is_empty() {
        frame.render_widget(Paragraph::new(hint).style(theme::muted()), v[3]);
    }
}

pub fn draw_specials(frame: &mut Frame, app: &App) {
    let areas = outer_layout(frame.area());
    draw_header(frame, areas[0], "Generate › Special Characters");
    draw_help(
        frame,
        areas[2],
        &[
            ("← →", "toggle"),
            ("↵", "confirm"),
            ("Esc", "back"),
        ]
    );
    super::draw_choice(
        frame,
        areas[1],
        "Include special characters?",
        "! @ # $ % ^ & *",
        &[
            ("Yes", ""),
            ("No", ""),
        ],
        app.menu_cursor
    );
}

const CONFIRM_BACK: &[(&str, &str)] = &[
    ("↵", "confirm"),
    ("Esc", "back"),
];

pub fn draw_word_count(frame: &mut Frame, app: &App) {
    draw_input(
        frame,
        "Generate › Memorable › Word Count",
        "Number of words:",
        "Range: 2 – 10",
        &app.input,
        app.error.as_deref(),
        CONFIRM_BACK
    );
}

pub fn draw_length(frame: &mut Frame, app: &App) {
    draw_input(
        frame,
        "Generate › Random › Length",
        "Password length:",
        "Range: 12 – 64",
        &app.input,
        app.error.as_deref(),
        CONFIRM_BACK
    );
}

pub fn draw_strengthener(frame: &mut Frame, app: &App) {
    draw_input(
        frame,
        "Strengthen Password",
        "Enter the password to strengthen:",
        "",
        &app.input,
        app.error.as_deref(),
        CONFIRM_BACK
    );
}

pub fn draw_evaluator(frame: &mut Frame, app: &App) {
    draw_input(
        frame,
        "Evaluate Password",
        "Enter the password to evaluate:",
        "",
        &app.input,
        app.error.as_deref(),
        CONFIRM_BACK
    );
}
