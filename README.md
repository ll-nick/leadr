# Leadr

**Leadr** is a customizable CLI shortcut manager inspired by the leader key concept in (Neo)Vim.
Use memorable key sequences to quickly execute or insert commands in your terminal.

## ⚡️ Requirements

- bash or zsh
- [crossterm](https://docs.rs/crossterm/latest/crossterm/index.html) compatible terminal (see [their Readme for a list](https://github.com/crossterm-rs/crossterm?tab=readme-ov-file#tested-terminals))

Note: `leadr` works best inside a `tmux` session since it can utilize `tmux`'s `send-keys` to execute commands.
Outside of `tmux`, `leadr` will fallback to `eval` and manually appending the command to the shell's history.

## 📦 Installation

<details>
<summary>From pre-built binaries</summary>

You can download pre-built binaries from the [releases page](https://github.com/ll-nick/leadr/releases/latest).
Just copy the binary to a directory in your `PATH`, e.g. using the following command:
```bash
curl -L https://github.com/ll-nick/leadr/releases/latest/download/leadr -o ~/.local/bin/leadr
chmod +x ~/.local/bin/leadr
```

</details>

<details>
<summary>From crates.io</summary>

You can install `leadr` using cargo:
```bash
cargo install leadr
```
This will install the latest version of `leadr` from [crates.io](https://crates.io/crates/leadr).

</details>

<details>
<summary>From source</summary>

You can build `leadr` from source using cargo:

```bash
git clone https://github.com/ll-nick/leadr.git
cd leadr
cargo install --path .
```

</details>

## 🐚 Shell Integration

To use `leadr`, simply add the following line to your shell configuration file (e.g. `~/.bashrc` or `~/.zshrc`):

```bash
# For bash
source <(leadr --bash)
```

```zsh
# For zsh
source <(leadr --zsh)
```

## 🎮 Usage

After installing `leadr`, you can start using it by pressing the `leadr` keybinding followed by a shortcut.

With the default config, you can e.g. execute `git status` by pressing `<Ctrl-g>` followed by `gs`.
Similarly, you can pre-populate `git commit -m ""` by pressing `<Ctrl-g>` followed by `gc`.
Notice how your cursor is placed in between the double quotes? Neat, right?
But that's not all!
`<Ctrl-g>s` will prepend `sudo` to your currently typed command, `<Ctrl-g>c` will append a pipe to the system clipboard.

To list your currently configured shortcuts, run:
```bash
leadr --list
```

Consult the [Configuration](#-configuration) section to learn how to make `leadr` your own.

## 🛠️ Configuration

### Configuration File

`leadr` will automatically create a configuration file and fill it with some default shortcuts the first time you run it.
See [confy's Readme](https://github.com/rust-cli/confy?tab=readme-ov-file#config-file-location) for the location of the configuration file.

Modify the configuration file to add your own shortcuts or adjust the `leadr` keybinding.

### Shortcuts

Define new shortcuts by adding a new entry to the `shortcuts` section of the configuration file.
The key will be the key sequence you want to use, `command` will be the command you want to execute or insert.
Optionally, add a `description` for the `--list` command to show.

Finally, you can specify a `type` to control how the command is executed or inserted.
Here's an overview of the shortcut types that can be configured:

| Type | Description | Cursor Position |
| ---- | ----------- | ---------------- |
| `Execute` (Default) | Execute the command right away | N/A |
| `Replace` | Sets your current prompt to the command | At the end of the command unless `#CURSOR` is specified |
| `Prepend` | Prepend the command to your current prompt | Where it was before adding the prefix |
| `Append` | Append the command to your current prompt | At the end of the command |

### Leadr Keybinding

For a list of currently supported keybindings, see [src/keymap.rs](src/keymap.rs).

### Visual Feedback

You can print the currently typed key sequence at the bottom right of your terminal by setting `print_sequence = true`.
Be aware though that this is somewhat experimental and might lead to issues.
