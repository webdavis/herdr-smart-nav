# herdr-smart-nav

Seamless `Ctrl-h/j/k/l` navigation across Neovim splits and [herdr](https://herdr.dev) panes. One
keypress moves within Neovim when the cursor has somewhere to go in that direction, and moves to the
neighbouring herdr pane when it does not, so the two never need different chords.

A herdr plugin exposing four actions, one per direction: `nav_left`, `nav_down`, `nav_up`,
`nav_right`. A `plugin_action` keybinding passes no arguments, so the direction is baked into each
action's argv.

## Install

```bash
herdr plugin install webdavis/herdr-smart-nav --ref <commit-or-tag> -y
```

herdr clones the repository and runs the manifest's build step (`cargo build --release --locked`), so
a Rust toolchain has to be on the machine. `--ref` is optional but recommended: herdr v1 has no
`plugin update`, so an unpinned install is whatever tip was on the day it ran.

Then bind the four actions in `~/.config/herdr/config.toml`:

```toml
[[keys.command]]
key = "ctrl+h"
type = "plugin_action"
command = "herdr-smart-nav.nav_left"
```

The Neovim side is [smart-splits.nvim](https://github.com/mrjones2014/smart-splits.nvim) with the
same four keys bound to its directional moves.

## License

MIT
