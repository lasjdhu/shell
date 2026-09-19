# Shell [![MIT License](https://img.shields.io/badge/license-MIT-22c55e.svg)](LICENSE)

A small Unix shell written in Rust, with built-in commands and execution of
programs available through `PATH`.

## Motivation

The project explores the core mechanics behind an interactive shell: reading
and parsing input, resolving executables, spawning processes, and maintaining
session state such as the current directory.

## Usage

```bash
./shell
```

The wrapper builds and runs the release binary through Cargo. You can also run
the project directly with `cargo run --release`.

## Features

- Run commands available in `PATH`
- Built-in commands:
  - `exit` — close the shell
  - `type` — describe a built-in or executable
  - `cd` — change the current directory
  - `pwd` — print the current directory
  - `echo` — print arguments

## Preview

<p align="center">
  <img alt="Shell session" src="readme-assets/preview.gif" />
</p>
