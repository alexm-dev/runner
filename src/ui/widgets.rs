use crate::app::AppState;
use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    widgets::{Block, Borders},
};

pub fn get_pane_block(title: &str, app: &AppState) -> Block<'static> {
    let mut b = Block::default();
    if app.config().display().is_split() {
        b = b
            .borders(Borders::ALL)
            .border_style(app.config().theme().accent().as_style());
        if app.config().display().titles() {
            b = b.title(title.to_string());
        }
    }
    b
}

pub fn draw_separator(frame: &mut Frame, area: Rect, style: Style) {
    frame.render_widget(
        Block::default().borders(Borders::LEFT).border_style(style),
        area,
    );
}

pub fn keycode_to_str(key: &KeyCode) -> &'static str {
    match key {
        KeyCode::Char('j') => "j",
        KeyCode::Char('k') => "k",
        KeyCode::Char('h') => "h",
        KeyCode::Char('l') => "l",
        KeyCode::Char('q') => "q",
        KeyCode::Enter => "Enter",
        KeyCode::Up => "Up Arrow",
        KeyCode::Down => "Down Arrow",
        KeyCode::Left => "Left Arrow",
        KeyCode::Right => "Right Arrow",
        KeyCode::Esc => "Esc",
        KeyCode::Backspace => "Backspace",
        _ => "",
    }
}
