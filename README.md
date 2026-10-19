# mpvpaper-stop-rs
Automatically pause and resume [mpvpaper](https://github.com/GhostNaN/mpvpaper) based on visible window coverage in [Hyprland](https://hyprland.org/).

When open windows cover the screen above the configurable threshold, `mpvpaper-stop-rs` sends a pause command over mpv's IPC socket to save resources.

## Features
- Responds instantly to Hyprland events thanks to the [Hyprland IPC](https://wiki.hypr.land/IPC/).
- Also periodically checks in background for consistency.
- You can customize some stuff.
- Written in Rust 🦀.

## Requirements
- [Hyprland](https://hyprland.org/)
- [mpvpaper](https://github.com/GhostNaN/mpvpaper) (started with an mpv IPC socket)

## Installation
### Nix
```nix
inputs = {
  mpvpaper-stop-rs.url = "github:Ev357/mpvpaper-stop-rs";
};
```

### From source
```bash
git clone https://github.com/Ev357/mpvpaper-stop-rs.git
cd mpvpaper-stop-rs
cargo build --release
```

## Usage
Add the following to `hyprland.lua`:

```lua
hl.on("hyprland.start", function()
	hl.exec_cmd("mpvpaper -o 'loop no-audio hwdec=auto --input-ipc-server=/tmp/mpvsocket' '*' /path/to/wallpaper.mp4")
	hl.exec_cmd("mpvpaper-stop-rs -d")
end)
```

## CLI Options
```
Usage: mpvpaper-stop-rs [OPTIONS]

Options:
  -d, --daemonize
  -s, --socket <SOCKET_PATH>                   [default: /tmp/mpvsocket]
  -c, --coverage <COVERAGE_THRESHOLD>          [default: 90]
  -f, --fallback-interval <FALLBACK_INTERVAL>  [default: 1]
  -r, --retry-interval <RETRY_INTERVAL>        [default: 2]
  -h, --help                                   Print help
  -V, --version                                Print version
```

## Credits
- [mpvpaper-stop](https://github.com/pvtoari/mpvpaper-stop)

## License
This project is licensed under the MIT license. See the `LICENSE` file for details.
