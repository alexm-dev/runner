use crate::app::{AppState, KeypressResult};
use crate::utils::parse_color;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::{io, time::Duration};

pub fn run_terminal(app: &mut AppState) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(&mut stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = event_loop(&mut terminal, app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn keycode_to_str(key: &KeyCode) -> &'static str {
    match key {
        KeyCode::Char('j') => "j",
        KeyCode::Char('k') => "k",
        KeyCode::Char('h') => "h",
        KeyCode::Char('l') => "l",
        KeyCode::Char('q') => "q",
        KeyCode::Enter | KeyCode::Char('\n') | KeyCode::Char('\r') => "Enter",
        KeyCode::Left => "Left Arrow",
        KeyCode::Right => "Right Arrow",
        KeyCode::Down => "Down Arrow",
        KeyCode::Up => "Up Arrow",
        KeyCode::Esc => "Esc",
        KeyCode::Backspace => "Backspace",
        _ => "",
    }
}

fn layout_chunks(size: Rect, app: &AppState) -> Vec<Rect> {
    let cfg = app.config().display();
    let mut constraints = Vec::new();
    let show_sep = cfg.separators() && !cfg.is_split();

    if cfg.origin() {
        constraints.push(Constraint::Percentage(cfg.origin_ratio()));
        if show_sep {
            constraints.push(Constraint::Length(1));
        }
    }

    constraints.push(Constraint::Percentage(cfg.main_ratio()));

    if cfg.preview() {
        if show_sep {
            constraints.push(Constraint::Length(1));
        }
        constraints.push(Constraint::Percentage(cfg.preview_ratio()));
    }

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(size)
        .to_vec()
}

pub fn draw_separator(frame: &mut Frame, area: Rect, color: Color) {
    let separator = Block::default()
        .borders(Borders::LEFT)
        .border_style(Style::default().fg(color));
    frame.render_widget(separator, area);
}

fn draw_main_pane(
    frame: &mut Frame,
    area: Rect,
    app: &AppState,
    accent_color: Color,
    entry_color: Color,
    highlight_symbol: &str,
    block: Block,
) {
    let show_marker = app.config().display().dir_marker();
    let items: Vec<ListItem> = app
        .visible_entries()
        .iter()
        .map(|e| {
            let text = if e.is_dir() && show_marker {
                e.display_name()
            } else {
                e.name_str()
            };
            ListItem::new(text).style(Style::default().fg(entry_color))
        })
        .collect();

    let mut state = ListState::default();
    if app.has_visible_entries() {
        state.select(app.visible_selected());
    }

    let padding = app.config().display().scroll_padding();

    frame.render_stateful_widget(
        List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .fg(accent_color)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(highlight_symbol)
            .scroll_padding(padding),
        area,
        &mut state,
    );
}

fn draw_preview_pane(frame: &mut Frame, area: Rect, lines: &[String], block: Block, color: Color) {
    let text: Vec<Line> = lines
        .iter()
        .map(|s| Line::from(s.as_str()).style(Style::default().fg(color)))
        .collect();

    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

fn draw_origin_pane(frame: &mut Frame, area: Rect, lines: &[String], block: Block, color: Color) {
    let text: Vec<Line> = lines
        .iter()
        .map(|s| Line::from(s.as_str()).style(Style::default().fg(color)))
        .collect();

    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);
}

fn render_ui(frame: &mut Frame, app: &AppState) {
    let mut root_area = frame.area();
    let cfg = app.config();
    let display_cfg = cfg.display();
    let theme_cfg = cfg.theme();

    // 0. PRE-CALCULATE COLORS & SHARED DATA
    // let bg_color = parse_color(theme_cfg.background());
    let accent_color = parse_color(theme_cfg.accent());
    let selection_color = parse_color(theme_cfg.selection());
    let parent_entry_color = parse_color(theme_cfg.origin());
    let preview_entry_color = parse_color(theme_cfg.preview());
    let entry_color = parse_color(theme_cfg.entry());
    let separator_color = parse_color(theme_cfg.separator());
    let show_separators = display_cfg.separators() && !display_cfg.is_split();
    let path_str = app.current_dir().to_string_lossy();

    // 1. HANDLE OUTER BORDER & TOP PATH
    if display_cfg.is_unified() {
        let mut outer_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(accent_color));
        if display_cfg.titles() {
            outer_block = outer_block.title(format!(" {} ", path_str));
        }
        frame.render_widget(outer_block, root_area);
        root_area = Block::default().borders(Borders::ALL).inner(root_area);
    } else {
        let header_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(root_area);
        let header =
            Paragraph::new(format!("   {} ", path_str)).style(Style::default().fg(accent_color));
        frame.render_widget(header, header_layout[0]);
        root_area = header_layout[1];
    }

    // 2. DEFINE PANE BLOCK GENERATOR
    let use_individual_borders = display_cfg.is_split();
    let get_pane_block = |title: &str| {
        let mut b = Block::default();
        if use_individual_borders {
            b = b
                .borders(Borders::ALL)
                .border_style(Style::default().fg(accent_color));
            if display_cfg.titles() {
                b = b.title(format!(" {} ", title));
            }
        }
        b
    };

    // 3. RENDER PANES
    let chunks = layout_chunks(root_area, app);
    let mut pane_idx = 0;

    // ORIGIN PANE
    if display_cfg.origin() && pane_idx < chunks.len() {
        draw_origin_pane(
            frame,
            chunks[pane_idx],
            &app.parent_content,
            get_pane_block("Parent"),
            parent_entry_color,
        );
        pane_idx += 1;

        if show_separators && pane_idx < chunks.len() {
            let sep_rect = Rect {
                x: chunks[pane_idx].x,
                y: root_area.y,
                width: 1,
                height: root_area.height,
            };
            draw_separator(frame, sep_rect, separator_color);
            pane_idx += 1;
        }
    }

    // MAIN PANE
    if pane_idx < chunks.len() {
        let symbol = if display_cfg.selection_marker() {
            theme_cfg.selection_icon()
        } else {
            ""
        };
        draw_main_pane(
            frame,
            chunks[pane_idx],
            app,
            selection_color,
            entry_color,
            symbol,
            get_pane_block("Files"),
        );
        pane_idx += 1;

        if show_separators && display_cfg.preview() && pane_idx < chunks.len() {
            let sep_rect = Rect {
                x: chunks[pane_idx].x,
                y: root_area.y,
                width: 1,
                height: root_area.height,
            };
            draw_separator(frame, sep_rect, separator_color);
            pane_idx += 1;
        }
    }

    // PREVIEW PANE
    if display_cfg.preview() && pane_idx < chunks.len() {
        draw_preview_pane(
            frame,
            chunks[pane_idx],
            &app.preview_content,
            get_pane_block("Preview"),
            preview_entry_color,
        );
    }
}

fn event_loop<B: Backend>(terminal: &mut Terminal<B>, app: &mut AppState) -> io::Result<()> {
    let mut should_render = true;
    loop {
        if should_render {
            terminal.draw(|frame| render_ui(frame, app))?;
            should_render = false;
        }

        if app.tick() {
            should_render = true;
            continue;
        }

        // INPUT HANDLING
        if event::poll(Duration::from_millis(10))? {
            match event::read()? {
                // 1. Handle Keys
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    let key_str = keycode_to_str(&key_event.code);
                    let result = app.handle_keypress(key_str);

                    should_render = true;

                    if let KeypressResult::Quit = result {
                        break;
                    }
                    if let KeypressResult::OpenedEditor = result {
                        terminal.clear()?;
                    }
                    continue;
                }

                // 2. Handle Resizes Directly
                Event::Resize(_, _) => {
                    should_render = true;
                    continue;
                }
                _ => {}
            }
        }
    }
    Ok(())
}
