use std::path::Path;

use extractor::{gzip::extract_gzip, tar::extract_tar, zip::extract_zip};

mod extractor;
mod handler;

/// Extracts an archive to the given path..// Returns Ok() if there is no error while extracting
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
pub fn extract(path: &Path, to: &Path) -> Result<(), anyhow::Error> {
    // let mut file = File::open(path)?;

    // Get extension of the file
    let extension = path.extension().unwrap().to_str().unwrap();

    // Get the mime type of the file
    let extension_type = handler::handler(extension);

    let file = match extension_type {
        Ok(handler::ArchiveType::Zip) => extract_zip(path, to),
        Ok(handler::ArchiveType::Tar) => extract_tar(path, to),
        Ok(handler::ArchiveType::Gzip) => extract_gzip(path, to),
        Err(e) => Err(e)?,
    };
    match file {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use std::path;

    use crate::extract;

    #[test]
    fn test_zip() {
        let file = path::Path::new("idm.zip");
        let to = path::Path::new("./extracted");
        let res = extract(file, to);
        match res {
            Ok(_) => {
                println!("Extracted successfully");
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    #[test]
    fn test_tar() {
        let file = path::Path::new("18.3.0.tar.xz");
        let to = path::Path::new("./extracted");
        let res = extract(file, to);
        match res {
            Ok(_) => {
                println!("Extracted successfully");
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    #[test]
    fn test_gzip() {
        let file = path::Path::new("node.tar.gz");
        let to = path::Path::new("./extracted");
        let res = extract(file, to);
        match res {
            Ok(_) => {
                println!("Extracted successfully");
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
