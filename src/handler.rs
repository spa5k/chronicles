// Enum containing various types of archive files, like ZIP, TAR, RAR

pub enum ArchiveType {
    Zip,
    Tar,
    Rar,
}

pub fn handler(r#type: &str) -> Result<ArchiveType, anyhow::Error> {
    match r#type {
        "zip" => Ok(ArchiveType::Zip),
        "xz" => Ok(ArchiveType::Tar),
        "tz" => Ok(ArchiveType::Tar),
        "gz" => Ok(ArchiveType::Tar),
        "rar" => Ok(ArchiveType::Rar),
        _ => Err(anyhow::anyhow!("Unknown file type")),
    }
}
