# Work Tracker Web

This is the web appearance for the Work Tracker app.

## Tech stack

- Server: Actix (Rust)
- SSR/Templating: Sailfish (Rust)
- Data fetching: htmx (HTML, Javascript)
- CSS: PicoCSS (CSS)
- Frontend: AlpineJS (Javascript)

## Development

### Setup

Install Rust via [Rustup](https://rustup.rs/).

(Optional) For auto recompilation on edit install [cargo-watch](https://github.com/watchexec/cargo-watch):

```shell
cargo install cargo-watch
```

### Run

Start dev server:

```shell
cargo run
```

(Optional) With auto recompilation on edit:

```shell
cargo watch -w src -w static -w templates -w Cargo.toml -w Cargo.lock -x run
```

### Build

Debug:

```shell
cargo build
```

Release:

```shell
cargo build -r
```
