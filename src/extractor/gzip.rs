use flate2::read::GzDecoder;
use std::{fs, path::Path};
use tar::Archive;

/// Extracts a gzip archive to the given path.
/// Returns Ok() if there is no error while extracting
/// or Err() if there is.
/// # Arguments
/// * `file` - Path to the archive
/// * `to` - Path to the destination directory
/// # Example
/// ```
/// use chronicles::extract;
/// let file = Path::new("./archive.tar.gz");
/// let to = Path::new("./extracted");
/// extract(file, to);
/// ```
pub fn extract_gzip(path: &Path, to: &Path) -> Result<(), anyhow::Error> {
    let file = fs::File::open(&path).unwrap();
    let gz = GzDecoder::new(file);
    let mut tar_archive = Archive::new(gz);
    tar_archive.unpack(&to)?;

    Ok(())
}
