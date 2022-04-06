# Note taker

Here will be logo, maybe.

## What is it

CLI tool that allows you to save a note with a single CLI command!

```sh
> note --topic 2read War & Peace by L. Tolstoy

Successfuly saved the note!
```

## Installation

1) Download the prebuilt binary from `Releases` page
2) Place the binary to $PATH and make it executable

## Configuration

Create config file at $HOME_DIR/.notes/Settings.toml with the list of available topics and corresponding text files (any text format is supported: `.txt`, `.md`, etc)

`Settings.toml`

```toml
[topic]
2read = "/home/stepan/Files/notes/2read.md"
thoughts = "/home/stepan/Files/notes/thoughts.md"
```

## Usage

* Save the note

```
note -t read "Rust Book"
```

* See all topics

```
note topics
```

* List of available commands with info

```
note --help
```

