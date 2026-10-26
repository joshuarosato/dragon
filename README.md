# dragon

A tiny GTK4/libadwaita drag-and-drop source for Wayland desktops.

```sh
dragon myfile.zip
```

Dragon opens a small window showing the file. Drag the card into a browser file
uploader, chat application, editor, or any other application that accepts file
DnD.

The program is compositor-agnostic: it uses GTK/GDK's normal drag-and-drop APIs,
not Hyprland IPC or layer-shell.

## Features

- One or many files (`dragon FILE...`)
- Reads newline-delimited file paths from stdin (piped input or `-`)
- Native `GdkFileList` plus explicit `text/uri-list` data
- `COPY` semantics only; Dragon never deletes or moves source files
- Uses the system file icon for a single file
- Closes after a successful drop by default; `Esc` cancels the popup
- `--keep` keeps it open for repeated drags
- Blueprint source is included; a GTK Builder XML counterpart is checked in

## Requirements

At runtime/build time:

- Rust 1.83+
- GTK 4.8+
- libadwaita 1.x

Blueprint Compiler is only needed when editing `data/window.blp`; ordinary
`cargo build` uses the checked-in `data/window.ui`.

### Arch Linux

```sh
sudo pacman -S --needed rust gtk4 libadwaita
```

For UI development too:

```sh
sudo pacman -S --needed blueprint-compiler
```

## Build

```sh
cargo build --release
./target/release/dragon ~/Downloads/myfile.zip
```

Or:

```sh
make release
```

## Install for your user

```sh
make install
```

This defaults to `~/.local`. Override `PREFIX` if desired:

```sh
make install PREFIX=/usr/local
```

## Usage

```text
dragon [OPTIONS] [FILE...]

FILES:
    -                Read newline-delimited file paths from stdin

OPTIONS:
    -k, --keep       Keep the window open after a successful drop
    -h, --help       Print help
    -V, --version    Print version
```

Examples:

```sh
dragon archive.zip
dragon photo.jpg report.pdf
dragon *.png
dragon --keep build/*.tar.zst
printf '%s\n' photo.jpg report.pdf | dragon
printf '%s\n' report.pdf | dragon photo.jpg -
dragon -- --file-starting-with-dash
```

When no file arguments are provided and stdin is piped, Dragon reads paths from stdin automatically. Use `-` to combine stdin paths with explicit file arguments. Empty input lines are ignored.

## Hyprland

Dragon creates a normal GTK toplevel. If you want Hyprland to enforce floating
or sizing rules, a configuration like this is a useful starting point:

```ini
windowrule = float, match:class io.github.joshuarosato.Dragon
windowrule = center, match:class io.github.joshuarosato.Dragon
```

You may not need any rule at all; the app requests a small, non-resizable window.

## Blueprint

`data/window.blp` is the source UI definition. The `data/window.ui` counterpart is
checked in intentionally so Blueprint is not a build dependency.

After changing the Blueprint file:

```sh
make ui
```

which runs:

```sh
blueprint-compiler compile --output data/window.ui data/window.blp
```

## How the drag works

For each path Dragon creates a `gio::File`. It exposes the collection as a
`gdk::FileList`, which GDK can serialize into file-transfer formats, and also
adds an explicit `text/uri-list` provider. A `gtk::DragSource` attached to the
whole card starts the drag and advertises `gdk::DragAction::COPY`.

The window closes on GDK's `dnd-finished` signal, which fires after the drop target
has finished reading the offered data. Cancelling a drag leaves Dragon open.

## License

Apache License 2.0. Copyright 2026 Joshua Rosato.
