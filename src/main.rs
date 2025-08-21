use anyhow::{anyhow, Context, Result};
use clap::Parser;
use dialoguer::{Confirm, FuzzySelect, Input};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

/// Characters from darkest to lightest.
const ASCII_CHARS: &str = " .`'^,:;Il!i><~+_-?][}{1)(|/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

#[derive(Parser, Debug)]
#[command(version, about = "Interactive video/image to ASCII frame generator.")]
struct Args {
    /// Input video file or directory of images
    input: Option<PathBuf>,

    /// Output directory for the generated files
    out: Option<PathBuf>,

    /// Target columns for scaling (width)
    #[arg(long)]
    columns: Option<u32>,

    /// Frames per second when extracting from video
    #[arg(long)]
    fps: Option<u32>,

    /// Font aspect ratio (character width:height)
    #[arg(long)]
    font_ratio: Option<f32>,

    /// Use default quality preset
    #[arg(long, default_value_t = false, conflicts_with_all = &["small", "large"])]
    default: bool,

    /// Use smaller default values for quality settings
    #[arg(long, short, default_value_t = false, conflicts_with_all = &["default", "large"])]
    small: bool,

    /// Use larger default values for quality settings
    #[arg(long, short, default_value_t = false, conflicts_with_all = &["default", "small"])]
    large: bool,

    /// Luminance threshold (0-255) for what is considered transparent
    #[arg(long)]
    luminance: Option<u8>,
}

fn main() -> Result<()> {
    let mut args = Args::parse();
    let is_interactive = !(args.default || args.small || args.large);

    // --- Interactive Prompts ---
    if args.input.is_none() {
        if !is_interactive {
            return Err(anyhow!("Input file must be provided when using a preset."));
        }
        let files = find_media_files()?;
        if files.is_empty() {
            return Err(anyhow!("No media files found in current directory."));
        }
        let selection = FuzzySelect::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt("Choose an input file")
            .default(0)
            .items(&files)
            .interact()?;
        args.input = Some(PathBuf::from(&files[selection]));
    }

    let input_path = args.input.unwrap();

    let output_path = args.out.unwrap_or_else(|| PathBuf::from("."));

    // Quality defaults based on flags
    let (default_cols, default_fps, default_ratio) = if args.small {
        (80, 24, 0.44)
    } else if args.large {
        (800, 60, 0.7)
    } else if args.default {
        (200, 24, 0.5)
    } else {
        (800, 30, 0.7)
    };

    if args.columns.is_none() && is_interactive {
        args.columns = Some(
            Input::new()
                .with_prompt("Columns (width)")
                .default(default_cols)
                .interact()?,
        );
    }

    if input_path.is_file() && args.fps.is_none() && is_interactive {
        args.fps = Some(
            Input::new()
                .with_prompt("Frames per second (FPS)")
                .default(default_fps)
                .interact()?,
        );
    }

    if args.font_ratio.is_none() && is_interactive {
        args.font_ratio = Some(
            Input::new()
                .with_prompt("Font Ratio")
                .default(default_ratio)
                .interact()?,
        );
    }

    if args.luminance.is_none() && is_interactive {
        args.luminance = Some(
            Input::new()
                .with_prompt("Luminance threshold")
                .default(1u8)
                .interact()?,
        );
    }

    let columns = args.columns.unwrap_or(default_cols);
    let fps = args.fps.unwrap_or(default_fps);
    let font_ratio = args.font_ratio.unwrap_or(default_ratio);
    let luminance = args.luminance.unwrap_or(1);

    // --- Execution ---
    fs::create_dir_all(&output_path).context("creating output dir")?;

    let frame_dir = output_path.join("frame_images");
    if frame_dir.exists() {
        if !is_interactive
            || Confirm::new()
                .with_prompt(format!(
                    "Directory {} already exists. Overwrite?",
                    frame_dir.display()
                ))
                .default(false)
                .interact()?
        {
            fs::remove_dir_all(&frame_dir)?;
        } else {
            println!("Operation cancelled.");
            return Ok(());
        }
    }
    fs::create_dir_all(&frame_dir)?;

    if input_path.is_file() {
        run_ffmpeg_extract(&input_path, &frame_dir, columns, fps)?;
        convert_dir_pngs_parallel(&frame_dir, &frame_dir, font_ratio, luminance)?;
    } else if input_path.is_dir() {
        convert_dir_pngs_parallel(&input_path, &frame_dir, font_ratio, luminance)?;
    } else {
        return Err(anyhow!("Input path does not exist"));
    }

    println!("\nASCII generation complete in {}", frame_dir.display());

    // --- Create details.txt ---
    let frame_count = WalkDir::new(&frame_dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "txt"))
        .count();

    let mut details = format!(
        "Frames: {}\nLuminance: {}\nFont Ratio: {}\nColumns: {}",
        frame_count, luminance, font_ratio, columns
    );

    if input_path.is_file() {
        details.push_str(&format!("\nFPS: {}", fps));
    }

    let details_path = output_path.join("details.md");
    fs::write(details_path, details).context("writing details file")?;
    
    Ok(())
}

