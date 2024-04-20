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

struct IconData {
	size: u32,
	ostype: &'static str,
}

fn main() -> Result<()> {
	color_eyre::install()?;
	let args = Cli::parse();

	let input = ImageReader::open(&args.input)?.decode()?;
	if input.height() != input.width() {
		bail!("Image dimensions must be square.")
	}

	let mut family = IconFamily::new();

	for entry in [
		IconData {
			size: 16,
			ostype: "is32",
		},
		IconData {
			size: 32,
			ostype: "ic11",
		},
		IconData {
			size: 32,
			ostype: "il32",
		},
		IconData {
			size: 64,
			ostype: "ic12",
		},
		IconData {
			size: 128,
			ostype: "ic07",
		},
		IconData {
			size: 256,
			ostype: "ic13",
		},
		IconData {
			size: 256,
			ostype: "ic08",
		},
		IconData {
			size: 512,
			ostype: "ic14",
		},
		IconData {
			size: 512,
			ostype: "ic09",
		},
		IconData {
			size: 1024,
			ostype: "ic10",
		},
	] {
		let size = entry.size;
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

		family.add_icon_with_type(
			&image,
			IconType::from_ostype(entry.ostype.parse().unwrap()).unwrap(),
		)?;
	}

	let mut out_file = BufWriter::new(File::create(args.output.unwrap_or_else(|| {
		PathBuf::from(format!(
			"{}.icns",
			args.input.file_stem().unwrap().to_string_lossy()
		))
	}))?);
	family.write(&mut out_file)?;
	out_file.flush()?;

	Ok(())
}
