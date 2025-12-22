pub mod panes;
pub mod widgets;

use self::panes::PaneContext;
use crate::{app::AppState, ui::panes::PaneStyles};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn render(frame: &mut Frame, app: &mut AppState) {
    let mut root_area = frame.area();
    {
        let chunks = layout_chunks(root_area, app);
        let mut m = crate::app::LayoutMetrics::default();
        let display_cfg = app.config().display();

        let mut c_idx = 0;
        let has_sep = display_cfg.separators() && !display_cfg.is_split();

        // Helper to determine inner space available for text
        let get_inner = |rect: ratatui::layout::Rect| {
            let w = if display_cfg.is_split() || display_cfg.is_unified() {
                rect.width.saturating_sub(2)
            } else {
                rect.width
            };
            let h = rect.height.saturating_sub(2);
            (w as usize, h as usize)
        };

        if display_cfg.origin() && c_idx < chunks.len() {
            m.parent_width = get_inner(chunks[c_idx]).0;
            c_idx += if has_sep { 2 } else { 1 };
        }

        if c_idx < chunks.len() {
            m.main_width = get_inner(chunks[c_idx]).0;
            c_idx += if has_sep && display_cfg.preview() {
                2
            } else {
                1
            };
        }

        if display_cfg.preview() && c_idx < chunks.len() {
            let (w, h) = get_inner(chunks[c_idx]);
            m.preview_width = w;
            m.preview_height = h;
        }

        app.metrics = m;
    }

    let cfg = app.config();
    let display_cfg = cfg.display();
    let theme_cfg = cfg.theme();

    let accent_style = theme_cfg.accent().as_style();
    let selection_style = theme_cfg.selection().as_style();
    let path_str = app.nav.current_dir().to_string_lossy();
    let path_style = theme_cfg.path().as_style();

    // Root Border / Header Logic
    if display_cfg.is_unified() {
        let mut outer_block = Block::default()
            .borders(Borders::ALL)
            .border_style(accent_style);
        if display_cfg.titles() {
            outer_block = outer_block.title(Line::from(vec![Span::styled(
                format!(" {} ", path_str),
                path_style,
            )]));
        }
        frame.render_widget(outer_block, root_area);
        root_area = Block::default().borders(Borders::ALL).inner(root_area);
    } else {
        let header_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(root_area);
        frame.render_widget(
            Paragraph::new(Line::from(vec![Span::styled(
                format!("   {} ", path_str),
                path_style,
            )])),
            header_layout[0],
        );
        root_area = header_layout[1];
    }

    // Render Panes
    let chunks = layout_chunks(root_area, app);
    let mut pane_idx = 0;
    let show_separators = display_cfg.separators() && !display_cfg.is_split();

    // ORIGIN PANE
    if display_cfg.origin() && pane_idx < chunks.len() {
        panes::draw_origin(
            frame,
            PaneContext {
                area: chunks[pane_idx],
                block: widgets::get_pane_block("Parent", app),
                accent_style,
                styles: PaneStyles {
                    item: theme_cfg.origin().as_style(),
                    dir: theme_cfg.directory().as_style(),
                    selection: selection_style,
                },
                highlight_symbol: "",
            },
            app.parent.entries(),
            app.parent.selected_idx(),
        );
        pane_idx += 1;
        if show_separators && pane_idx < chunks.len() {
            widgets::draw_separator(
                frame,
                Rect {
                    x: chunks[pane_idx].x,
                    y: root_area.y,
                    width: 1,
                    height: root_area.height,
                },
                theme_cfg.separator().as_style(),
            );
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

        let pane_style = PaneStyles {
            item: theme_cfg.entry().as_style(),
            dir: theme_cfg.directory().as_style(),
            selection: selection_style,
        };

        panes::draw_main(
            frame,
            app,
            PaneContext {
                area: chunks[pane_idx],
                block: widgets::get_pane_block("Files", app),
                accent_style,
                styles: pane_style,
                highlight_symbol: symbol,
            },
        );
        pane_idx += 1;
        if show_separators && display_cfg.preview() && pane_idx < chunks.len() {
            widgets::draw_separator(
                frame,
                Rect {
                    x: chunks[pane_idx].x,
                    y: root_area.y,
                    width: 1,
                    height: root_area.height,
                },
                theme_cfg.separator().as_style(),
            );
            pane_idx += 1;
        }
    }

    // PREVIEW PANE
    if display_cfg.preview() && pane_idx < chunks.len() {
        let area = chunks[pane_idx];
        let bg_filler = Block::default().style(theme_cfg.preview().as_style());
        frame.render_widget(bg_filler, area);

        let is_dir = app
            .nav
            .selected_entry()
            .map(|e| e.is_dir())
            .unwrap_or(false);

        panes::draw_preview(
            frame,
            PaneContext {
                area: chunks[pane_idx],
                block: widgets::get_pane_block("Preview", app),
                accent_style,
                styles: PaneStyles {
                    item: theme_cfg.preview().as_style(),
                    dir: theme_cfg.directory().as_style(),
                    selection: selection_style,
                },
                highlight_symbol: "",
            },
            app.preview.data(),
            if is_dir {
                Some(app.preview.selected_idx())
            } else {
                None
            },
            display_cfg.preview_underline(),
        );
    }
}

pub fn layout_chunks(size: Rect, app: &AppState) -> Vec<Rect> {
    let cfg = app.config().display();
    let mut constraints = Vec::new();
    let show_sep = cfg.separators() && !cfg.is_split();

    let origin = if cfg.origin() { cfg.origin_ratio() } else { 0 };
    let preview = if cfg.preview() {
        cfg.preview_ratio()
    } else {
        0
    };
    let main = cfg.main_ratio();

    let total = origin + preview + main;

    let factor = if total > 100 {
        100.0 / total as f32
    } else {
        1.0
    };

    if cfg.origin() {
        constraints.push(Constraint::Percentage((origin as f32 * factor) as u16));
        if show_sep {
            constraints.push(Constraint::Length(1));
        }
    }

    if total > 100 {
        constraints.push(Constraint::Percentage((main as f32 * factor) as u16));
    } else {
        constraints.push(Constraint::Fill(1));
    }

    if cfg.preview() {
        if show_sep {
            constraints.push(Constraint::Length(1));
        }
        constraints.push(Constraint::Percentage((preview as f32 * factor) as u16));
    }

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(size)
        .to_vec()
}
