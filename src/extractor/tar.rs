use std::{fs, path::Path};
use tar::Archive;

pub fn extract_tar(path: &Path, to: &Path) -> Result<(), anyhow::Error> {
    let file = fs::File::open(&path).unwrap();

    let mut archive = tar::Archive::new(file);

    for file in archive.entries().unwrap() {
        // Make sure there wasn't an I/O error
        let mut file = file.unwrap();

        // Inspect metadata about the file
        println!("{:?}", file.header().path().unwrap());
        println!("{}", file.header().size().unwrap());
        println!("{}", file.path().unwrap().display());
    }

    Ok(())
}
