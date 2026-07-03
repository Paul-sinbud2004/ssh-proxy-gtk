# SSH Port Forwarding

A GTK4-based SSH dynamic port forwarding application for Linux, providing a user-friendly GUI to create SOCKS5 proxies via SSH tunnels.

## Features

- **SSH Dynamic Port Forwarding**: Creates SOCKS5 proxy via SSH tunnel, similar to `ssh -D local_port user@server -p ssh_port -N`
- **GTK4 GUI**: Modern, intuitive interface with support for both Chinese and English
- **Dual Authentication**: Support for both password authentication and SSH key authentication
- **Connection History**: Automatically saves and loads last connection settings
- **Server Address History**: Quick access to previously used server addresses
- **Security Tips**: Built-in security reminders for best practices

## Requirements

- Rust (stable)
- GTK4 development libraries
- `sshpass` (for password authentication)
- `openssh-client`

### Ubuntu/Debian

```bash
sudo apt install libgtk-4-dev sshpass openssh-client
```

### Fedora

```bash
sudo dnf install gtk4-devel sshpass openssh-clients
```

## Installation

```bash
git clone https://github.com/yourusername/ssh-proxy-gtk.git
cd ssh-proxy-gtk
cargo build --release
```

## Usage

```bash
# Run the application
cargo run

# Or run the release binary
./target/release/ssh-proxy-gtk
```

### Configuration

1. **Server Address**: The SSH server IP address or hostname
2. **SSH Port**: The SSH port (default: 22)
3. **Username**: SSH username
4. **Password**: SSH password (or use key authentication)
5. **Local Port**: Local SOCKS5 proxy port (default: 1088)
6. **Use Key Auth**: Check to use SSH key authentication instead of password
7. **Key Path**: Path to your SSH private key (default: ~/.ssh/id_ed25519)

### Browser Configuration

Set your browser proxy to:
- **Protocol**: SOCKS5
- **Host**: 127.0.0.1
- **Port**: Your chosen local port (default: 1088)

## Security

- **Recommended**: Use SSH key authentication instead of passwords
- Config files are set to readable only by owner (600 permissions)
- SSH host key verification is enabled by default
- Passwords are stored in plain text in the config file (consider using key auth)

## Build from Source

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release
```

## Project Structure

```
ssh-proxy-gtk/
├── src/
│   └── main.rs          # Main application code
├── Cargo.toml           # Rust dependencies
├── Cargo.lock           # Locked dependencies
└── README.md            # This file
```

## License

MIT License

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.