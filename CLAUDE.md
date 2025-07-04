# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a coastal engineering platform built in Rust for performing common coastal engineering tasks such as designing breakwaters, modelling waves, and predicting floods in harbours.

## Development Commands

- `cargo build` - Compile the project
- `cargo run` - Run the main application
- `cargo nextest run` - Run tests with nextest runner
- `cargo check` - Check code for errors without building
- `cargo clean` - Remove build artifacts
- `cargo llvm-cov nextest` - Run tests with coverage reporting

## Architecture

This is a basic Rust project with:
- Single binary crate structure
- Main entry point at `src/main.rs`
- Uses Rust edition 2024
- Currently minimal implementation with room for expansion into coastal engineering modules

## Key Files

- `Cargo.toml` - Project configuration and dependencies
- `src/main.rs` - Main application entry point

## Writing Guidelines

- Always use latex when writing mathematical symbols in markdown documents except for inside code blocks

## Claude Code Guidelines

- Always refer to @plan.md before doing anything
- Always use context7 when unsure about the api
