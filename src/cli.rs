use crate::config::Config;

pub enum CliAction {
    RunApp,
    Exit,
}

pub fn handle_args() -> CliAction {
    let args: Vec<String> = std::env::args().collect();
    let config_path = Config::default_path();

    if args.len() <= 1 {
        return CliAction::RunApp;
    }

    match args[1].as_str() {
        "-h" | "--help" => {
            print_help();
            CliAction::Exit
        }
        "--config-help" => {
            print_config_help();
            CliAction::Exit
        }
        "--init" => {
            if let Err(e) = Config::generate_default(&config_path, false) {
                eprintln!("Error: {}", e);
            }
            CliAction::Exit
        }
        "--init-minimal" => {
            if let Err(e) = Config::generate_default(&config_path, true) {
                eprintln!("Error: {}", e);
            }
            CliAction::Exit
        }
        arg => {
            eprintln!("Unknown argument: {}", arg);
            CliAction::Exit
        }
    }
}

fn print_help() {
    println!(
        r#"runner - A fast and lightweight console file browser written in Rust

USAGE:
    rn [OPTIONS]

OPTIONS:
    -h, --help            Print help information
    --init                Generate full default config at ~/.config/runner/runner.toml
    --init-minimal        Generate minimal config (overrides only)
    --config-help

ENVIRONMENT:
    RUNNER_CONFIG         Override the default config path
"#
    );
}

fn print_config_help() {
    let help_text = r##"
runner - Full Configuration Guide (runner.toml)

# General Settings
  dirs_first       (bool) Sort directories before files
  show_hidden      (bool) Show hidden files (dotfiles)
  show_system      (bool) Show system/protected files (Mostly useful for windows)
  case_insensitive (bool) Ignore case when searching/sorting
  always_show      (vec)  Options to always show certain hidden directories

[display]
  selection_marker (bool) Show the cursor marker
  dir_marker       (bool) Show a marker for directories
  borders          (str)  "none", "unified", or "split"
  titles           (bool) Show pane titles at the top
  separators       (bool) Draw vertical lines between panes
  origin           (bool) Show the parent directory pane
  preview          (bool) Show the file preview pane
  origin_ratio     (u16)  Width % of the origin pane
  main_ratio       (u16)  Width % of the center pane
  preview_ratio    (u16)  Width % of the preview pane
  scroll_padding   (usize)Scroll padding of the main pane

[theme]
  background       (str)  Hex (#RRGGBB) or "default"
  selection_fg     (str)  Hex or color name for selection
  accent_fg        (str)  Color for markers and highlights
  entry_fg         (str)  Default text color for entries
  separator_fg     (str)  Color of the vertical split lines
  selection_icon   (str)  The cursor string (e.g., "> ")
  origin_fg        (str)  Text color for the parent pane
  preview_fg       (str)  Text color for the preview pane

[editor]
  cmd              (str)  Command to open files (e.g., "nvim", "code")

[keys]
  open_file        (list) e.g., ["Enter"]
  go_up            (list) e.g., ["k", "Up Arrow"]
  go_down          (list) e.g., ["j", "Down Arrow"]
  go_origin        (list) e.g., ["h", "Left Arrow", "Backspace"]
  go_into_dir      (list) e.g., ["l", "Right Arrow"]
  quit             (list) e.g., ["q", "Esc"]

EXAMPLES:
  separator_fg = "#555555"
  main_ratio = 40
  borders = "unified"
  open_file = ["Enter", "o"]
"##;

    println!("{}", help_text);
}
