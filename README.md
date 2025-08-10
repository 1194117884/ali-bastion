# Ali-Bastion

A CLI tool for managing and connecting to Aliyun bastion hosts.

## Building

```bash
cargo build
```

## Running Tests

```bash
cargo test
```

This will run all unit tests and integration tests for the application.

## Test Structure

- Unit tests are included within each module file using `#[cfg(test)]` blocks
- Integration tests are in the `tests/` directory (if any)
- The `lib.rs` file exposes modules for testing purposes

## Running the Application

```bash
cargo run -- [commands]
```

## Platform Requirements

### Unix-like Systems (Linux, macOS)
- Standard OpenSSH client (usually pre-installed)
- `sshpass` utility for password-based authentication:
  - Ubuntu/Debian: `sudo apt-get install sshpass`
  - macOS: `brew install sshpass`

### Windows
- For password-based authentication: `plink` (from PuTTY suite)
  - Download from the [PuTTY website](https://www.chiark.greenend.org.uk/~sgtatham/putty/latest.html)
  - Ensure `plink.exe` is in your PATH
- For key-based authentication: Windows 10+ includes OpenSSH client by default

### Available Commands

- `add` - Add a new host
- `remove` - Remove a host
- `list` - List all hosts
- `connect` - Connect to a host

#### Connect Command

The connect command can be used in two ways:

1. **Direct mode**: `cargo run -- connect [host_name]` - Connect directly to a specific host
2. **Interactive mode**: `cargo run -- connect` - List all configured hosts and allow selection using arrow keys (↑/↓) and Enter