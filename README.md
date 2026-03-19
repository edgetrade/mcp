# @edgedottrade/mcp

MCP server for Edge Trade - blockchain trading and market intelligence for AI agents.

## Installation

```bash
npm install -g @edgedottrade/mcp
```

Or use with npx (no installation required):

```bash
npx @edgedottrade/mcp --help
```

## Binary Variants

This package provides two binary variants optimized for different use cases:

### `edge-desktop` (Default)

**Best for:** Desktop environments with OS keyring support (macOS Keychain, Windows Credential Manager, Linux Secret Service)

- **Key Storage:** OS keyring only - keys never touch the filesystem
- **Authentication:** No passwords required - keys are generated and stored directly in the OS keyring
- **Security:** Maximum security - no unencrypted key material on disk
- **Trade-off:** Keys are tied to the OS keyring and cannot be easily backed up or transferred

```bash
# Key management with edge-desktop
edge-desktop key create    # Generate and store key in OS keyring
edge-desktop key unlock  # Verify key exists in keyring
edge-desktop key lock    # Remove key from keyring
edge-desktop key update  # Generate new key in keyring
```

### `edge-server`

**Best for:** Server environments, CI/CD pipelines, or when you need portable key files

- **Key Storage:** Password-encrypted files in XDG config directory (`~/.config/edge/` on Linux, `~/Library/Application Support/edge/` on macOS, `%APPDATA%\edge\` on Windows)
- **Authentication:** Password-based authentication with PBKDF2 key derivation
- **Security:** Keys are encrypted at rest with user-provided password
- **Trade-off:** Keys can be backed up and transferred, but require password management

```bash
# Key management with edge-server
edge-server key create    # Create password-protected key file
edge-server key unlock    # Unlock with password (stores in keyring temporarily)
edge-server key lock      # Lock session (clear from keyring)
edge-server key update    # Change password
```

## Usage

### With Claude Desktop

Add to your Claude Desktop config:

```json
{
  "mcpServers": {
    "edge": {
      "command": "npx",
      "args": ["-y", "@edgedottrade/mcp", "--api-key", "sk-your-key-here"]
    }
  }
}
```

### With Cursor

Add to your Cursor MCP config:

```json
{
  "mcpServers": {
    "edge": {
      "command": "npx",
      "args": ["-y", "@edgedottrade/mcp", "--api-key", "sk-your-key-here"]
    }
  }
}
```

### Direct Usage

```bash
# Desktop variant (default)
edge-desktop --api-key sk-your-key-here

# Server variant
edge-server --api-key sk-your-key-here
```

## What's Included

This package automatically downloads the appropriate binary for your platform:

- Linux (x86_64, aarch64)
- macOS (Intel, Apple Silicon)
- Windows (x86_64)

The binary is downloaded from [GitHub Releases](https://github.com/edgetrade/mcp/releases) during installation.

## Tools

- **search** - Find tokens by name or address
- **inspect** - Token/pair analytics with 9 views
- **screen** - Filter tokens by market metrics
- **portfolio** - Holdings, PnL, and trade history
- **trade** - Limit orders and strategies
- **alerts** - Real-time price and order alerts

## Key Management Comparison

| Feature | edge-desktop | edge-server |
|---------|--------------|-------------|
| Key Storage | OS keyring only | Password-encrypted files |
| Password Required | No | Yes |
| Filesystem Storage | None | `~/.config/edge/` (XDG) |
| Backup/Transfer | Not possible | Possible with password |
| Best For | Desktop use | Servers, CI/CD |

## Building from Source

### Build desktop variant (default):
```bash
cargo build --release --bin edge-desktop
```

### Build server variant:
```bash
cargo build --release --bin edge-server --features server
```

### Install desktop variant:
```bash
cargo install --bin edge-desktop --path .
```

### Install server variant:
```bash
cargo install --bin edge-server --path . --features server
```

## Documentation

Full docs: [https://docs.edge.trade/agents](https://docs.edge.trade/agents)

## License

MIT
