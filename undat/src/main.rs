use std::{fs::{self, File}, path::Path};

use clap::Parser;
use dat_entry::{DatEntry, DatEntryType};
use dat_footer::DatFooter;
use zune_inflate::DeflateDecoder;

mod dat_entry;
mod dat_footer;

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
    let data: Vec<u8> = fs::read(&args.input_filepath)?;
    println!("total file size: {}", data.len());
    let footer_data = &data[data.len() - DatFooter::LENGTH..];
    let footer = DatFooter::try_from(footer_data).unwrap();
    println!("footer: {:?}", footer);
    let filetable_start = data.len() - footer.dat_entry_start_from_end;
    println!(
        "filetable: {:#x?}",
        &data[filetable_start..filetable_start + 8]
    );
    let mut entry_start = filetable_start + 8;
    let mut entries = Vec::new();
    while entry_start < data.len() - DatFooter::LENGTH {
        let entry = DatEntry::try_from(&data[entry_start..]).unwrap();
        println!("entry: {:?}, len: {}", entry, entry.len());
        entry_start += entry.len();
        entries.push(entry);
    }

    println!("found {} entries", entries.len());

    for entry in entries {
        let path = Path::new(&args.output_filepath).join(&entry.filepath);
        match entry.entry_type {
            DatEntryType::Directory => {
                if !fs::exists(&path)? {
                    fs::create_dir_all(&path)?;
                }
            }
            DatEntryType::Compressed => {
                let mut decoder = DeflateDecoder::new(&data[entry.offset..entry.offset+entry.deflate_size]);
                let decompressed_data = decoder.decode_zlib().unwrap();
                fs::write(&path, decompressed_data)?;
            },
            DatEntryType::Stored => {
                fs::write(
                    &path,
                    &data[entry.offset..entry.offset + entry.original_size],
                )?;
            }
        }
    }

    Ok(())
}
