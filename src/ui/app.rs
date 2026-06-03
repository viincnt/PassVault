use std::{ io, time::Duration };
use crossterm::event::{ self, Event, KeyCode, KeyEventKind };
use arboard::Clipboard;
use ratatui::{ backend::Backend, Terminal };

use crate::modules::{ evaluator, generator, strengthener };
use super::screens;

pub enum Screen {
    MainMenu,
    Generator,
    GenWordCount,
    GenLength,
    GenSpecials,
    Strengthener,
    Evaluator,
    Result,
}

pub struct App {
    pub running: bool,
    pub screen: Screen,
    pub menu_cursor: usize,
    pub input: String,
    pub error: Option<String>,
    pub gen_memorable: bool,
    pub gen_word_count: usize,
    pub gen_length: usize,
    pub result_title: String,
    pub result_lines: Vec<String>,
    pub result_score: Option<u8>,
    pub copied: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            screen: Screen::MainMenu,
            menu_cursor: 0,
            input: String::new(),
            error: None,
            gen_memorable: false,
            gen_word_count: 0,
            gen_length: 0,
            result_title: String::new(),
            result_lines: Vec::new(),
            result_score: None,
            copied: false,
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> io::Result<()> {
        while self.running {
            terminal.draw(|frame| screens::draw(frame, self))?;
            if event::poll(Duration::from_millis(100))? {
                self.handle_event(event::read()?);
            }
        }
        Ok(())
    }

    fn handle_event(&mut self, event: Event) {
        let Event::Key(key) = event else {
            return;
        };
        if key.kind != KeyEventKind::Press {
            return;
        }
        self.error = None;
        self.copied = false;
        match self.screen {
            Screen::MainMenu => self.on_main_menu(key.code),
            Screen::Generator => self.on_generator_menu(key.code),
            Screen::GenWordCount => self.on_gen_word_count(key.code),
            Screen::GenLength => self.on_gen_length(key.code),
            Screen::GenSpecials => self.on_gen_specials(key.code),
            Screen::Strengthener => self.on_strengthener(key.code),
            Screen::Evaluator => self.on_evaluator(key.code),
            Screen::Result => self.on_result(key.code),
        }
    }

    fn on_main_menu(&mut self, key: KeyCode) {
        match key {
            KeyCode::Left => {
                self.menu_cursor = (self.menu_cursor + 3) % 4;
            }
            KeyCode::Right => {
                self.menu_cursor = (self.menu_cursor + 1) % 4;
            }
            KeyCode::Enter =>
                match self.menu_cursor {
                    0 => {
                        self.screen = Screen::Generator;
                        self.menu_cursor = 0;
                    }
                    1 => {
                        self.screen = Screen::Strengthener;
                        self.input.clear();
                    }
                    2 => {
                        self.screen = Screen::Evaluator;
                        self.input.clear();
                    }
                    _ => {
                        self.running = false;
                    }
                }
            KeyCode::Char('q') | KeyCode::Esc => {
                self.running = false;
            }
            _ => {}
        }
    }

    fn on_generator_menu(&mut self, key: KeyCode) {
        match key {
            KeyCode::Left | KeyCode::Right => {
                self.menu_cursor = if self.menu_cursor == 0 { 1 } else { 0 };
            }
            KeyCode::Enter =>
                match self.menu_cursor {
                    0 => {
                        self.gen_memorable = true;
                        self.screen = Screen::GenWordCount;
                        self.input.clear();
                    }
                    _ => {
                        self.gen_memorable = false;
                        self.screen = Screen::GenLength;
                        self.input.clear();
                    }
                }
            KeyCode::Esc => {
                self.screen = Screen::MainMenu;
                self.menu_cursor = 0;
            }
            _ => {}
        }
    }

