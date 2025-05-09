use std::{ffi::OsStr, io::Error, path::Path};

use art_file::ArtFile;
use clap::Parser;

mod art_file;
mod art_header;
mod artconverter_error;
mod color;
mod color_table;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Settings {
    #[arg(short, long)]
    input_filepath: String,
    #[arg(short, long)]
    output_filepath: String,
}

fn main() -> std::io::Result<()> {
    let args = Settings::parse();
    let input_filepath = Path::new(&args.input_filepath);
    match input_filepath.extension().and_then(OsStr::to_str) {
        Some("ART") => {
            let art_file = ArtFile::load_from_file(args.input_filepath).unwrap();
            art_file.save_as_bmp(args.output_filepath);
        }
        Some("bmp") => {}
        _ => {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "must be .ART or .bmp",
            ));
        }
    };

    Ok(())
}
