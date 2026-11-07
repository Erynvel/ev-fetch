# EV//Fetch

A clean, minimal system fetch written in Rust. No dependencies, one small binary.

Shows your distro icon next to basic system info: OS, kernel, uptime, shell, CPU and memory.

## Supported distros

Icons are picked from `ID` and `ID_LIKE` in `/etc/os-release`, so derivatives work automatically.

| Icon | Distros |
|------|---------|
| Arch | Arch, Manjaro, EndeavourOS, CachyOS, Artix, Garuda |
| Debian | Debian |
| Ubuntu | Ubuntu, Linux Mint, Pop!_OS, Zorin, elementary |
| Fedora | Fedora, RHEL, CentOS, Rocky, AlmaLinux |
| NixOS | NixOS |
| openSUSE | openSUSE, SUSE |
| Tux | everything else |

Linux only.

## Installation

### Arch Linux (AUR)

```
yay -S ev-fetch
```

### Cargo

Requires [Rust](https://rustup.rs). Installs to `~/.cargo/bin`:

```
cargo install --git https://github.com/YOUR_USERNAME/ev-fetch
```

Make sure `~/.cargo/bin` is in your `PATH`.

### From source

```
git clone https://github.com/YOUR_USERNAME/ev-fetch
cd ev-fetch
cargo build --release
sudo install -Dm755 target/release/evfetch /usr/local/bin/evfetch
```

## Usage

```
evfetch
```

To run it on every new terminal, add `evfetch` to the end of your `~/.bashrc` or `~/.zshrc`.

## Adding a distro

Open `src/main.rs`, find `logo_for` and add a new `match` arm with the distro ID, a color and the ASCII art lines.

## Uninstall

```
cargo uninstall ev-fetch     # installed with cargo
sudo pacman -R ev-fetch      # installed from AUR
```

## License

[MIT](LICENSE)
