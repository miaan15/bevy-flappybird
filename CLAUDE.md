# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Bevy game project implementing a Flappy Bird clone using Rust and the Bevy game engine version 0.17.3. The project is currently in early development with minimal structure.

## Development Commands

### Building and Running
- `cargo run` - Build and run the game in debug mode
- `cargo build` - Build the project without running
- `cargo build --release` - Create optimized release build

### Testing and Quality
- `cargo test` - Run all tests
- `cargo clippy` - Run linter checks
- `cargo fmt` - Format code according to Rust standards
- `cargo check` - Quick compile check without producing executable

## Architecture Notes

- **Game Engine**: Bevy 0.17.3 with dynamic linking enabled for faster compilation
- **Language**: Rust 2024 edition
- **Project Structure**: Single-file application currently in src/main.rs
- **Build Configuration**: Development profile uses opt-level 1 for dependencies, opt-level 3 for external packages to balance compile speed and performance

## Bevy-specific Development

- Use ECS (Entity Component System) architecture
- Follow Bevy's plugin-based structure
- Systems are functions that operate on queries of components
- Resources are global data accessible to systems
- Prefer using Bevy's built-in components and systems when possible

## Performance Considerations

The project uses dynamic linking for faster iteration during development. Release builds should use the default settings for optimal performance.