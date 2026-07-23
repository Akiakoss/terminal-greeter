# A terminal greeter
A small terminal greeter, written in Rust.

### Features:
- Colored greeting with your username
- Shows the system uptime (e.g. "2 hours, 1 minute and 9 seconds")
- More features planned (weather, and more)

### Installation:
1. Download the binary from the [Releases](../../releases) page
2. Move it into a directory in your $PATH, e.g.:
```bash
   mv terminal-greeter ~/.cargo/bin/
```
3. Check if that directory is in your $PATH:
```fish
   echo $PATH
```
If not, add it to your `$PATH`. For the fish shell:
```fish
   fish_add_path ~/.cargo/bin
```
4. Add it to your shell config to run on startup.
   Fish shell config: `~/.config/fish/config.fish`
```fish
   if status is-interactive
      terminal-greeter
   end
```

### Supported platforms
Linux and macOS only (relies on the 'users' crate, which doesn't support Windows).