use ratatui::style::{ Color, Modifier, Style };

pub const ACCENT: Color = Color::Rgb(97, 175, 239);
pub const SUCCESS: Color = Color::Rgb(152, 195, 121);
pub const WARNING: Color = Color::Rgb(229, 192, 123);
pub const ERR: Color = Color::Rgb(224, 108, 117);
pub const MUTED: Color = Color::Rgb(92, 99, 112);
pub const SUBTLE: Color = Color::Rgb(55, 62, 77);
pub const BRIGHT: Color = Color::Rgb(171, 178, 191);
pub const WHITE: Color = Color::Rgb(220, 223, 228);

pub fn accent() -> Style {
    Style::default().fg(ACCENT)
}
pub fn err() -> Style {
    Style::default().fg(ERR)
}
pub fn muted() -> Style {
    Style::default().fg(MUTED)
}
pub fn subtle() -> Style {
    Style::default().fg(SUBTLE)
}
pub fn bright() -> Style {
    Style::default().fg(BRIGHT)
}
pub fn bold_white() -> Style {
    Style::default().fg(WHITE).add_modifier(Modifier::BOLD)
}
pub fn bold_accent() -> Style {
    Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
}

pub fn input_border() -> Style {
    accent()
}
pub fn input_border_err() -> Style {
    err()
}

pub fn score_color(score: u8) -> Color {
    match score {
        0..=4 => ERR,
        5 => WARNING,
        _ => SUCCESS,
    }
}
