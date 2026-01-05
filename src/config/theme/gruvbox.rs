use super::{ColorPair, MarkerTheme, Theme, WidgetTheme};
use ratatui::style::Color;

const BG_HARD: (u8, u8, u8) = (29, 32, 33);
const BG: (u8, u8, u8) = (40, 40, 40);
const BG_SOFT: (u8, u8, u8) = (50, 48, 47);
const BG_LIGHT: (u8, u8, u8) = (251, 241, 199);
const FG4: (u8, u8, u8) = (168, 153, 132);
const GRAY: (u8, u8, u8) = (146, 131, 116);
const YELLOW: (u8, u8, u8) = (250, 189, 47);
const GREEN: (u8, u8, u8) = (152, 151, 26);
const BLUE: (u8, u8, u8) = (131, 165, 152);

fn rgb(c: (u8, u8, u8)) -> Color {
    Color::Rgb(c.0, c.1, c.2)
}

// Gruvbox Dark Hard - subtle
pub fn gruvbox_dark_hard() -> Theme {
    Theme {
        accent: ColorPair {
            fg: rgb(YELLOW),
            bg: Color::Reset,
            selection_fg: None,
            selection_bg: None,
        },
        selection: ColorPair {
            bg: rgb(BG_SOFT),
            fg: Color::Reset,
            selection_fg: None,
            selection_bg: None,
        },
        underline: ColorPair {
            fg: Color::Reset,
            bg: Color::Reset,
            selection_fg: None,
            selection_bg: None,
        },
        entry: ColorPair::default(),
        directory: ColorPair {
            fg: rgb(GREEN),
            ..ColorPair::default()
        },
        separator: ColorPair {
            fg: rgb(GRAY),
            ..ColorPair::default()
        },
        selection_icon: "".to_string(),
        parent: ColorPair::default(),
        preview: ColorPair::default(),
        path: ColorPair {
            fg: rgb(YELLOW),
            ..ColorPair::default()
        },
        status_line: ColorPair {
            fg: rgb(FG4),
            bg: rgb(BG_HARD),
            ..ColorPair::default()
        },
        marker: MarkerTheme::default(),
        widget: WidgetTheme::default(),
        info: WidgetTheme::default(),
        name: Some("gruvbox-dark-hard".to_string()),
    }
}

// Gruvbox Dark (medium)
pub fn gruvbox_dark() -> Theme {
    Theme {
        accent: ColorPair {
            fg: rgb(YELLOW),
            bg: Color::Reset,
            selection_fg: None,
            selection_bg: None,
        },
        selection: ColorPair {
            bg: rgb(BG_SOFT),
            fg: Color::Reset,
            selection_fg: None,
            selection_bg: None,
        },
        underline: ColorPair {
            fg: Color::Reset,
            bg: Color::Reset,
            selection_fg: None,
            selection_bg: None,
        },
        entry: ColorPair::default(),
        directory: ColorPair {
            fg: rgb(GREEN),
            ..ColorPair::default()
        },
        separator: ColorPair {
            fg: rgb(GRAY),
            ..ColorPair::default()
        },
        selection_icon: "".to_string(),
        parent: ColorPair::default(),
        preview: ColorPair::default(),
        path: ColorPair {
            fg: rgb(YELLOW),
            ..ColorPair::default()
        },
        status_line: ColorPair {
            fg: rgb(FG4),
            bg: rgb(BG),
            ..ColorPair::default()
        },
        marker: MarkerTheme::default(),
        widget: WidgetTheme::default(),
        info: WidgetTheme::default(),
        name: Some("gruvbox-dark".to_string()),
    }
}

pub fn gruvbox_light() -> Theme {
    Theme {
        accent: ColorPair {
            fg: rgb(BLUE),
            bg: Color::Reset,
            selection_fg: None,
            selection_bg: None,
        },
        selection: ColorPair {
            bg: rgb((239, 213, 174)),
            fg: Color::Reset,
            selection_fg: None,
            selection_bg: None,
        },
        underline: ColorPair {
            fg: Color::Reset,
            bg: Color::Reset,
            selection_fg: None,
            selection_bg: None,
        },
        entry: ColorPair::default(),
        directory: ColorPair {
            fg: rgb(BLUE),
            ..ColorPair::default()
        },
        separator: ColorPair {
            fg: rgb(GRAY),
            ..ColorPair::default()
        },
        selection_icon: "".to_string(),
        parent: ColorPair::default(),
        preview: ColorPair::default(),
        path: ColorPair {
            fg: rgb(YELLOW),
            ..ColorPair::default()
        },
        status_line: ColorPair {
            fg: rgb(FG4),
            bg: rgb(BG_LIGHT),
            ..ColorPair::default()
        },
        marker: MarkerTheme::default(),
        widget: WidgetTheme::default(),
        info: WidgetTheme::default(),
        name: Some("gruvbox-light".to_string()),
    }
}
