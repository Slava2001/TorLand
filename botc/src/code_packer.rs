use anyhow::{ensure, Context, Result};
use flate2::Compression;

use crate::code::Command;

const BASE32_ALP: base32::Alphabet = base32::Alphabet::Rfc4648 { padding: false };

pub fn from_b32(b32: &str) -> Result<Vec<Command>> {
    let compresed_bin = base32::decode(BASE32_ALP, b32).context("Failed to decode as base32")?;

    let mut decompressor = flate2::Decompress::new(false);
    // TODO: make the buffer size dynamic, now the hard size is 1 MB
    let mut bin = Vec::with_capacity(1024*1024);
    let status = decompressor
        .decompress_vec(&compresed_bin, &mut bin, flate2::FlushDecompress::Finish)
        .context("Failed to decompress")?;
    ensure!(
        status == flate2::Status::StreamEnd,
        "Some error while decompressing code: {:?}",
        status
    );

    Ok(bincode::deserialize(&bin).context("Failed to deserialize commands")?)
}

pub fn to_b32(cmds: &Vec<Command>) -> Result<String> {
    let bin = bincode::serialize(&cmds).context("Failed to serialize commands")?;
    let mut compressor = flate2::Compress::new(Compression::best(), false);
    let mut compresed_bin = Vec::with_capacity(bin.len());
    let status = compressor
        .compress_vec(&bin, &mut compresed_bin, flate2::FlushCompress::Finish)
        .context("Failed to compress")?;
    ensure!(
        status == flate2::Status::StreamEnd,
        "Some error while compressing code: {:?}",
        status
    );

    Ok(base32::encode(BASE32_ALP, &compresed_bin))
}
