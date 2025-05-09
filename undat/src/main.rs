use std::{fs, path::Path, time::Instant};

use clap::Parser;
use console::{Emoji, style};
use dat_entry::{DatEntry, DatEntryType};
use dat_footer::DatFooter;
use indicatif::{HumanDuration, ProgressBar};
use zune_inflate::DeflateDecoder;

mod dat_entry;
mod dat_footer;

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
static FOOTER: Emoji<'_, '_> = Emoji("🦶  ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");
static PARSER: Emoji<'_, '_> = Emoji("📋  ", "");
static EXTRACTING: Emoji<'_, '_> = Emoji("💾  ", "");

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Settings {
    #[arg(short, long)]
    input_filepath: String,
    #[arg(short, long)]
    output_filepath: String,
}

fn main() -> std::io::Result<()> {
    let started = Instant::now();
    let args = Settings::parse();
    println!(
        "{} {}Reading file...",
        style("[1/4]").bold().dim(),
        LOOKING_GLASS
    );
    let data: Vec<u8> = fs::read(&args.input_filepath)?;
    println!(
        "{} {}Extracting footer...",
        style("[2/4]").bold().dim(),
        FOOTER
    );
    let footer_data = &data[data.len() - DatFooter::LENGTH..];
    let footer = DatFooter::try_from(footer_data).unwrap();
    let filetable_start = data.len() - footer.dat_entry_start_from_end;

    let entry_count = u32::from_le_bytes(
        data[filetable_start..filetable_start + 4]
            .try_into()
            .unwrap(),
    );
    let mut entry_start = filetable_start + 8;
    let mut entries = Vec::new();
    println!(
        "{} {}Parsing file entries...",
        style("[3/4]").bold().dim(),
        PARSER
    );
    let progress_bar = ProgressBar::new(entry_count as u64);
    while entry_start < data.len() - DatFooter::LENGTH {
        let entry = DatEntry::try_from(&data[entry_start..]).unwrap();
        // println!("entry: {:?}, len: {}", entry, entry.len());
        entry_start += entry.len();
        entries.push(entry);
        progress_bar.inc(1);
    }
    progress_bar.finish_and_clear();

    println!(
        "{} {}Extracting files...",
        style("[4/4]").bold().dim(),
        EXTRACTING
    );
    let progress_bar = ProgressBar::new(entries.len() as u64);
    for entry in entries {
        let path = Path::new(&args.output_filepath).join(&entry.filepath);
        if let Some(parent) = path.parent() {
            if !fs::exists(parent)? {
                fs::create_dir_all(parent)?;
            }
        }
        match entry.entry_type {
            DatEntryType::Directory => {
                if !fs::exists(&path)? {
                    fs::create_dir_all(&path)?;
                }
            }
            DatEntryType::Compressed => {
                let mut decoder =
                    DeflateDecoder::new(&data[entry.offset..entry.offset + entry.deflate_size]);
                let decompressed_data = decoder.decode_zlib().unwrap();
                fs::write(&path, decompressed_data)?;
            }
            DatEntryType::Stored => {
                fs::write(
                    &path,
                    &data[entry.offset..entry.offset + entry.original_size],
                )?;
            }
        }
        progress_bar.inc(1);
    }
    progress_bar.finish_and_clear();

    println!("{} Done in {}", SPARKLE, HumanDuration(started.elapsed()));
    Ok(())
}
