use super::error::{ProcessingError, ProcessingErrorType};

use std::io;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Manifest {
    #[serde(rename = "PackageName")]
    package_name: String,
    #[serde(rename = "PackageGuid")]
    package_guid: String,
    #[serde(rename = "PackageVersion")]
    package_version: String,
    #[serde(rename = "PackageProvider")]
    package_provider: String,
    #[serde(rename = "PackageDescription")]
    package_description: String,
    #[serde(rename = "PackageDate")]
    package_date: String,
}

impl Manifest {
    pub fn new(
        package_name: String,
        package_guid: String,
        package_version: String,
        package_provider: String,
        package_description: String,
        package_date: String,
    ) -> Self {
        return Self {
            package_name,
            package_guid,
            package_version,
            package_provider,
            package_description,
            package_date,
        };
    }

    pub fn new_str(
        package_name: &str,
        package_guid: &str,
        package_version: &str,
        package_provider: &str,
        package_description: &str,
        package_date: &str,
    ) -> Self {
        return Self {
            package_name: package_name.to_string(),
            package_guid: package_guid.to_string(),
            package_version: package_version.to_string(),
            package_provider: package_provider.to_string(),
            package_description: package_description.to_string(),
            package_date: package_date.to_string(),
        };
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, ProcessingError> {
        return Self::from_reader(io::Cursor::new(bytes));
    }

    pub fn from_str(s: &str) -> Result<Self, ProcessingError> {
        return serde_xml_rs::from_str(s).map_err(|e| {
            ProcessingErrorType::ManifestDeserializationError.into_error(e.to_string())
        });
    }

    pub fn from_reader<R: io::Read>(reader: R) -> Result<Self, ProcessingError> {
        return serde_xml_rs::from_reader(reader).map_err(|e| {
            ProcessingErrorType::ManifestDeserializationError.into_error(e.to_string())
        });
    }

    #[cfg(feature = "json_exporter")]
    pub fn to_json(&self, pretty: bool) -> Result<String, ProcessingError> {
        if pretty {
            return serde_json::to_string_pretty(self)
                .map_err(|e| ProcessingErrorType::JSONExportError.into_error(e.to_string()));
        } else {
            return serde_json::to_string(self)
                .map_err(|e| ProcessingErrorType::JSONExportError.into_error(e.to_string()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_simple_manifest() {
        let file_contents = include_str!("test_files/sample_manifest_1.xml");
        let manifest = Manifest::from_str(file_contents).unwrap();

        assert_eq!(
            manifest,
            Manifest::new_str(
                "Package Name",
                "8585b948-2c07-4a38-bb2a-a7a5cb0f753b",
                "6.9.10100.1114",
                "Test Package Provider",
                "",
                "2022-04-14T02:39:35.1480573Z"
            )
        );
    }

    #[cfg(feature = "json_exporter")]
    mod json_export_tests {
        use super::*;

        #[test]
        pub fn test_simple_manifest_export() {
            let manifest = Manifest::new_str(
                "Package Name",
                "8585b948-2c07-4a38-bb2a-a7a5cb0f753b",
                "6.9.10100.1114",
                "Test Package Provider",
                "",
                "2022-04-14T02:39:35.1480573Z",
            );

            assert_eq!(manifest.to_json(false).unwrap(), "{\"PackageName\":\"Package Name\",\"PackageGuid\":\"8585b948-2c07-4a38-bb2a-a7a5cb0f753b\",\"PackageVersion\":\"6.9.10100.1114\",\"PackageProvider\":\"Test Package Provider\",\"PackageDescription\":\"\",\"PackageDate\":\"2022-04-14T02:39:35.1480573Z\"}");
        }
    }
}
