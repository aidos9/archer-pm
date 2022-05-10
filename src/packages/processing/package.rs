use std::fs::OpenOptions;

use super::{Datafeed, Manifest, PackageFile, ProcessingError, ProcessingErrorType};

pub struct Package {
    manifest_path: String,
    data_feeds: bool,
    // notifications: bool,
    // solutions: bool,
}

pub struct LoadedPackage {
    manifest: Manifest,
    datafeeds: Option<Vec<Datafeed>>,
    // notifications: Option<Vec<Notifications>>,
    // solutions: Option<Vec<Solution>>,
}

impl Package {
    pub fn new(manifest_path: &str) -> Self {
        return Self {
            manifest_path: manifest_path.to_string(),
            data_feeds: false,
            // notifications: false,
            // solutions: false,
        };
    }

    with_function!(data_feeds, bool);
    // with_function!(notifications, bool);
    // with_function!(solutions, bool);

    pub fn load(self) -> Result<LoadedPackage, ProcessingError> {
        let f = OpenOptions::new()
            .read(true)
            .open(self.manifest_path)
            .map_err(|e| ProcessingErrorType::FileOpenError.into_error(e.to_string()))?;

        let mut loaded_package = LoadedPackage::new(Manifest::from_xml_read(f)?);

        if self.data_feeds {
            loaded_package.load_datafeeds()?;
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

    fn load_datafeeds(&mut self) -> Result<(), ProcessingError> {
        todo!();
    }
}
