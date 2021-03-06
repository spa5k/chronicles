use std::{fs, path::Path};
use tar::Archive;
use xz2::read;

/// Extracts a tar xz  archive to the given path..// Returns Ok() if there is no error while extracting
/// or Err() if there is.
/// # Arguments
/// * `file` - Path to the archive
/// * `to` - Path to the destination directory
/// # Example
/// ```
/// use chronicles::extract;
/// let file = Path::new("./archive.tar.xz");
/// let to = Path::new("./extracted");
/// extract(file, to);
/// ```
pub fn extract_tar(path: &Path, to: &Path) -> Result<(), anyhow::Error> {
    let file = fs::File::open(&path).unwrap();
    let xz_stream = read::XzDecoder::new(file);
    let mut tar_archive = Archive::new(xz_stream);
    tar_archive.unpack(&to)?;

    Ok(())
}
