use clap::Parser;
use color_eyre::eyre::{bail, Result};

use icns::{IconFamily, IconType};
use image::{
	codecs::png::{CompressionType, FilterType as PngFilterType, PngEncoder},
	imageops::FilterType,
	io::Reader as ImageReader,
	ColorType, ImageEncoder,
};
use std::{
	fs::File,
	io::{BufWriter, Write},
	path::PathBuf,
};

#[derive(Parser)]
struct Cli {
	input: PathBuf,

	#[clap(short, long)]
	output: Option<PathBuf>,
}

const ICNS: &[(&str, u32)] = &[
	("is32", 16),
	("ic11", 32),
	("il32", 32),
	("ic12", 64),
	("ic07", 128),
	("ic13", 256),
	("ic08", 256),
	("ic14", 512),
	("ic09", 512),
	("ic10", 1024),
];

fn generate_icon(input: &image::DynamicImage, size: u32) -> Result<icns::Image> {
	let mut buf = Vec::new();
	let encoder =
		PngEncoder::new_with_quality(&mut buf, CompressionType::Best, PngFilterType::Adaptive);
	encoder.write_image(
		input
			.resize_exact(size, size, FilterType::Lanczos3)
			.as_bytes(),
		size,
		size,
		ColorType::Rgba8.into(),
	)?;

	let image = icns::Image::read_png(&buf[..])?;
	Ok(image)
}

fn main() -> Result<()> {
	color_eyre::install()?;
	let args = Cli::parse();

	let input = ImageReader::open(&args.input)?.decode()?;
	if input.height() != input.width() {
		bail!("Image dimensions must be square.")
	}

	let mut family = IconFamily::new();

	for &(ostype, size) in ICNS {
		let icon = generate_icon(&input, size)?;
		family.add_icon_with_type(
			&icon,
			IconType::from_ostype(ostype.parse().unwrap()).unwrap(),
		)?;
	}

	let mut output = BufWriter::new(File::create(args.output.unwrap_or_else(|| {
		let name = args.input.file_stem().unwrap().to_string_lossy();
		PathBuf::from(format!("{}.icns", name))
	}))?);
	family.write(&mut output)?;
	output.flush()?;

	Ok(())
}
