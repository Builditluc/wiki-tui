# AGENTS.md

## Build/Lint/Test Commands
- `cargo build --release` - Build the project in release mode
- `cargo test --release` - Run all tests in release mode  
- `cargo test --release <test_name>` - Run a specific test
- `cargo clippy` - Run clippy lints
- `cargo fmt --all` - Format all code
- `cargo fmt --all -- --check` - Check formatting without changes

## Code Style Guidelines

### Imports & Structure
- Use `std::sync::Arc` for shared state across async components
- Import `anyhow::{Context, Result}` for error handling
- Use `tokio::sync::{mpsc, Mutex}` for async communication
- Structure: lib.rs contains module declarations, main.rs handles async runtime

### Types & Naming  
- Use `Component` trait for UI components with `init`, `handle_events`, `update`, `render` methods
- Name components ending with `Component` (e.g., `SearchComponent`)
- Use `ActionResult::Consumed(Action)` and `ActionResult::Ignored` for event handling
- Constants use `SCREAMING_SNAKE_CASE`

### Error Handling
- Use `anyhow::Result` for error propagation
- Use `.context()` for adding context to errors
- Use `unwrap_or_else` with fallback defaults for config loading

### Async Patterns
- Use `#[tokio::main]` for main function
- Use `Arc<Mutex<T>>` for shared mutable state
- Use `mpsc::unbounded_channel()` for action communication
- Spawn tasks with `tokio::spawn` for concurrent operations