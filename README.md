# ga

A streamlined Git automation CLI tool that simplifies the everyday workflow of staging, committing, and pushing changes to remote repositories.

## Overview

`ga` (Git Automation) is a command-line utility designed to reduce the repetitive nature of common Git operations. Instead of executing multiple Git commands separately, `ga` combines `git add`, `git commit`, and `git push` into a single, interactive workflow with built-in validation and user-friendly output.

## Features

- **Automated Workflow**: Executes `git add .`, `git commit`, and `git push` in sequence
- **Interactive Prompts**: Optionally prompts for commit messages when not provided via command-line arguments
- **Branch Flexibility**: Push to any branch with the `--origin` flag, defaults to `main`
- **Safety Checks**: Validates Git repository presence before executing commands
- **Verbose Mode**: Optional detailed output from underlying Git commands
- **Error Handling**: Clear, actionable error messages for common failure scenarios
- **Colored Output**: Visual feedback with color-coded success, error, and informational messages
- **Zero Configuration**: Works immediately in any Git repository

## Installation

### From Source

Ensure you have Rust and Cargo installed. If not, install them from [rustup.rs](https://rustup.rs/).

```bash
# Clone the repository
git clone <repository-url>
cd ga

# Build and install
cargo install --path .
```

The binary will be installed to `~/.cargo/bin/ga`, which should be in your PATH.

### Manual Build

```bash
# Build in release mode
cargo build --release

# The binary will be available at ./target/release/ga
# Optionally, move it to a location in your PATH
sudo mv ./target/release/ga /usr/local/bin/
```

## Usage

```
ga [OPTIONS]
```

### Options

| Flag | Long Form | Description |
|------|-----------|-------------|
| `-m` | `--message <MESSAGE>` | Commit message (prompts interactively if not provided) |
| `-o` | `--origin <ORIGIN>` | Branch to push to (defaults to `main`) |
| `-v` | `--verbose` | Display detailed output from Git commands |
| `-h` | `--help` | Print help information |
| `-V` | `--version` | Print version information |

### Examples

**Interactive mode with commit message prompt:**
```bash
ga
```

**Provide commit message directly:**
```bash
ga -m "feat: implement user authentication"
```

**Push to a specific branch:**
```bash
ga -m "fix: resolve merge conflicts" -o develop
```

**Combine options for different workflows:**
```bash
ga -m "docs: update API documentation" -o feature/docs-update -v
```

**Use verbose mode to see Git command output:**
```bash
ga -m "refactor: optimize database queries" --verbose
```

## Workflow

When you run `ga`, the following operations are performed in order:

1. **Repository Validation**: Checks for the presence of a `.git` directory
2. **Stage Changes**: Executes `git add .` to stage all modified files
3. **Commit Message**: Either uses the provided message or prompts for input
4. **Create Commit**: Executes `git commit -m "<message>"`
5. **Push Changes**: Pushes to the specified branch (or `main` by default)

Each step provides visual feedback, and the process stops immediately if any operation fails.

## Error Handling

The tool handles various error conditions gracefully:

- **Not a Git Repository**: Exits with an error if no `.git` directory is found
- **Empty Commit Message**: Validates that commit messages are non-empty
- **Nothing to Commit**: Detects when the working tree is clean
- **Push Failures**: Reports remote repository errors clearly

All error messages are color-coded in red for easy identification.

## Requirements

- Git must be installed and accessible in your system PATH
- A valid Git repository (contains a `.git` directory)
- Rust 1.70.0 or later (for building from source)

## Dependencies

- **clap**: Command-line argument parsing with derive macros
- **dialoguer**: Interactive command-line prompts
- **colored**: Terminal text coloring and styling

## Performance

The release build is optimized for size and speed with the following settings:

- Link-time optimization (LTO) enabled
- Symbol stripping for smaller binary size
- Single codegen unit for maximum optimization
- Panic abort strategy for reduced binary size

## License

This project is provided as-is for personal and commercial use.

## Contributing

Contributions are welcome. Please ensure that any pull requests maintain the existing code style and include appropriate error handling.

## Troubleshooting

**Command not found after installation:**
Ensure `~/.cargo/bin` is in your PATH. Add this to your shell configuration file:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

**Permission denied when pushing:**
Verify your Git credentials and remote repository permissions.

**Interactive prompt not working:**
Ensure your terminal supports interactive input. Use the `-m` flag as an alternative.

## Changelog

### Version 0.1.0
- Initial release
- Basic Git workflow automation (add, commit, push)
- Interactive commit message prompts
- Branch selection support
- Verbose mode
- Colored terminal output
