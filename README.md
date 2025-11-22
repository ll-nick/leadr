# leadr

**leadr** is a customizable CLI command manager inspired by the leader key concept in (Neo)Vim.
Use memorable key sequences to quickly execute or insert commands in your terminal.

## 🚀 Demo

https://github.com/user-attachments/assets/ace9ec1c-0976-4868-abe9-a7499101c03a

## 💪 Features

- **Customizable Mappings**: Define your own key sequences to set your prompt.
- **Insert or Execute**: Immediately execute `git status` or just insert `git commit -m ""` ready for you to fill in.
- **Prepend/Append**: Forgot `sudo`? Just prepend it to the your prompt and keep typing.
- **Surround**: Wrap your current command in quotes or use `$(command substitution)` in the blink of an eye.
- **Cursor Positioning**: Automatically place your cursor at the right position after inserting or replacing commands.
- **Evaluate pre-insert**: Need the current date in your file name? Evaluate a command, then insert it.
- **Keybinding Panel**: In case they slipped your mind, see available mappings in a fancy looking pop-up.

![Panel](assets/panel.png)

## 🎮 Usage

After installing `leadr`, you can start using it by pressing the `leadr` keybinding followed by a key sequence.

With the default config, you can e.g. execute `git status` by pressing `<Ctrl-g>` followed by `gs`.
Similarly, you can pre-populate `git commit -m ""` by pressing `<Ctrl-g>` followed by `gc`.
Notice how your cursor is placed in between the double quotes? Neat, right?

But that's not all!
`<Ctrl-g>ps` will prepend `sudo` to your currently typed command, `<Ctrl-g>y` will append a pipe to the system clipboard.

Want me to continue?
Alright: `<Ctrl-g>id` will insert the current date wherever your cursor is, `<Ctrl-g>sq` will surround your current command in quotes.

You can get the tl;dr of all this by running
```bash
leadr --list
```
which will show you a list of all available mappings and their descriptions.

