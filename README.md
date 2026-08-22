# obs-waybar-plugin

obs-waybar-plugin is a simple plugin written in Rust for the [Waybar](https://github.com/Alexays/Waybar) status bar that displays the current scene and blur status in the [Open Broadcaster Software (OBS)](https://obsproject.com/).

## What is "blur status"?

Streamers often use a source (a black screen or blur) that covers the entire screen. To make sure I don't forget to turn it off, I added the status of this source to the line.

## How it works

In your waybar, the following line format will be displayed:
```
<Scene name> [(muted)]
```

For example, if the current scene is `Screen scene` and the blur source is active, the line will be:
```
Screen scene (muted)
```

## Usage

1. Turn on the OBS websocket server

   - Open OBS
   - Go to `Tools` > `Websocket Server`
   - Mark `Enable Websocket Server`

2. Install the plugin

### From releases

You can install the plugin from the [releases page](https://github.com/lynx20wz/obs-waybar-plugin/releases/latest).

### From source

Also, you can build the plugin from source.

You need to have [Git](https://git-scm.com/) and [Rust](https://www.rust-lang.org/) installed to build the plugin.

```bash
git clone https://github.com/lynx20wz/obs-waybar-plugin
cd obs-waybar-plugin
cargo build --release
```

### From package manager

AUR Soon!

---

Optional: move or create a link to the binary in a directory on your PATH.

```bash
sudo mv target/release/obs-waybar-plugin /.local/bin/
# or
sudo ln -s target/release/obs-waybar-plugin /.local/bin/
```

3. Add the following to your Waybar config file:

```jsonc
"custom/obs": {
    "exec": "<path to obs-waybar-plugin> [name of blur source]",
    "interval": 1 # or any other interval you prefer
}
```

Note: if you don't specify a blur source, the plugin will only show the name of the current scene.

## TODO

- [ ] Custom line format
- [ ] Different styles for waybar

## Contributing

Contributions are welcome! If you find a bug or have a feature request, please open an issue or submit a pull request.
