use std::{fs, io, path::Path};

/// Extracts a zip  archive to the given path..// Returns Ok() if there is no error while extracting
/// or Err() if there is.
/// # Arguments
/// * `file` - Path to the archive
/// * `to` - Path to the destination directory
/// # Example
/// ```
/// use chronicles::extract;
/// let file = Path::new("./archive.zip");
/// let to = Path::new("./extracted");
/// extract(file, to);
/// ```
pub fn extract_zip(path: &Path, to: &Path) -> Result<(), anyhow::Error> {
    let file = fs::File::open(&path).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        // Join outpath to to
        let outpath = to.join(outpath);

        {
            let comment = file.comment();
            if !comment.is_empty() {}
        }

        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    Ok(())
}
