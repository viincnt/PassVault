use tui_big_text::{ BigText, PixelSize };
use ratatui::{
    Frame,
    layout::{ Alignment, Constraint, Direction, Layout, Margin, Rect },
    style::{ Modifier, Style },
    text::{ Line, Span },
    widgets::{ Block, Borders, Paragraph },
};
use crate::ui::app::App;
use super::{ draw_header, draw_help, outer_layout, theme };

pub fn draw(frame: &mut Frame, app: &App) {
    let areas = outer_layout(frame.area());
    let title = if app.copied { "Copied to clipboard!" } else { &app.result_title };
    draw_header(frame, areas[0], title);
    draw_help(
        frame,
        areas[2],
        &[
            ("c", "copy"),
            ("↵ / Esc", "back to menu"),
        ]
    );

    match app.result_score {
        Some(score) => draw_evaluation(frame, areas[1], app, score),
        None => {
            let content = areas[1].inner(Margin { horizontal: 4, vertical: 1 });
            draw_password_box(frame, content, app);
        }
    }
}

fn draw_password_box(frame: &mut Frame, area: Rect, app: &App) {
    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(3), Constraint::Fill(1)])
        .split(area);

    let password = app.result_lines.first().map(String::as_str).unwrap_or("");

    frame.render_widget(
        Paragraph::new(password)
            .block(Block::default().borders(Borders::ALL).border_style(theme::accent()))
            .style(Style::default().fg(theme::WHITE).add_modifier(Modifier::BOLD)),
        v[1]
    );
}

fn draw_evaluation(frame: &mut Frame, area: Rect, app: &App, score: u8) {
    let strength_color = theme::score_color(score);
    let strength_label = match score {
        0..=7 => "Weak",
        8..=9 => "Fair",
        _ => "Strong",
    };

    let password = app.result_lines.first().map(String::as_str).unwrap_or("");
    let criteria: Vec<&str> = app.result_lines.iter().skip(1).map(String::as_str).collect();

    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(1), // password
            Constraint::Length(1), // spacer
            Constraint::Length(4), // BigText score number
            Constraint::Length(1), // bar + context
            Constraint::Length(1), // spacer
            Constraint::Length(3), // criteria row 1
            Constraint::Length(1), // spacer
            Constraint::Length(3), // criteria row 2
            Constraint::Length(1), // spacer
            Constraint::Length(3), // criteria row 3
            Constraint::Length(1), // spacer
            Constraint::Length(3), // criteria row 4
            Constraint::Fill(1),
        ])
        .split(area);

    // Password (centered, muted)
    frame.render_widget(
        Paragraph::new(password).alignment(Alignment::Center).style(theme::bold_white()),
        v[1]
    );

    // BigText: the score number — large and prominent
    let score_big = BigText::builder()
        .pixel_size(PixelSize::HalfHeight)
        .centered()
        .lines([
            Line::from(score.to_string()).style(
                Style::default().fg(strength_color).add_modifier(Modifier::BOLD)
            ),
        ])
        .build();
    frame.render_widget(score_big, v[3]);

    // Bar + "/ 10 · Label" on the line below the big number
    const BAR: usize = 16;
    let filled = ((score as usize) * BAR) / 10;
    let mut bar_line: Vec<Span> = (0..BAR)
        .map(|i|
            Span::styled(
                "█",
                Style::default().fg(if i < filled { strength_color } else { theme::SUBTLE })
            )
        )
        .collect();
    bar_line.extend([
        Span::styled("  / 10  ", theme::muted()),
        Span::styled(
            strength_label,
            Style::default().fg(strength_color).add_modifier(Modifier::BOLD)
        ),
    ]);
    frame.render_widget(Paragraph::new(Line::from(bar_line)).alignment(Alignment::Center), v[4]);

    // Criteria grid — three full rows of 3 + one centered row for the 10th
    if criteria.len() >= 3 {
        draw_criteria_row(frame, v[6], &criteria[..3]);
    }
    if criteria.len() >= 6 {
        draw_criteria_row(frame, v[8], &criteria[3..6]);
    }
    if criteria.len() >= 9 {
        draw_criteria_row(frame, v[10], &criteria[6..9]);
    }
    if criteria.len() >= 10 {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Fill(1), Constraint::Fill(1), Constraint::Fill(1)])
            .split(v[12]);
        let text = criteria[9];
        let is_pass = text.starts_with('✓');
        let icon_len = text
            .chars()
            .next()
            .map(|c| c.len_utf8())
            .unwrap_or(0);
        let (icon, rest) = text.split_at(icon_len);
        let icon_color = if is_pass { theme::SUCCESS } else { theme::ERR };
        let border_style = if is_pass { theme::subtle() } else { Style::default().fg(theme::ERR) };
        frame.render_widget(
            Paragraph::new(
                Line::from(
                    vec![
                        Span::styled(
                            icon,
                            Style::default().fg(icon_color).add_modifier(Modifier::BOLD)
                        ),
                        Span::styled(rest, theme::bright())
                    ]
                )
            )
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).border_style(border_style)),
            cols[1]
        );
    }
}

fn draw_criteria_row(frame: &mut Frame, area: Rect, items: &[&str]) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(2),
            Constraint::Fill(1),
        ])
        .split(area);

    let boxes = [cols[0], cols[2], cols[4]];

    for (i, text) in items.iter().enumerate() {
        let is_pass = text.starts_with('✓');
        let icon_len = text
            .chars()
            .next()
            .map(|c| c.len_utf8())
            .unwrap_or(0);
        let (icon, rest) = text.split_at(icon_len);

        let icon_color = if is_pass { theme::SUCCESS } else { theme::ERR };
        let border_style = if is_pass { theme::subtle() } else { Style::default().fg(theme::ERR) };

        frame.render_widget(
            Paragraph::new(
                Line::from(
                    vec![
                        Span::styled(
                            icon,
                            Style::default().fg(icon_color).add_modifier(Modifier::BOLD)
                        ),
                        Span::styled(rest, theme::bright())
                    ]
                )
            )
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).border_style(border_style)),
            boxes[i]
        );
    }
}
