use super::{Manifest, ManifestObjectTag, PackageDatafeedsFile, ProcessingErrorType, XMLObject};
use crate::error::{APMError, APMErrorType};
use crate::zip_manipulation::extract_file_from_archive;

use std::fmt;
use std::io::{Read, Seek};
use std::str::FromStr;

use clap::ArgEnum;
use const_format::concatcp;
use zip::ZipArchive;

const MANIFEST_FILE_NAME: &'static str = "archerpackage.manifest";
const BASE_PATH: &'static str = "en-US";

pub struct Package<R: Read + Seek> {
    package_archive: ZipArchive<R>,
    data_feeds: bool,
    // notifications: bool,
    // solutions: bool,
}

pub struct LoadedPackage {
    manifest: Manifest,
    datafeeds: Option<PackageDatafeedsFile>,
    // notifications: Option<Vec<Notifications>>,
    // solutions: Option<Vec<Solution>>,
}

#[derive(Clone, Copy, PartialEq, Hash, Debug, ArgEnum)]
pub enum PackageObject {
    Datafeeds,
}

impl<R: Read + Seek> Package<R> {
    pub fn new(package_archive: ZipArchive<R>) -> Self {
        return Self {
            package_archive,
            data_feeds: false,
            // notifications: false,
            // solutions: false,
        };
    }

    with_function!(data_feeds, bool, true);
    // with_function!(notifications, bool);
    // with_function!(solutions, bool);

    pub fn load(mut self) -> Result<LoadedPackage, APMError> {
        let mainfest_contents = extract_file_from_archive(
            &mut self.package_archive,
            concatcp!(BASE_PATH, "/", MANIFEST_FILE_NAME),
        )?;

        let mut loaded_package = LoadedPackage::new(
            Manifest::from_xml_bytes(mainfest_contents)
                .map_err(|e| APMErrorType::ProcessingError.into_apm_error(e.to_string()))?,
        );

        if self.data_feeds {
            loaded_package.load_datafeeds(&mut self.package_archive)?;
        }

        return Ok(loaded_package);
    }
}

impl LoadedPackage {
    fn new(manifest: Manifest) -> Self {
        return Self {
            manifest,
            datafeeds: None,
        };
    }

    fn load_datafeeds<R: Read + Seek>(
        &mut self,
        archive: &mut ZipArchive<R>,
    ) -> Result<(), APMError> {
        if let Some(object_group) = self
            .manifest
            .find_object_group_with_tag(ManifestObjectTag::Datafeed)
        {
            let bytes = extract_file_from_archive(
                archive,
                &format!("{}/{}", BASE_PATH, object_group.file_uri()),
            )?;

            self.datafeeds = Some(
                PackageDatafeedsFile::from_xml_bytes(bytes)
                    .map_err(|e| APMErrorType::ProcessingError.into_apm_error(e.to_string()))?,
            );
        } else {
            return Err(APMErrorType::ProcessingError.into_apm_error(
                ProcessingErrorType::ManifestTagNotFound
                    .into_error(format!(
                        "Could not find the key \"{}\" in the manifest file.",
                        ManifestObjectTag::Datafeed
                    ))
                    .to_string(),
            ));
        }

        return Ok(());
    }

    pub fn manifest(&self) -> &Manifest {
        return &self.manifest;
    }

    pub fn datafeeds(&self) -> Option<&PackageDatafeedsFile> {
        return self.datafeeds.as_ref();
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::zip_manipulation::read_archive_from_bytes;

    use super::*;

    #[test]
    fn my_test() {
        let path = Path::new("/file.txt")
            .parent()
            .map(|p| p.as_os_str().to_str().unwrap());

        assert_eq!(path, Some("/"));
    }

    #[test]
    fn test_kfv_manifest_only() {
        let zip_bytes = include_bytes!("test_files/kfv.zip");

        let archive = read_archive_from_bytes(zip_bytes).unwrap();

        let package = Package::new(archive);
        package.load().unwrap();
    }

    #[test]
    fn test_kfv_datafeeds() {
        let zip_bytes = include_bytes!("test_files/kfv.zip");

        let archive = read_archive_from_bytes(zip_bytes).unwrap();

        let package = Package::new(archive).with_data_feeds();
        package.load().unwrap();
    }
}
