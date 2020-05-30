use std::fs;
use std::io::Write;

use flate2::Compression;
use flate2::write::ZlibEncoder;

use crate::formats::common::Pack;
use crate::formats::dcx;
use crate::repackers::errors::PackError;

/// Repack a previously unpacked DCX with this new data.
///
/// Params that are not well understood are reused, others are replaced
/// with accurate values.
pub fn pack_dcx(dcx: &mut dcx::Dcx, data: &[u8], output_path: &str) -> Result<(), PackError> {
    dcx.sizes.uncompressed_size = data.len() as u32;
    let compressed = compress(dcx, data)?;
    dcx.sizes.compressed_size = compressed.len() as u32;

    let mut output_file = fs::File::create(output_path)?;
    dcx.header.write(&mut output_file)?;
    dcx.sizes.write(&mut output_file)?;
    dcx.params.write(&mut output_file)?;
    dcx.archive.write(&mut output_file)?;
    output_file.write_all(&compressed)?;
    Ok(())
}

/// Compress data using DCX params.
pub fn compress(dcx: &dcx::Dcx, data: &[u8]) -> Result<Vec<u8>, PackError> {
    let method: &[u8] = dcx.params.method.as_slice();
    if method == b"DFLT" {
        compress_dflt(dcx, data)
    } else {
        let method_string = String::from_utf8_lossy(method).to_string();
        Err(PackError::Compression(format!("Method unknown: {}", method_string)))
    }
}

fn compress_dflt(dcx: &dcx::Dcx, data: &[u8]) -> Result<Vec<u8>, PackError> {
    let level = dcx.params.unk0C as u32;  // Unsure if it really is compression level.
    let half_size = data.len() / 2;  // Quicker allocation.
    let encoder = ZlibEncoder::new(Vec::with_capacity(half_size), Compression::new(level));
    Ok(encoder.finish()?)
}
