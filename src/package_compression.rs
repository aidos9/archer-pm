use crate::error::{APMError, APMErrorType};

use sha2::{Digest, Sha256};
use std::fs::{File, OpenOptions};
use std::io::{Cursor, Read, Write};
use walkdir::WalkDir;
use zip::write::FileOptions;
use zip::{ZipArchive, ZipWriter};

const HIDDEN_FILE_PATH: &'static str = "/hidden";

pub fn remove_checksum_zip(path: &str) -> Result<(Vec<u8>, bool), APMError> {
    let mut archive = read_archive(path)?;
    let options = zip::write::FileOptions::default();

    let mut output = Vec::new();
    let mut zip_writer = ZipWriter::new(Cursor::new(&mut output));
    let mut checksum_removed = false;

    for i in 0..archive.len() {
        let mut f = archive
            .by_index(i)
            .map_err(|e| APMErrorType::ZIPArchiveReadError.into_apm_error(e.to_string()))?;

        if f.name() != HIDDEN_FILE_PATH {
            if f.is_dir() {
                zip_writer.add_directory(f.name(), options).map_err(|e| {
                    APMErrorType::ZIPAddDirectoryError.into_apm_error(e.to_string())
                })?;
            } else if f.is_file() {
                zip_writer
                    .start_file(f.name(), options)
                    .map_err(|e| APMErrorType::ZIPStartFileError.into_apm_error(e.to_string()))?;
                let mut buf = Vec::new();

                f.read_to_end(&mut buf)
                    .map_err(|e| APMErrorType::ZIPFileReadError.into_apm_error(e.to_string()))?;

                zip_writer
                    .write_all(&mut buf)
                    .map_err(|e| APMErrorType::ZIPFileWriteError.into_apm_error(e.to_string()))?;
            }
        } else {
            checksum_removed = true;
        }
    }

    zip_writer
        .finish()
        .map_err(|e| APMErrorType::ZIPFinishError.into_apm_error(e.to_string()))?;

    drop(zip_writer);

    return Ok((output, checksum_removed));
}

fn read_archive(path: &str) -> Result<ZipArchive<File>, APMError> {
    let mut f = OpenOptions::new()
        .read(true)
        .write(false)
        .open(path)
        .map_err(|e| APMErrorType::FileOpenError.into_apm_error(e.to_string()))?;

    return Ok(ZipArchive::new(f)
        .map_err(|e| APMErrorType::ZIPArchiveOpenError.into_apm_error(e.to_string()))?);
}

pub fn insert_checksum_zip(
    path: &str,
    remove_checksum: bool,
) -> Result<(Vec<u8>, String), APMError> {
    let mut contents;

    if remove_checksum {
        (contents, _) = remove_checksum_zip(path)?;
    } else {
        contents = Vec::new();

        let mut f = OpenOptions::new()
            .read(true)
            .write(false)
            .open(path)
            .map_err(|e| APMErrorType::FileOpenError.into_apm_error(e.to_string()))?;

        f.read_to_end(&mut contents)
            .map_err(|e| APMErrorType::FileReadError.into_apm_error(e.to_string()))?;
    }

    return add_checksum_zip(contents);
}

pub fn add_checksum_zip(mut contents: Vec<u8>) -> Result<(Vec<u8>, String), APMError> {
    let options = zip::write::FileOptions::default();

    let hash_bytes = generate_archer_hash_from_bytes(&contents);
    let hash_string = base64::encode(hash_bytes);

    let mut zip_writer = ZipWriter::new_append(Cursor::new(&mut contents))
        .map_err(|e| APMErrorType::ZIPOpenError.into_apm_error(e.to_string()))?;

    zip_writer
        .start_file(HIDDEN_FILE_PATH, options)
        .map_err(|e| APMErrorType::ZIPModificationError.into_apm_error(e.to_string()))?;

    zip_writer
        .write_all(hash_string.as_bytes())
        .map_err(|e| APMErrorType::ZIPModificationError.into_apm_error(e.to_string()))?;

    zip_writer
        .finish()
        .map_err(|e| APMErrorType::ZIPFinishError.into_apm_error(e.to_string()))?;

    drop(zip_writer);

    return Ok((contents, hash_string));
}

pub fn dump_file_names_zip(path: &str) -> Result<Vec<String>, APMError> {
    let mut f = OpenOptions::new()
        .read(true)
        .write(false)
        .open(path)
        .map_err(|e| APMErrorType::FileOpenError.into_apm_error(e.to_string()))?;

    let mut contents = Vec::new();

    f.read_to_end(&mut contents)
        .map_err(|e| APMErrorType::FileReadError.into_apm_error(e.to_string()))?;

    return dump_file_names_zip_bytes(&contents);
}

