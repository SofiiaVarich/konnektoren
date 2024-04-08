# konnektoren

This is the repository for the Konnektoren project.

The project is a web application that helps German learners to practice the use of Konnektoren. It can be visited at:

https://konnektoren.help/

## Quickstart

## git clone and pull

```sh
git pull
```

### Server

Server code can be found in the `server` branch:

https://github.com/SofiiaVarich/konnektoren/tree/server

### Prerequisites

```bash
cargo install trunk wasm-bindgen-cli
```

### Build

```bash
cargo build
```

### Run

```bash
trunk serve
```

### Frontend

Frontend code can be found in the `main` branch in the src folder.
It is written in Rust using the Yew framework.

### Styles

Styles are written in SCSS and can be found in the `styles` folder.

### Run TUI on terminal

```bash
cargo run --bin konnektoren-tui --features=tui
```

## Source of Konnektoren examples

The examples are taken from the German Course DTB C1.

Thanks to the Teachers and the Students of the German DTB C1 Course at School IFS-Akademy.
