# Rust-Development

Collection of Rust projects maintained in one repository.

## Projects

| Project             | Description               |
| ------------------- | ------------------------- |
| [`snake`](snake/)   | Terminal snake game       |
| [`termix`](termix/) | Rust music player project |

## Workspace commands

Build every project:

```bash
cargo build --workspace
```

Run one project:

```bash
cargo run -p snake
cargo run -p termix
```

Test every project:

```bash
cargo test --workspace
```
