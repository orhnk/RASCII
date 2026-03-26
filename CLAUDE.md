# RASCII Project Notes

## Primary Focus: Camera-to-ASCII Pipeline

The core workflow is native Rust webcam capture to colored ASCII art in the terminal — no Python required.

### Key commands
```bash
# Single-shot webcam capture (block charset, background fill, color)
./target/release/rascii --camera -c -b -C block

# Live webcam feed
./target/release/rascii --camera --live -c -b -C block

# Resize to fit — useful widths depending on context:
./target/release/rascii --camera -c -b -C block -w 80    # quarter-screen, fast
./target/release/rascii --camera -c -b -C block -w 120   # half-screen
./target/release/rascii --camera -c -b -C block -w 200   # wide terminal
./target/release/rascii --camera --live -c -b -C block -w 80 --fps 10  # small + fast refresh

# Render a file (original mode)
./target/release/rascii image.png -c -b -C block -w 100
```

Without `-w`, width defaults to your terminal's column count. On a retina display that can be 200+ columns — pass `-w 80` or `-w 120` for a more manageable size.

### Pipeline
1. `nokhwa` captures a frame from the webcam (with 30-frame warmup for auto-exposure)
2. Frame is mirrored (selfie view) in memory — no temp files
3. `image` crate feeds the frame directly to the ASCII renderer
4. Output goes to terminal with full 24-bit RGB color via ANSI codes

### Camera flags
- `--camera` — use webcam instead of a file
- `--live` — continuous refresh mode (Ctrl+C or `q` to stop)
- `--fps N` — refresh rate for live mode (default 4)
- `--camera-index N` — device index if you have multiple cameras (default 0)
- `--no-mirror` — disable selfie horizontal flip
- `--warmup N` — auto-exposure warmup frames (default 30)

### Rendering flags
- `-c` — enable color output
- `-b` — background color fill (densest color output)
- `-C block` — block charset (`░▒▓█`) for pixel-art look
- `-w N` — width in characters (defaults to terminal width)
- `-H N` — height in characters (aspect ratio preserved if only one is set)
- `-i` — invert for light backgrounds

### Future directions
- Vision LLM integration (LLaVA or similar) to analyze or steer rendering
- Live mode refinements

## Project Structure

- `src/main.rs` — CLI entry point, arg parsing, dispatch
- `src/camera.rs` — webcam capture, live loop, terminal control
- `src/lib.rs` — public API (`render`, `render_image`, `render_to`, `render_image_to`)
- `src/image_renderer.rs` — image-to-ASCII rendering with ANSI color
- `src/renderer.rs` — `RenderOptions` and `Renderer` trait
- `src/charsets.rs` — character set definitions
- `camera_ascii.py` — legacy Python+OpenCV pipeline (superseded by `--camera`)
- `ansi_to_hex.py` — parses RASCII ANSI output to hex colors, JSON, or HTML canvas
- `paint_berry.py` — paints ASCII art onto Berry Fast blockchain pixel board

## Build

```bash
cargo build --release
```
