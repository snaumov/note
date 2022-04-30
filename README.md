# Note taker

Here will be logo, maybe.

## What is it

CLI tool that allows you to save a note with a single command!

```sh
> note --topic 2read "Rust Book"

Successfuly saved the note!
```

## Installation

1) Download the prebuilt binary from `Releases` page
2) Place the binary to $PATH and make it executable

## Configuration

Create config file at $HOME_DIR/.note/Settings.toml with the list of available topics and corresponding text files (any text format is supported: `.txt`, `.md`, etc)

### Paths to files

```toml
[topic]
2read = "/home/stepan/Files/notes/2read.md"
thoughts = "/home/stepan/Files/notes/thoughts.md"
```

### (Optional) Text editor to use

```toml
[general]
editor = "nvim"
```

## Usage

* Save the note

```
note -t read "Rust Book"
```

* Leave the note empty to prompt for an external editor

```
note -t read
```


* See all topics

```
note topics
```

* Use `--prepend` (`-p`) flag to prepend the notes instead of appending to an end of file.

```
note -p
```

* List of available commands with info

```
note --help
```

