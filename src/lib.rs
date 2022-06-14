use std::path::Path;

use extractor::{rar::extract_rar, tar::extract_tar, zip::extract_zip};

mod extractor;
mod handler;

pub fn extract(path: &Path, to: &Path) -> Result<(), anyhow::Error> {
    // let mut file = File::open(path)?;

    // Get extension of the file
    let extension = path.extension().unwrap().to_str().unwrap();
    println!("Extension: {}", extension);

    // Get the mime type of the file
    let extension_type = handler::handler(extension);

    let file = match extension_type {
        Ok(handler::ArchiveType::Zip) => extract_zip(path, to),
        Ok(handler::ArchiveType::Tar) => extract_tar(path, to),
        Ok(handler::ArchiveType::Rar) => extract_rar(path, to),
        Err(e) => Err(e)?,
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path;

    use crate::extract;

    #[test]
    fn test_zip() {
        // Path to hitmo.zip
        let file = path::Path::new("idm.zip");
        // Path to extract to
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
        // Path to hitmo.tar
        let file = path::Path::new("18.3.0.tar.xz");
        // Path to extract to
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