    fn on_gen_word_count(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) if c.is_ascii_digit() => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Enter =>
                match self.input.trim().parse::<usize>() {
                    Ok(n) if (2..=10).contains(&n) => {
                        self.gen_word_count = n;
                        self.menu_cursor = 0;
                        self.screen = Screen::GenSpecials;
                    }
                    _ => {
                        self.error = Some("Enter a number between 2 and 10.".into());
                        self.input.clear();
                    }
                }
            KeyCode::Esc => {
                self.screen = Screen::Generator;
                self.input.clear();
                self.menu_cursor = 0;
            }
            _ => {}
        }
    }

    fn on_gen_length(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) if c.is_ascii_digit() => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Enter =>
                match self.input.trim().parse::<usize>() {
                    Ok(n) if (12..=64).contains(&n) => {
                        self.gen_length = n;
                        self.menu_cursor = 0;
                        self.screen = Screen::GenSpecials;
                    }
                    _ => {
                        self.error = Some("Enter a number between 12 and 64.".into());
                        self.input.clear();
                    }
                }
            KeyCode::Esc => {
                self.screen = Screen::Generator;
                self.input.clear();
                self.menu_cursor = 1;
            }
            _ => {}
        }
    }

    fn on_gen_specials(&mut self, key: KeyCode) {
        match key {
            KeyCode::Left | KeyCode::Right => {
                self.menu_cursor = if self.menu_cursor == 0 { 1 } else { 0 };
            }
            KeyCode::Enter => {
                self.generate_and_show(self.menu_cursor == 0);
            }
            KeyCode::Esc => {
                self.screen = if self.gen_memorable {
                    Screen::GenWordCount
                } else {
                    Screen::GenLength
                };
                self.input.clear();
            }
            _ => {}
        }
    }

    fn generate_and_show(&mut self, specials: bool) {
        let password = if self.gen_memorable {
            generator::generate_memorable(self.gen_word_count, specials)
        } else {
            generator::generate_random(self.gen_length, specials)
        };
        self.result_title = "Generated Password".into();
        self.result_lines = vec![password];
        self.result_score = None;
        self.screen = Screen::Result;
    }

    fn on_strengthener(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Enter => {
                if self.input.trim().is_empty() {
                    self.error = Some("Input cannot be empty.".into());
                    return;
                }
                let result = strengthener::strengthen(self.input.trim());
                self.result_title = "Strengthened Password".into();
                self.result_lines = vec![result];
                self.result_score = None;
                self.input.clear();
                self.screen = Screen::Result;
            }
            KeyCode::Esc => {
                self.screen = Screen::MainMenu;
                self.input.clear();
                self.menu_cursor = 1;
            }
            _ => {}
        }
    }

    fn on_evaluator(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Enter => {
                if self.input.trim().is_empty() {
                    self.error = Some("Input cannot be empty.".into());
                    return;
                }
                if self.input.trim().len() < 5 {
                    self.error = Some(
                        "Password too short — minimum 5 characters for evaluation.".into()
                    );
                    return;
                }
                let ev = evaluator::evaluate(self.input.trim());
                self.result_title = "Password Evaluation".into();
                self.result_score = Some(ev.score);
                self.result_lines = vec![
                    self.input.trim().to_string(),
                    format!("{} Lowercase", if ev.has_lower { "✓" } else { "✗" }),
                    format!("{} Uppercase", if ev.has_upper { "✓" } else { "✗" }),
                    format!("{} Numbers", if ev.has_number { "✓" } else { "✗" }),
                    format!("{} Symbols", if ev.has_symbol { "✓" } else { "✗" }),
                    format!("{} Min 8 chars", if ev.length >= 8 { "✓" } else { "✗" }),
                    format!("{} Min 12 chars", if ev.length >= 12 { "✓" } else { "✗" }),
                    format!("{} No 3x repeats", if ev.no_repeats { "✓" } else { "✗" }),
                    format!("{} No sequences", if ev.no_sequences { "✓" } else { "✗" }),
                    format!("{} Not common", if ev.not_common { "✓" } else { "✗" }),
                    format!("{} 16+ chars", if ev.very_long { "✓" } else { "✗" })
                ];
                self.input.clear();
                self.screen = Screen::Result;
            }
            KeyCode::Esc => {
                self.screen = Screen::MainMenu;
                self.input.clear();
                self.menu_cursor = 2;
            }
            _ => {}
        }
    }

    fn on_result(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('c') => {
                if let Some(password) = self.result_lines.first() {
                    if let Ok(mut clipboard) = Clipboard::new() {
                        if clipboard.set_text(password).is_ok() {
                            self.copied = true;
                        }
                    }
                }
            }
            KeyCode::Enter | KeyCode::Esc | KeyCode::Char(' ') => {
                self.screen = Screen::MainMenu;
                self.menu_cursor = 0;
            }
            _ => {}
        }
    }
}