Consult the [Configuration](#-configuration) section to learn how to make `leadr` your own.

## ⚡️ Requirements

- A supported shell: bash, fish, nushell or zsh
- [crossterm](https://docs.rs/crossterm/latest/crossterm/index.html) compatible terminal (see [their Readme for a list](https://github.com/crossterm-rs/crossterm?tab=readme-ov-file#tested-terminals))

## 📦 Installation

<details>
<summary>📦 Using pre-built binaries</summary>

You can download pre-built binaries from the [releases page](https://github.com/ll-nick/leadr/releases/latest).
Just copy the binary to a directory in your `PATH` and make it executable.

I highly recommend the amazing [mise-en-place](https://mise.jdx.dev/) to do this for you while also managing updates.
With mise installed, run:
`mise use --global ubi:ll-nick/leadr`

</details>

<details>
<summary>🦀 Via crates.io</summary>

You can install `leadr` using cargo:
```bash
cargo install leadr
```
This will install the latest version of `leadr` from [crates.io](https://crates.io/crates/leadr).

</details>

<details>
<summary>🍺 Via Homebrew</summary>

On macOS or Linux with [Homebrew](https://brew.sh/) installed, you can use:

```bash
brew tap ll-nick/leadr
brew install leadr
```

</details>

<details>
<summary>💻 From source</summary>

You can build `leadr` from source using cargo:

```bash
git clone https://github.com/ll-nick/leadr.git
cd leadr
cargo install --path .
```

</details>

## 🐚 Shell Integration

To use `leadr`, simply add the following to your shell configuration file:

```bash
# ~/.bashrc
source <(leadr --bash)
```

```fish
# ~/.config/fish/config.fish
leadr --fish | source
```

```nushell
# nushell/config.nu
mkdir ($nu.data-dir | path join "vendor/autoload")
leadr --nu | save -f ($nu.data-dir | path join "vendor/autoload/leadr.nu")
```

```zsh
# ~/.zshrc
source <(leadr --zsh)
```

## 🛠️ Configuration

To get started configuring `leadr`, run `leadr --init` to create the default configuration files and start tweaking from there.

To overwrite the default configuration directory (see [the directories crate](https://crates.io/crates/directories) for the default value of the `config_dir`) you can set the `LEADR_CONFIG_DIR` environment variable to your desired path.

### config.toml

The main configuration file to set your `leadr` key, tweak the keybinding panel and other global settings.
Most of these settings should be self-explanatory but here are some notes on a few of them:

##### leadr_key

The default keybinding is `<C-g>` (the `Ctrl` key and the `g` key pressed in one chord), but you can change that by modifying the `leadr_key` in the `config.toml` file.
The syntax mimics that of Vim's keybindings, e.g. `<M-x>`, `<C-s>`, `<F5>`, etc. and supports (with the exception of nushell) chains like `<C-x><C-s>abc`.

> **Fair warning**: Keybindings in the shell are a bit of an arcane mess.
> I asked my good friend Chad Gibbidy to help me out with this.
> Lots of bindings work but some don't.
> Feel free to experiment but if you run into issues, `Ctrl` + a letter is probably your safest bet.

##### redraw_prompt_line

> **Note**: This setting concerns only `bash` users. It has no effect in other shells.

Due to the way key bindings work in `bash`, the current prompt line will disappear while `leadr` is activated.
To cover this up, `leadr` will redraw it after start-up.
If you experience issues with this, you can disable it by setting `redraw_prompt_line = false`.

### Mappings

Mappings are defined in the `mappings.toml` file located in the `leadr` config directory.
If you prefer some more structure, you can also create a `mappings/` directory and define your mappings in separate and arbitrarily nested toml files inside that directory.

A mapping looks like this:

```toml
[abc]
command = "my-command"
description = "My optional command description"
insert_type = "Append" # See below for options, default: "Replace"
evaluate = true # Default: false
execute = true # Default: false
```

The only required field is `command` (and the key of course).
All other fields will use their default values if not specified.

You can customize the behavior of the mapping by specifying `insert_type`, `evaluate`, and `execute` options.
Here's an overview of the available options:

| Setting | Options | Description |
| ------- | ------- | ----------- |
| `insert_type` | 'Replace' (default) | Clears the current prompt and replaces it with the command. Cursor will be placed at the end of the prompt. |
|               | 'Insert' | Inserts the command at the current cursor position. Cursor will be placed at the end of the inserted command. |
|               | 'Prepend' | Prepends the command to the current prompt. Cursor will be placed where it was before adding the prefix. |
|               | 'Append' | Appends the command to the current prompt. Cursor will be placed at the end of the prompt. |
|               | 'Surround' | Surrounds the current prompt, i.e. adds a prefix and a suffix. The defined command has to contain `#COMMAND` which will be replaced by the current prompt. The cursor will be placed at the end of the prompt. |
| `evaluate` | `true` or `false` (default) | If `true`, the command will be evaluated before being inserted. |
| `execute` | `true` or `false` (default) | If `true`, the command will be executed immediately. |

The cursor position after inserting or replacing commands can be customized by adding `#CURSOR` to the command.
For the `git commit -m ""` example, define the command as `git commit -m "#CURSOR"` to place the cursor between the double quotes after inserting the command.
This works for all insert types but will have no effect if `evaluate` or `execute` is set to `true`.

> **Note**: For `bash` and `zsh`, `execute` works best inside a `tmux` session since it can utilize `tmux`'s `send-keys` to execute commands.
> Outside of `tmux`, `leadr` will fallback to `eval` and manually append the command to the shell's history.


### Keybinding Panel

`leadr` comes with a user interface that looks suspiciously similar to [which-key](https://github.com/folke/which-key.nvim).
It is activated by default and will pop up shortly after pressing the `leadr` keybinding.

You can customize the panel by modifying the `panel` section in the `config.toml` file.

#### Color Theme

The default color theme uses the [catppuccin mocha](https://github.com/catppuccin/catppuccin?tab=readme-ov-file#-palette) color palette.
You can customize the colors by modifying the `theme_name` in the panel section of the `config.toml` file.

All catppuccin flavors are builtin and can be activated by setting `theme_name` to `catppuccin-{flavor}`.

Custom themes can be defined by adding `themes/theme-name.toml` in the `leadr` config directory.
To e.g. create a high contrast theme, add `themes/high-contrast.toml` with the following content:

```toml
accent = { r = 255, g = 255, b = 0 }
background = { r = 0, g = 0, b = 0 }
text_highlight_primary = { r = 255, g = 0, b = 0 }
text_highlight_secondary = { r = 255, g = 255, b = 255 }
text_primary = { r = 255, g = 255, b = 255 }
text_secondary = { r = 192, g = 192, b = 192 }
```

and set `theme_name = "high-contrast"` in the `config.toml` file.

## ❤️ Contributions

Thanks @Banh-Canh for contributing the fish integration!  
Thanks @johnallen3d and @bjohnso5 for testing `leadr` on macOS!  
Thanks @ltaupiac for setting up the Homebrew tap!  
Thanks @johnstegeman for improving the fish shell key parsing!  
