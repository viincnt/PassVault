use ratatui::Frame;
use crate::ui::app::App;
use super::{ draw_choice, draw_header, draw_help, outer_layout };

const OPTIONS: &[(&str, &str)] = &[
    ("Memorable", "Word-based, easier to remember"),
    ("Random", "Character-based, maximum entropy"),
];

pub fn draw(frame: &mut Frame, app: &App) {
    let areas = outer_layout(frame.area());
    draw_header(frame, areas[0], "Generate Password");
    draw_help(
        frame,
        areas[2],
        &[
            ("← →", "toggle"),
            ("↵", "select"),
            ("Esc", "back"),
        ]
    );
    draw_choice(frame, areas[1], "Password Type", "", OPTIONS, app.menu_cursor);
}
