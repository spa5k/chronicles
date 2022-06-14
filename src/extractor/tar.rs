use std::{fs, path::Path};
use tar::Archive;
use xz2::read;

pub fn extract_tar(path: &Path, to: &Path) -> Result<(), anyhow::Error> {
    let file = fs::File::open(&path).unwrap();
    let xz_stream = read::XzDecoder::new(file);
    let mut tar_archive = Archive::new(xz_stream);
    tar_archive.unpack(&to)?;

    Ok(())
}
