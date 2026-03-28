#!/usr/bin/env python3
"""Capture a webcam frame and render it as colored ASCII art via RASCII.

Usage:
    python camera_ascii.py                  # snapshot from webcam
    python camera_ascii.py --live           # continuous live view
    python camera_ascii.py photo.jpg        # render a file
    python camera_ascii.py -b -C block      # pixel-art style
"""

import argparse
import os
import subprocess
import sys
import tempfile
import time

import cv2


def terminal_width():
    try:
        return os.get_terminal_size().columns
    except OSError:
        return 80


def render(image_path, width=None, color=True, background=False,
           invert=False, charset=None):
    """Run RASCII on an image file. Returns the ANSI string."""
    cmd = ["./target/release/rascii", image_path, "-w", str(width or terminal_width())]
    if color:
        cmd.append("-c")
    if background:
        cmd.append("-b")
    if invert:
        cmd.append("-i")
    if charset:
        cmd.extend(["-C", charset])
    result = subprocess.run(cmd, capture_output=True, text=True)
    if result.returncode != 0:
        print(f"RASCII error: {result.stderr}", file=sys.stderr)
        return ""
    return result.stdout


def main():
    parser = argparse.ArgumentParser(
        description="Webcam to ASCII art via RASCII",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="Examples:\n"
               "  %(prog)s                     Snapshot from webcam\n"
               "  %(prog)s --live              Live camera view (Ctrl+C to stop)\n"
               "  %(prog)s --live --fps 8      Faster refresh\n"
               "  %(prog)s -b -C block         Pixel-art style\n"
               "  %(prog)s photo.jpg           Render an image file\n"
               "  %(prog)s --save snap.jpg     Snapshot and keep the photo\n")
    parser.add_argument("image", nargs="?",
                        help="Image file to render (uses webcam if omitted)")
    parser.add_argument("-w", "--width", type=int,
                        help="Width of ASCII output (default: terminal width)")
    parser.add_argument("--camera", type=int, default=0,
                        help="Camera device index (default: 0)")
    parser.add_argument("--save", metavar="PATH",
                        help="Save the captured frame to this path")
    parser.add_argument("-b", "--background", action="store_true",
                        help="Use background color highlighting")
    parser.add_argument("-i", "--invert", action="store_true",
                        help="Invert character weights (for light backgrounds)")
    parser.add_argument("-C", "--charset",
                        help="Character set: block, emoji, default, russian, slight")
    parser.add_argument("--no-color", action="store_true",
                        help="Disable color output")
    parser.add_argument("--no-mirror", action="store_true",
                        help="Disable horizontal flip (webcam mirrors by default)")
    parser.add_argument("--live", action="store_true",
                        help="Continuous live view (Ctrl+C to stop)")
    parser.add_argument("--fps", type=float, default=4,
                        help="Target frames per second in live mode (default: 4)")
    args = parser.parse_args()

    rargs = dict(
        width=args.width or terminal_width(),
        color=not args.no_color,
        background=args.background,
        invert=args.invert,
        charset=args.charset,
    )

    # Static image file mode
    if args.image:
        print(render(args.image, **rargs), end="")
        return

    # --- Camera modes ---
    cap = cv2.VideoCapture(args.camera)
    if not cap.isOpened():
        print(f"Error: could not open camera {args.camera}", file=sys.stderr)
        sys.exit(1)

    # Warmup: let auto-exposure settle
    for _ in range(30):
        cap.read()

    mirror = not args.no_mirror
    fd, tmp_path = tempfile.mkstemp(suffix=".jpg")

    try:
        if args.live:
            _live_loop(cap, tmp_path, mirror, args.fps, rargs)
        else:
            _single_shot(cap, tmp_path, mirror, args.save, rargs)
    except KeyboardInterrupt:
        pass
    finally:
        cap.release()
        os.close(fd)
        os.unlink(tmp_path)
        if args.live:
            sys.stdout.write("\033[?25h\n")  # restore cursor
            sys.stdout.flush()


def _grab(cap, tmp_path, mirror):
    """Read a frame, optionally mirror it, write to tmp file. Returns the frame."""
    ret, frame = cap.read()
    if not ret:
        print("Error: could not read frame", file=sys.stderr)
        sys.exit(1)
    if mirror:
        frame = cv2.flip(frame, 1)
    cv2.imwrite(tmp_path, frame)
    return frame


def _single_shot(cap, tmp_path, mirror, save_path, rargs):
    frame = _grab(cap, tmp_path, mirror)
    if save_path:
        cv2.imwrite(save_path, frame)
        print(f"Saved frame to {save_path}", file=sys.stderr)
    print(render(tmp_path, **rargs), end="")


def _live_loop(cap, tmp_path, mirror, fps, rargs):
    interval = 1.0 / fps
    sys.stdout.write("\033[?25l\033[2J")  # hide cursor, clear screen
    sys.stdout.flush()
    while True:
        t0 = time.monotonic()
        _grab(cap, tmp_path, mirror)
        output = render(tmp_path, **rargs)
        sys.stdout.write("\033[H")  # cursor home (overwrites in place)
        sys.stdout.write(output)
        sys.stdout.write("\033[J")  # clear any leftover lines below
        sys.stdout.flush()
        elapsed = time.monotonic() - t0
        if elapsed < interval:
            time.sleep(interval - elapsed)


if __name__ == "__main__":
    main()
