mod modules;
mod ui;

fn main() {
    let mut terminal = ratatui::init();
    ui::App::new().run(&mut terminal).expect("failed to run app");
    ratatui::restore();
}
