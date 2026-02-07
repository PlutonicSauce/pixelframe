# pixelframe

A terminal-based pixel art animation player written in Rust. Load PNG images and watch them animate in your terminal with real-time rendering!

## Features

- 🎨 **Load PNG images** and display them as pixel art in the terminal
- ⚡ **Real-time rendering** with customizable frame delay
- 🎯 **Brightness-based ASCII mapping** for dynamic visual representation
- 🛠️ **Cross-platform** support (Windows, macOS, Linux)

## How It Works

The player converts pixel colors into ASCII characters based on brightness levels:
- Dark pixels → spaces and dots
- Medium pixels → dashes and plus signs
- Bright pixels → # and █ symbols

This creates a surprisingly accurate representation of the original image in your terminal!

## Installation

Make sure you have Rust installed: https://rustup.rs/

Clone and build:

```bash
git clone https://github.com/PlutonicSauce/pixelframe.git
cd pixelframe
cargo build --release
```

## Usage

Run the animation player:

```bash
cargo run -- <path-to-image> [frame-delay-ms]
```

**Examples:**

```bash
# Display an image with 100ms delay between frames
cargo run -- art.png 100

# Display with 50ms delay (faster animation)
cargo run -- ./sprites/character.png 50

# Default 100ms delay
cargo run -- image.png
```

**Controls:**
- Press `Ctrl+C` to stop the animation

## Project Structure

```
pixelframe/
├── src/
│   ├── main.rs          # Entry point and CLI argument handling
│   ├── animation.rs     # Animation loading and frame management
│   ├── frame.rs         # Individual frame data representation
│   ├── player.rs        # Animation playback logic
│   └── terminal.rs      # Terminal rendering and management
├── Cargo.toml           # Project dependencies
└── README.md            # This file
```

## Dependencies

- **image** - Image loading and processing (PNG, JPEG, etc.)
- **crossterm** - Cross-platform terminal manipulation (raw mode, alternate screen)

## Tips for Best Results

- **Image size**: Works best with images around 80x40 pixels or smaller
- **High contrast**: Images with high contrast between light and dark areas look better
- **Monochrome**: Black and white or grayscale images often look the best
- **Terminal size**: Make sure your terminal is large enough to display the full image

## Example

Load a pixel art PNG file and watch it animate:

```bash
pixelframe my_sprite.png 80
```

The image will display in your terminal using ASCII characters, updating every 80 milliseconds!

## Future Ideas

- Support for sprite sheets (auto-detect grid layout)
- Color support using terminal color codes
- Animation looping controls
- Pause/resume functionality
- Frame-by-frame navigation

## License

MIT