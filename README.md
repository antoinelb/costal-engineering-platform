# Coastal Engineering Platform

Platform to perform common coastal engineering tasks such as designing breakwaters, modelling waves or predicting floods in harbours.

## Development

### Building and Running

```bash
# Compile the project
cargo build

# Run the main application
cargo run

# Check code for errors without building
cargo check

# Remove build artifacts
cargo clean
```

### Testing

```bash
# Run tests with nextest runner
cargo nextest run

# Run tests with coverage reporting
cargo llvm-cov nextest --no-cfg-coverage

# Run linter
cargo clippy
```
