// Enum containing various types of archive files, like ZIP, TAR, RAR

pub enum ArchiveType {
    Zip,
    Tar,
    Gzip,
}

pub fn handler(r#type: &str) -> Result<ArchiveType, anyhow::Error> {
    match r#type {
        "zip" => Ok(ArchiveType::Zip),
        "xz" => Ok(ArchiveType::Tar),
        "tz" => Ok(ArchiveType::Tar),
        "gz" => Ok(ArchiveType::Gzip),
        _ => Err(anyhow::anyhow!("Unknown file type")),
    }
}