fn find_media_files() -> Result<Vec<String>> {
    Ok(WalkDir::new(".")
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().is_file()
                && e.path()
                    .extension()
                    .map_or(false, |ext| matches!(ext.to_str(), Some("mp4" | "mkv" | "mov" | "avi" | "webm" | "png" | "jpg")))
        })
        .map(|e| e.path().to_str().unwrap_or("").to_string())
        .collect())
}

fn run_ffmpeg_extract(input: &Path, out_dir: &Path, columns: u32, fps: u32) -> Result<()> {
    println!("Extracting frames with ffmpeg...");
    let out_pattern = out_dir.join("frame_%04d.png");
    let status = Command::new("ffmpeg")
        .args([
            "-loglevel",
            "error",
            "-i",
            input.to_str().unwrap(),
            "-vf",
            &format!("scale={}:-2,fps={}", columns, fps),
            out_pattern.to_str().unwrap(),
        ])
        .status()
        .context("running ffmpeg")?;
    if !status.success() {
        return Err(anyhow!("ffmpeg failed"));
    }
    Ok(())
}

fn convert_dir_pngs_parallel(src_dir: &Path, dst_dir: &Path, font_ratio: f32, threshold: u8) -> Result<()> {
    fs::create_dir_all(dst_dir)?;
    let mut pngs: Vec<PathBuf> = WalkDir::new(src_dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.into_path())
        .filter(|p| p.extension().map(|e| e == "png").unwrap_or(false))
        .collect();
    pngs.sort();

    println!("Converting {} images to ASCII...", pngs.len());
    let pb = ProgressBar::new(pngs.len() as u64);
    pb.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("##-"),
    );

    pngs.par_iter()
        .progress_with(pb)
        .try_for_each(|img_path| -> Result<()> {
            let file_stem = img_path
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or_else(|| anyhow!("bad file name"))?;
            let out_txt = dst_dir.join(format!("{}.txt", file_stem));
            convert_image_to_ascii(img_path, &out_txt, font_ratio, threshold)
        })?;

    Ok(())
}

fn convert_image_to_ascii(img_path: &Path, out_txt: &Path, font_ratio: f32, threshold: u8) -> Result<()> {
    let mut img = image::open(img_path).with_context(|| format!("opening {}", img_path.display()))?.to_rgb8();
    let (w, h) = img.dimensions();
    let new_h = ((h as f32) * font_ratio).max(1.0).round() as u32;
    if new_h != h {
        img = image::imageops::resize(&img, w, new_h, image::imageops::FilterType::Triangle);
    }

    let mut out = String::with_capacity((w as usize + 1) * (new_h as usize));
    for y in 0..new_h {
        for x in 0..w {
            let px = img.get_pixel(x, y);
            let l = luminance(*px);
            out.push(char_for(l, threshold));
        }
        out.push('\n');
    }
    fs::write(out_txt, out).with_context(|| format!("writing {}", out_txt.display()))?;
    Ok(())
}

fn luminance(rgb: image::Rgb<u8>) -> u8 {
    let r = rgb[0] as f32;
    let g = rgb[1] as f32;
    let b = rgb[2] as f32;
    (0.2126 * r + 0.7152 * g + 0.0722 * b).round() as u8
}

fn char_for(luma: u8, threshold: u8) -> char {
    if luma < threshold {
        return ' ';
    }
    let chars = ASCII_CHARS.as_bytes();
    let idx = (((luma.saturating_sub(threshold)) as f32 / (255u16.saturating_sub(threshold as u16) as f32))
        * ((chars.len() - 1) as f32))
        .clamp(0.0, (chars.len() - 1) as f32)
        as usize;
    chars[idx] as char
}
