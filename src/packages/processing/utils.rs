use std::fs::{File, OpenOptions};
use std::io::{self, Read};
use std::path::Path;

use super::error::{ProcessingError, ProcessingErrorType};

const MAX_FILES: usize = 9999;

#[derive(Clone, Debug, PartialEq, Hash)]
pub struct PackageFile {
    base_file: String,
    files: Vec<String>,
}

pub struct PackageFileReader {
    files: Vec<File>,
    current_file_index: usize,
}

impl PackageFile {
    pub fn new(base_file: String) -> Result<Self, ProcessingError> {
        let mut files = Vec::new();

        if !Path::new(&base_file).exists() {
            return Err(ProcessingErrorType::BaseFileNotFoundError(base_file)
                .into_error("File does not exist".to_string()));
        }

        for i in 1..MAX_FILES {
            let name = format!("{}.{:0>4}", &base_file, i);

            if Path::new(&name).exists() {
                files.push(name);
            } else {
                break;
            }
        }

        return Ok(Self { base_file, files });
    }

    pub fn get_reader(&self) -> Result<PackageFileReader, ProcessingError> {
        return PackageFileReader::new(&self.base_file, &self.files);
    }

    pub fn read_all_files(&self) -> Result<Vec<u8>, ProcessingError> {
        let mut reader = self.get_reader()?;
        let mut buffer = Vec::new();

        reader
            .read_to_end(&mut buffer)
            .map_err(|e| ProcessingErrorType::FileReadError.into_error(e.to_string()))?;

        return Ok(buffer);
    }
}

impl PackageFileReader {
    pub fn new(base_file: &str, files: &[String]) -> Result<Self, ProcessingError> {
        let mut reader = PackageFileReader {
            files: Vec::with_capacity(files.len() + 1),
            current_file_index: 0,
        };

        reader.open(base_file, files)?;

        return Ok(reader);
    }

    fn open(&mut self, base_file: &str, files: &[String]) -> Result<(), ProcessingError> {
        self.files.push(
            OpenOptions::new()
                .read(true)
                .write(false)
                .open(base_file)
                .map_err(|e| ProcessingErrorType::FileOpenError.into_error(e.to_string()))?,
        );

        for f in files {
            self.files.push(
                OpenOptions::new()
                    .read(true)
                    .write(false)
                    .open(f)
                    .map_err(|e| ProcessingErrorType::FileOpenError.into_error(e.to_string()))?,
            );
        }

        return Ok(());
    }
}

impl Read for PackageFileReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        while self.current_file_index < self.files.len() {
            let read = self.files[self.current_file_index].read(buf)?;

            if read > 0 {
                return Ok(read);
            }

            self.current_file_index += 1;
        }

        return Ok(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_multiple() {
        let f_name = "test_files/test_dir/next_level/file.xml";
        let f = PackageFile::new(f_name.to_string()).unwrap();
        let mut buf = Vec::new();
        f.get_reader().unwrap().read_to_end(&mut buf).unwrap();
        assert_eq!(buf, b"abcdefghi");
    }

    #[test]
    fn test_read_multiple_2() {
        let f_name = "test_files/test_dir/next_level/file.xml";
        let contents = PackageFile::new(f_name.to_string())
            .unwrap()
            .read_all_files()
            .unwrap();

        assert_eq!(contents, b"abcdefghi");
    }

    #[test]
    fn test_read_singlee() {
        let f_name = "test_files/test_dir/next_level/test.xml";
        let f = PackageFile::new(f_name.to_string()).unwrap();
        let mut buf = Vec::new();
        f.get_reader().unwrap().read_to_end(&mut buf).unwrap();
        assert_eq!(buf, b"test");
    }
}
