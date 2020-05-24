use std::fs;
use std::io::{self, Write};
use std::path;

use crate::formats::common::Pack;
use crate::formats::dat;
use crate::utils::bin as utils_bin;
use crate::utils::fs as utils_fs;

/// Pack a directory as a DAT archive.
///
/// Walks recursively in `files_path` to build all file entries.
/// For performance and laziness, the archive is built directly in RAM.
pub fn pack_dat(files_path: &str, output_path: &str) -> Result<(), io::Error> {
    // Pack all files and entries description in memory.
    let files_path = path::Path::new(files_path);
    let mut entries = vec!();
    let mut files_data = vec!();
    pack_dat_dir(files_path, "", &mut entries, &mut files_data)?;

    let mut output_file = fs::File::create(output_path)?;
    let mut ofs = 0usize;

    // Write header.
    let header = dat::DatHeader { unk00: dat::MAGIC, num_files: entries.len() as u32 };
    header.write(&mut output_file)?;
    output_file.write_all(&vec![0u8; dat::HEADER_PAD])?;
    ofs += dat::HEADER_SIZE;

    // Write entries, but shift their data offset beforehand.
    let entries_size = entries.len() * dat::FILE_ENTRY_SIZE;
    let entries_pad = utils_bin::pad(ofs + entries_size, dat::DATA_ALIGN);
    let ofs_data = ofs + entries_size + entries_pad;
    for entry in &mut entries {
        entry.ofs_data += ofs_data as u32;
        entry.write(&mut output_file)?;
    }
    output_file.write_all(&vec![0u8; entries_pad])?;

    // Finally, write files data.
    output_file.write_all(&files_data)?;

    Ok(())
}

/// Recursively walks in `dir` to create `DatFileEntry`s.
///
/// `prefix` is initially "" and will contain current relative dir with
/// separator suffixed during walks, e.g. "param/".
fn pack_dat_dir(
    dir: &path::Path,
    prefix: &str,
    entries: &mut Vec<dat::DatFileEntry>,
    files_data: &mut Vec<u8>,
) -> Result<(), io::Error> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?.path();
        if entry.is_dir() {
            if let Some(dir_name) = entry.file_name().and_then(|n| n.to_str()) {
                let mut prefix = String::from(prefix);
                prefix.push_str(dir_name);
                prefix.push(dat::INTERNAL_PATH_SEP);
                pack_dat_dir(&entry, &prefix, entries, files_data)?;
            }
        } else if entry.is_file() /* No symlink support. */ {
            if let Some(name) = entry.file_name().and_then(|n| n.to_str()) {
                if let Ok(metadata) = entry.metadata() {
                    let mut entry_name = String::from(prefix);
                    entry_name.push_str(name);
                    pack_dat_entry(&entry, entry_name, &metadata, entries, files_data)?;
                }
            }
        }
    }
    Ok(())
}

/// Pack the file in `files_data` and update `entries` accordingly.
fn pack_dat_entry(
    file_entry: &path::PathBuf,
    internal_name: String,
    metadata: &fs::Metadata,
    entries: &mut Vec<dat::DatFileEntry>,
    files_data: &mut Vec<u8>,
) -> Result<(), io::Error> {
    let file_size = metadata.len() as u32;
    let padding = utils_bin::pad(file_size as usize, dat::DATA_ALIGN);
    entries.push(dat::DatFileEntry {
        name: internal_name,
        size: file_size,
        padded_size: file_size + padding as u32,
        ofs_data: files_data.len() as u32,  // Data will be pushed at the current end of file.
    });

    let mut data = utils_fs::open_file_to_vec(file_entry)?;
    files_data.append(&mut data);
    let mut padding_data = vec![0u8; padding];
    files_data.append(&mut padding_data);
    Ok(())
}
