# CodeRift

A command-line tool that formats source code for effective AI analysis by converting code into markdown-formatted blocks with appropriate language identifiers.

## Features

- Automatically processes source code files in specified directories
- Excludes common build and dependency directories
- Supports multiple output options:
  - Standard output (default)
  - File output
  - Clipboard support
- Handles various file types including Rust, TypeScript, SQL, YAML, and more

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/ysakoda/coderift
cd coderift

# Build and install
cargo build --release
cargo install --path .
```

## Usage

```bash
# Process current directory
coderift

# Process specific directory
coderift ./my-project

# Output to file
coderift -o output.md

# Copy to clipboard
coderift -c
```

## Supported File Types

- Rust (.rs)
- TypeScript (.ts, .tsx)
- Astro (.astro)
- SQL (.sql)
- YAML (.yml, .yaml)
- Terraform (.tf)
- JSON (.json)
- TOML (.toml)
- Markdown (.md)
- Dockerfile
- Makefile

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

1. Clone the repository
2. Install Rust (latest stable version)
3. Run tests: `cargo test`
4. Run linter: `cargo clippy`

### Commit Guidelines

We follow conventional commits specification:
- `feat:` New features
- `fix:` Bug fixes
- `docs:` Documentation changes
- `chore:` Maintenance tasks
- `test:` Adding or updating tests

## License

This project is licensed under the MIT License - see the LICENSE file for details.
