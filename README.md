# Catppuccin Flavours

Theme-hopping is nice, but base16 & base24 color themes are pretty limited in their capabilities, and their ports are not always pretty.
This is **Catppuccin Flavours**, a [`flavours`](https://github.com/Misterio77/flavours)-like cli tool to change themes for your apps by using existing Catppuccin themes for these apps as templates !

Note: the original tera templates that are built in were slightly modified to be easier to render. This program does not support 100% of templates made for Catppuccin.

## Builtin Themes

The [`themes`](./themes) folder contains builtin themes. You can add custom themes in the `~/.config/cfl/themes/` directory. Your custom themes take precedence over builtin ones.

## Builtin Templates

The [`templates`](./templates) folder contains builtin templates. You can add custom templates in the `~/.config/cfl/templates/` directory. Your custom templates take precedence over builtin ones.

## Configuration

Configure the files you want to be regenerated like so (the command is optional):

```toml
[[entries]]
template = "fzf"
target = "~/.config/fish/fzf.fish"
command = "fish ~/.config/fish/fzf.fish"
```