pub fn dump_file_names_zip_bytes(zip_bytes: &[u8]) -> Result<Vec<String>, APMError> {
    let zip_archive = ZipArchive::new(Cursor::new(zip_bytes))
        .map_err(|e| APMErrorType::ZIPArchiveOpenError.into_apm_error(e.to_string()))?;

    return Ok(zip_archive.file_names().map(|s| s.to_string()).collect());
}

pub fn dump_archer_hash_zip_file(zip_bytes: &[u8]) -> Result<String, APMError> {
    let mut zip_archive = ZipArchive::new(Cursor::new(zip_bytes))
        .map_err(|e| APMErrorType::ZIPArchiveOpenError.into_apm_error(e.to_string()))?;
    let mut zip_file = zip_archive
        .by_name(HIDDEN_FILE_PATH)
        .map_err(|e| APMErrorType::ZIPArchiveHiddenNotFoundError.into_apm_error(e.to_string()))?;

    let mut hash = Vec::new();

    zip_file
        .read_to_end(&mut hash)
        .map_err(|e| APMErrorType::ZIPArchiveReadError.into_apm_error(e.to_string()))?;

    return String::from_utf8(hash)
        .map_err(|e| APMErrorType::HashUTF8Error.into_apm_error(e.to_string()));
}

fn generate_archer_hash_from_bytes(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();

    hasher.update(bytes);

    return hasher.finalize().into();
}

pub fn compress_directory(
    path: &str,
    track_file_names: bool,
) -> Result<(Vec<u8>, Option<Vec<String>>), APMError> {
    let mut buffer = Vec::new();
    let options = FileOptions::default();
    let mut zip_writer = ZipWriter::new(Cursor::new(&mut buffer));
    let mut file_names = {
        if track_file_names {
            Some(Vec::new())
        } else {
            None
        }
    };

    for entry in WalkDir::new(path).into_iter() {
        let entry = entry.map_err(|e| APMErrorType::WalkdirError.into_apm_error(e.to_string()))?;

        let name = entry.path().display().to_string();

        // Skip the current directory
        if name == path {
            continue;
        }

        if entry.file_type().is_symlink() {
            return Err(APMErrorType::SymlinkFoundError.into_apm_error(format!(
                "Found symlink at path {}\nSymlinks cannot be compressed.",
                entry.file_name().to_str().unwrap_or("PATH_UNKNOWN")
            )));
        } else if entry.file_type().is_dir() {
            zip_writer
                .add_directory(&name, options)
                .map_err(|e| APMErrorType::ZIPAddDirectoryError.into_apm_error(e.to_string()))?;

            if let Some(file_names) = &mut file_names {
                file_names.push(name);
            }
        } else if entry.file_type().is_file() {
            zip_writer
                .start_file(&name, options)
                .map_err(|e| APMErrorType::ZIPStartFileError.into_apm_error(e.to_string()))?;

            let mut temp = Vec::new();
            let mut f = OpenOptions::new().read(true).open(&name).map_err(|e| {
                APMErrorType::FileOpenError.into_apm_error(format!(
                    "{}\nFile:{}",
                    e.to_string(),
                    name
                ))
            })?;

            f.read_to_end(&mut temp)
                .map_err(|e| APMErrorType::ZIPFileReadError.into_apm_error(e.to_string()))?;

            zip_writer
                .write_all(&mut temp)
                .map_err(|e| APMErrorType::ZIPFileWriteError.into_apm_error(e.to_string()))?;

            if let Some(file_names) = &mut file_names {
                file_names.push(name);
            }
        }
    }

    zip_writer
        .finish()
        .map_err(|e| APMErrorType::ZIPFinishError.into_apm_error(e.to_string()))?;

    drop(zip_writer);

    return Ok((buffer, file_names));
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_ZIP: &'static str = "test_files/no_hidden.zip";
    const SAMPLE_ZIP_HASH: &'static str = "BHp9Y1ONQRDhB6HN8mEgfktQnrigycbH+dQ3vNCFQew=";

    #[test]
    fn test_modify_sample() {
        let (modified_zip, _) = insert_checksum_zip(SAMPLE_ZIP, false).unwrap();
        let files = dump_file_names_zip_bytes(&modified_zip).unwrap();
        let mut found = false;

        for file in files {
            found = found || &file == "/hidden";
        }

        assert!(found);
        let hash = dump_archer_hash_zip_file(&modified_zip).unwrap();
        assert_eq!(hash, SAMPLE_ZIP_HASH);
    }
}
