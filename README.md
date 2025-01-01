<div align="center">

> V3 is finally out - check out the changelog [here](https://github.com/glzr-io/GlazeWM/releases) 🔥

  <br>
  <img src="./resources/assets/logo.svg" width="230" alt="GlazeWM logo" />
  <br>

# GlazeWM

**A tiling window manager for Windows inspired by i3wm.**

[![Discord invite][discord-badge]][discord-link]
[![Downloads][downloads-badge]][downloads-link]
[![Good first issues][issues-badge]][issues-link]

GlazeWM lets you easily organize windows and adjust their layout on the fly by using keyboard-driven commands.

[Installation](#installation) •
[Default keybindings](#default-keybindings) •
[Config documentation](#config-documentation) •
[FAQ](#faq) •
[Contributing ↗](https://github.com/glzr-io/glazewm/blob/main/CONTRIBUTING.md)

![Demo video][demo-video]

</div>

### 🌟 Key features

- Simple YAML configuration
- Multi-monitor support
- Customizable rules for specific windows
- Easy one-click installation
- Integration with [Zebar](https://github.com/glzr-io/zebar) as a status bar

## Installation

**The latest version of GlazeWM is downloadable via [releases](https://github.com/glzr-io/GlazeWM/releases).** Zebar can optionally be installed as well via a checkbox during installation.

### Winget

GlazeWM can be downloaded via Winget package manager:

```sh
winget install GlazeWM
```

[contributing guide](https://github.com/glzr-io/glazewm/blob/main/CONTRIBUTING.md).

## Default keybindings

On the first launch of GlazeWM, a default configuration can optionally be generated.

Below is a cheat sheet of all available commands and their default keybindings.

![Infographic](/resources/assets/cheatsheet.png)

## Config documentation

The [default config](https://github.com/glzr-io/glazewm/blob/main/resources/assets/sample-config.yaml) file is generated at `%userprofile%\.glzr\glazewm\config.yaml`.

To use a different config file location, you can launch the GlazeWM executable with the CLI argument `--config="..."`, like so:

```sh
./glazewm.exe start --config="C:\<PATH_TO_CONFIG>\config.yaml"
```

Or pass a value for the `GLAZEWM_CONFIG_PATH` environment variable:

```sh
setx GLAZEWM_CONFIG_PATH "C:\<PATH_TO_CONFIG>\config.yaml"
```

With the benefit of using a custom path being that you can choose a different name for the config file, such as `glazewm.yaml`.

### Config: General

```yaml
general:
  # Commands to run when the WM has started (e.g. to run a script or launch
  # another application).
  startup_commands: []

  # Commands to run just before the WM is shutdown.
  shutdown_commands : []

  # Commands to run after the WM config has reloaded.
  config_reload_commands: []

  # Whether to automatically focus windows underneath the cursor.
  focus_follows_cursor: false

  # Whether to switch back and forth between the previously focused
  # workspace when focusing the current workspace.
  toggle_workspace_on_refocus: false

  cursor_jump:
    # Whether to automatically move the cursor on the specified trigger.
    enabled: true

    # Trigger for cursor jump:
    # - 'monitor_focus': Jump when focus changes between monitors.
    # - 'window_focus': Jump when focus changes between windows.
    trigger: "monitor_focus"
```

### Config: Keybindings

The available keyboard shortcuts can be customized via the `keybindings` option. A keybinding consists of one or more key combinations and one or more commands to run when pressed.

It's recommended to use the alt key for keybindings. The Windows key is unfortunately a pain to remap, since the OS reserves certain keybindings (e.g. `lwin+l`).

```yaml
keybindings:
  # Command(s) to run.
  - commands: ["focus --workspace 1"]

    # Key combination(s) to trigger the keybinding.
    bindings: ["alt+1"]

  # Multiple commands can be run in a sequence (e.g. to move a window to a
  # workspace + focus workspace).
  - commands: ["move --workspace 1", "focus --workspace 1"]
    bindings: ["alt+shift+1"]
```

**Full list of keys that can be used for keybindings:**  
<details>
<summary>Keys list</summary>

| Key                   | Description                                                        |
| --------------------- | ------------------------------------------------------------------ |
| `a` - `z`             | Alphabetical letter keys                                           |
| `0` - `9`             | Number keys                                                        |
| `numpad0` - `numpad9` | Numerical keypad keys                                              |
| `f1` - `f24`          | Function keys                                                      |
| `shift`               | Either left or right SHIFT key                                     |
| `lshift`              | The left SHIFT key                                                 |
| `rshift`              | The right SHIFT key                                                |
| `control`             | Either left or right CTRL key                                      |
| `lctrl`               | The left CTRL key                                                  |
| `rctrl`               | The right CTRL key                                                 |
| `alt`                 | Either left or right ALT key                                       |
| `lalt`                | The left ALT key                                                   |
| `ralt`                | The right ALT key                                                  |
| `lwin`                | The left ⊞ Windows logo key                                        |
| `rwin`                | The right ⊞ Windows logo key                                       |
| `space`               | The spacebar key                                                   |
| `escape`              | The ESCAPE key                                                     |
| `back`                | The BACKSPACE key                                                  |
| `tab`                 | The TAB key                                                        |
| `enter`               | The ENTER key                                                      |
| `left`                | The ← arrow key                                                    |
| `right`               | The → arrow key                                                    |
| `up`                  | The ↑ arrow key                                                    |
| `down`                | The ↓ arrow key                                                    |
| `num_lock`            | The NUM LOCK key                                                   |
| `scroll_lock`         | The SCROLL LOCK key                                                |
| `caps_lock`           | The CAPS LOCK key                                                  |
| `page_up`             | The PAGE UP key                                                    |
| `page_down`           | The PAGE DOWN key                                                  |
| `insert`              | The INSERT key                                                     |
| `delete`              | The DELETE key                                                     |
| `end`                 | The END key                                                        |
| `home`                | The HOME key                                                       |
| `print_screen`        | The PRINT SCREEN key                                               |
| `multiply`            | The `*` key (only on numpad)                                       |
| `add`                 | The `+` key (only on numpad)                                       |
| `subtract`            | The `-` key (only on numpad)                                       |
| `decimal`             | The DEL key (only on numpad)                                       |
| `divide`              | The `/` key (only on numpad)                                       |
| `volume_up`           | The volume up key                                                  |
| `volume_down`         | The volume down key                                                |
| `volume_mute`         | The volume mute key                                                |
| `media_next_track`    | The media next track key                                           |
| `media_prev_track`    | The media prev track key                                           |
| `media_stop`          | The media stop key                                                 |
| `media_play_pause`    | The media play/pause key                                           |
| `oem_semicolon`       | The `;`/`:` key on a US standard keyboard (varies by keyboard)     |
| `oem_question`        | The `/`/`?` key on a US standard keyboard (varies by keyboard)     |
| `oem_tilde`           | The `` ` ``/`~` key on a US standard keyboard (varies by keyboard) |
| `oem_open_brackets`   | The `[`/`{` key on a US standard keyboard (varies by keyboard)     |
| `oem_pipe`            | The `\`/`\|` key on a US standard keyboard (varies by keyboard)    |
| `oem_close_brackets`  | The `]`/`}` key on a US standard keyboard (varies by keyboard)     |
| `oem_quotes`          | The `'`/`"` key on a US standard keyboard (varies by keyboard)     |
| `oem_plus`            | The `=`/`+` key on a US standard keyboard (varies by keyboard)     |
| `oem_comma`           | The `,`/`<` key on a US standard keyboard (varies by keyboard)     |
| `oem_minus`           | The `-`/`_` key on a US standard keyboard (varies by keyboard)     |
| `oem_period`          | The `.`/`>` key on a US standard keyboard (varies by keyboard)     |

</details>

If a key is not in the list above, it is likely still supported if you use its character in a keybinding (e.g. `alt+å` for the Norwegian Å character).

> German and US international keyboards treat the right-side alt key differently. For these keyboard layouts, use `ralt+ctrl` instead of `ralt` to bind the right-side alt key.

### Config: Gaps

The gaps between windows can be changed via the `gaps` property in the config file. Inner and outer gaps are set separately.

```yaml
gaps:
  # Gap between adjacent windows.
  inner_gap: "20px"

  # Gap between windows and the screen edge.
  outer_gap:
    top: "20px"
    right: "20px"
    bottom: "20px"
    left: "20px"
```

### Config: Workspaces

Workspaces need to be predefined via the `workspaces` property in the config file. A workspace is automatically assigned to each monitor on startup.

```yaml
workspaces:
  - name: "1"
    display_name: "Work"
    bind_to_monitor: 0
    keep_alive: false
```

### Config: Window rules

Commands can be run when a window is first launched. This is useful for adding window-specific behaviors like always starting a window as fullscreen or assigning to a specific workspace.

```yaml
window_rules:
  - commands: ["move --workspace 1"]
    match:
      - window_process: { regex: "msedge|brave|chrome" }
```

### Config: Window effects

Visual effects can be applied to windows via the `window_effects` option.

```yaml
window_effects:
  focused_window:
    border:
      enabled: true
      color: "#0000ff"
```

### Config: Window behavior

The `window_behavior` config option exists to customize the states that a window can be in (`tiling`, `floating`, `minimized`, and `fullscreen`).

```yaml
window_behavior:
  initial_state: "tiling"
```

### Config: Binding modes

Binding modes are used to modify keybindings while GlazeWM is running.

```yaml
binding_modes:
  - name: "resize"
    keybindings:
      - commands: ["resize --width -2%"]
        bindings: ["h", "left"]
```

## FAQ

**Q: How do I run GlazeWM on startup?**  
Create a shortcut for the executable by right-clicking on the GlazeWM executable -> `Create shortcut`. Put the shortcut in your startup folder, which you can get to by entering `shell:startup` in the top bar in File Explorer.

**Q: How can I create `<insert layout>`?**  
You can create custom layouts by changing the tiling direction with `alt+v`. This changes where the next window is placed in relation to the current window.

**Q: How do I create a rule for `<insert application>`?**  
To match a specific application, you need a command to execute and either the window's process name, title, or class name. For example, if you use Flow-Launcher and want to make the settings window float:

```yaml
window_rules:
  - commands: ["set-floating"]
    match:
      - window_process: { equals: "Flow.Launcher" }
        window_title: { equals: "Settings" }
```

**Q: How can I ignore GlazeWM's keybindings when `<insert application>` is focused?**  
This isn't currently supported, however, the keybinding `alt+shift+p` in the default config is used to disable all other keybindings until `alt+shift+p` is pressed again.

### Tabbed layout
Toggling a tabbed layout can be done via `alt+b` by default. Only one tab is shown at a time, and you can switch tabs by the provided commands.
This can help organize windows in a single container under multiple tabs, similarly to a tabbed layout in i3 or SwayWM.

[discord-badge]: https://img.shields.io/discord/1041662798196908052.svg?logo=discord&colorB=7289DA
[discord-link]: https://discord.gg/ud6z3qjRvM
[downloads-badge]: https://img.shields.io/github/downloads/glzr-io/glazewm/total?logo=github&logoColor=white
[downloads-link]: https://github.com/glzr-io/glazewm/releases
[issues-badge]: https://img.shields.io/badge/good_first_issues-7057ff
[issues-link]: https://github.com/users/lars-berger/projects/2/views/1?sliceBy%5Bvalue%5D=good+first+issue
[demo-video]: resources/assets/demo.webp
