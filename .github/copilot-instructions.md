# MCP Rust Starter - Copilot Coding Agent Instructions

## Building and Testing

- **Build the project:**
  ```bash
  cargo build
  ```

- **Build release:**
  ```bash
  cargo build --release
  ```

- **Run the server:**
  ```bash
  cargo run --bin mcp-rust-starter-stdio
  ```

- **Run tests:**
  ```bash
  cargo test
  ```

- **Format code:**
  ```bash
  cargo fmt
  ```

- **Lint code:**
  ```bash
  cargo clippy
  ```

- **Check compilation without building:**
  ```bash
  cargo check
  ```

## Code Conventions

- Use `rustfmt` for consistent formatting
- Address all `clippy` warnings before committing
- Use `?` operator for error propagation
- Prefer `&str` over `String` for function parameters when possible

## Before Committing Checklist

1. ✅ Run `cargo fmt` to format code
2. ✅ Run `cargo clippy` and fix any warnings
3. ✅ Run `cargo build` to verify compilation
4. ✅ Run `cargo test` to verify tests pass

