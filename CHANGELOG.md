# Changelog

## [v0.2.4] - 2025-12-20
### Changed
- Switched `always_show` config to use `HashSet<OsString>` for much faster and efficient lookups.
- Set default value of `always_show` to empty for a saner default config.

### Internal
- Refactored config module for better maintainability: introduced `RawConfig` for deserialization, and `Config` for runtime usage.

---

## What's New in v0.2.1 and v0.2.2

## UI improvements

- Improved pane customization by adding pane specific selection cache.
- Custom Path Styling (v0.2.2): Addded a dedicated theme.path configuration to customize the path at the top of the UI.
- Pane-Specific Selections (v0.2.2): Added selection foreground and background colors for each pane for more customization.

## Performance

- Switched to crossbeam-channel for better thread sync.
- Performance improvements by reducing event_loop string creation.
- Optimized always_show and other flags by using Atomics.
- Migrated text rendering to use Line and Span for better performance and future-proofing.

## Fixes
- Fixed preview sorting issue. Now shows the directories in the correct oder.
