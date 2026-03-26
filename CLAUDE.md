# RASCII Project Notes

## Primary Focus: Camera-to-ASCII Pipeline

The core workflow is `camera_ascii.py` — webcam capture to colored ASCII art in the terminal.

### Key command
```bash
python camera_ascii.py -b -C block
```

### Pipeline
1. OpenCV captures a frame from the webcam (with 30-frame warmup for auto-exposure)
2. Frame is mirrored (selfie view) and written to a temp file
3. RASCII (Rust binary at `./target/release/rascii`) renders it as colored ANSI art
4. Output goes to terminal with full 24-bit RGB color

### Flags worth knowing
- `-b` — background color fill (densest color output)
- `-C block` — block charset (`░▒▓█`) for pixel-art look
- `--live` — continuous refresh mode (Ctrl+C to stop)
- `--fps N` — refresh rate for live mode (default 4)
- `-w N` — width in characters (defaults to terminal width)
- `-i` — invert for light backgrounds
- `--save PATH` — keep the captured photo

### Future directions
- Vision LLM integration (LLaVA or similar) to analyze or steer rendering
- Live mode refinements

## Project Structure

- `camera_ascii.py` — **primary script**, webcam-to-ASCII pipeline
- `ansi_to_hex.py` — parses RASCII ANSI output to hex colors, JSON, or HTML canvas
- `paint_berry.py` — paints ASCII art onto Berry Fast blockchain pixel board
- `src/` — Rust source for the RASCII CLI/library
- `reference-images/` — test portraits and generated visualizations

## Build

```bash
cargo build --release
```

The Python scripts shell out to `./target/release/rascii`.
