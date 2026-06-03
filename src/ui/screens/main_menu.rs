use tui_big_text::{ BigText, PixelSize };
use ratatui::{
    Frame,
    layout::{ Alignment, Constraint, Direction, Layout },
    text::Line,
    widgets::Paragraph,
};
use crate::ui::app::App;
use super::theme;

const ITEMS: &[(&str, &str)] = &[
    ("Generate", "Create new secure passwords"),
    ("Strengthen", "Transform a weak password"),
    ("Evaluate", "Score your password strength"),
    ("Exit", "Close the application"),
];

pub fn draw(frame: &mut Frame, app: &App) {
    let area = super::centered_area(frame.area());

    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1), // [0] top spacer
            Constraint::Length(4), // [1] logo
            Constraint::Length(1), // [2] subtitle
            Constraint::Length(2), // [3] gap
            Constraint::Length(3), // [4] toggle buttons
            Constraint::Length(1), // [5] description of selected item
            Constraint::Fill(1), // [6] bottom spacer
            Constraint::Length(2), // [7] help bar
        ])
        .split(area);

    let big_text = BigText::builder()
        .pixel_size(PixelSize::HalfHeight)
        .centered()
        .lines([Line::from("PassVault").style(theme::bold_accent())])
        .build();
    frame.render_widget(big_text, outer[1]);

    frame.render_widget(
        Paragraph::new("Secure Password Toolkit")
            .alignment(Alignment::Center)
            .style(theme::muted()),
        outer[2]
    );

    super::draw_buttons(frame, outer[4], ITEMS, app.menu_cursor);

    if let Some((_, desc)) = ITEMS.get(app.menu_cursor) {
        frame.render_widget(
            Paragraph::new(*desc).alignment(Alignment::Center).style(theme::muted()),
            outer[5]
        );
    }

    super::draw_help(
        frame,
        outer[7],
        &[
            ("← →", "navigate"),
            ("↵", "select"),
            ("q", "quit"),
        ]
    );
}
