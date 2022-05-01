use crate::error::{APMError, APMErrorType};

use std::fs::{File, OpenOptions};
use std::io::{copy, Cursor, Read, Seek, Write};

use walkdir::WalkDir;
use zip::{write::FileOptions, ZipArchive, ZipWriter};

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
            add_file_to_archive(&mut zip_writer, &name, Some(options))?;

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

pub fn read_archive(path: &str) -> Result<ZipArchive<File>, APMError> {
    let mut f = OpenOptions::new()
        .read(true)
        .write(false)
        .open(path)
        .map_err(|e| APMErrorType::FileOpenError.into_apm_error(e.to_string()))?;

    return Ok(ZipArchive::new(f)
        .map_err(|e| APMErrorType::ZIPArchiveOpenError.into_apm_error(e.to_string()))?);
}

pub fn add_file_to_archive<A: Read + Seek + Write>(
    archive: &mut ZipWriter<A>,
    file: &str,
    options: Option<FileOptions>,
) -> Result<(), APMError> {
    let options = options.unwrap_or(FileOptions::default());

    archive
        .start_file(file, options)
        .map_err(|e| APMErrorType::ZIPStartFileError.into_apm_error(e.to_string()))?;

    let mut f = OpenOptions::new().read(true).open(&file).map_err(|e| {
        APMErrorType::FileOpenError.into_apm_error(format!("{}\nFile:{}", e.to_string(), file))
    })?;

    copy(&mut f, archive)
        .map_err(|e| APMErrorType::ZIPFileCopyError.into_apm_error(e.to_string()))?;

    todo!();
}
