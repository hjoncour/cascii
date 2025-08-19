use anyhow::{anyhow, Context, Result};
use clap::Parser;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

/// Characters from darkest to lightest.
const ASCII_CHARS: &str = " .`'^,:;Il!i><~+_-?][}{1)(|/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

#[derive(Parser, Debug)]
#[command(version, about = "High-performance video/image to ASCII frame generator.")]
struct Args {
    /// Input video file or directory of images
    #[arg(short, long)]
    input: PathBuf,

    /// Output directory (will be created if missing)
    #[arg(short, long)]
    out: PathBuf,

    /// Target columns for scaling (width)
    #[arg(long, default_value_t = 800)]
    columns: u32,

    /// Frames per second when extracting from video
    #[arg(long, default_value_t = 30)]
    fps: u32,

    /// Font aspect ratio (character width:height). 0.5->tall, 1.0->square
    #[arg(long, default_value_t = 0.7)]
    font_ratio: f32,
}

fn run_ffmpeg_extract(input: &Path, out_dir: &Path, columns: u32, fps: u32) -> Result<()> {
    fs::create_dir_all(out_dir).context("creating frame_images dir")?;
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

fn convert_image_to_ascii(img_path: &Path, out_txt: &Path, font_ratio: f32, threshold: u8) -> Result<()> {
    let mut img = image::open(img_path).with_context(|| format!("opening {}", img_path.display()))?.to_rgb8();

    // Vertical squash to account for character aspect ratio
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

    let pb = ProgressBar::new(pngs.len() as u64);
    pb.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("##-"),
    );

    pngs
        .par_iter()
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

fn main() -> Result<()> {
    let args = Args::parse();

    fs::create_dir_all(&args.out).context("creating output dir")?;

    let frame_dir = args.out.join("frame_images");
    fs::create_dir_all(&frame_dir)?;

    if args.input.is_file() {
        // Extract frames from video first
        run_ffmpeg_extract(&args.input, &frame_dir, args.columns, args.fps)?;
        // Convert extracted PNGs to ASCII in parallel
        convert_dir_pngs_parallel(&frame_dir, &frame_dir, args.font_ratio, 1)?;
    } else if args.input.is_dir() {
        // Convert existing PNGs in a directory
        convert_dir_pngs_parallel(&args.input, &frame_dir, args.font_ratio, 1)?;
    } else {
        return Err(anyhow!("input path does not exist"));
    }

    println!("ASCII generation complete in {}", frame_dir.display());
    Ok(())
}


