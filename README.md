# cascii

rust cli to convert stuff to make ascii art

# casci - Interactive ASCII Frame Generator

`casci` is a high-performance, interactive tool for converting videos and image sequences into ASCII art frames.

## Features

- **Interactive Mode**: If you don't provide arguments, `casci` will prompt you for them.
- **Flexible Input**: Works with video files or directories of PNGs.
- **Performance**: Uses `ffmpeg` for fast frame extraction and parallel processing with Rayon for ASCII conversion.
- **Presets**: `--small` and `--large` flags for quick quality adjustments.
- **Non-interactive Mode**: Use `--default` to run without prompts, using default values.

## Installation

An `install.sh` script is provided to build and install `casci` to `/usr/local/bin`.

```bash
# Make sure you are in the casci directory
./install.sh
```

You will be prompted for your password as it uses `sudo` to copy the binary.

## Usage

### Interactive

Run `casci` without any arguments to be guided through the process:

```bash
casci
```

It will first ask you to select an input file from the current directory, then prompt for the output directory, and finally for the quality settings.

### With Arguments

You can also provide arguments directly:

```bash
# Basic usage with a video file
casci my_video.mp4 --out ./my_frames

# Using presets
casci my_video.mp4 --out ./my_frames --large

# Non-interactive mode (will fail if input is not provided)
casci my_video.mp4 --out ./my_frames --default
```

### Options

- `[input]`: (Optional) The input video file or directory of images.
- `-o`, `--out`: (Optional) The output directory. Defaults to the current directory.
- `--columns`: (Optional) The width of the output ASCII art.
- `--fps`: (Optional) The frames per second to extract from a video.
- `--font-ratio`: (Optional) The aspect ratio of the font used for rendering.
- `--default`: Skips all prompts and uses default values for any missing arguments.
- `-s`, `--small`: Uses smaller default values for quality settings.
- `-l`, `--large`: Uses larger default values for quality settings.
- `-h`, `--help`: Shows the help message.
- `-V`, `--version`: Shows the version information.


# Test Image

./target/release/ascii-gen \
  --input ./some_frames_dir \
  --out ../public/sunset_hl \
  --font-ratio 0.7

# Test Video

./target/release/ascii-gen \
  --input ../input.webm \
  --out ../public/sunset_hl \
  --columns 800 \
  --fps 30 \
  --font-ratio 0.7
