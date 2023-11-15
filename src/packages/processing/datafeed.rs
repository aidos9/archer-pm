use super::data_format::IntoDataFormat;
use crate::packages::processing::XMLObject;

use super::error::{ProcessingError, ProcessingErrorType};

use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PackageDatafeedsFile {
    #[serde(rename = "DataFeeds", default)]
    datafeeds: PackageDataFeeds,
    #[serde(rename = "Users")]
    users: Users,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
struct PackageDataFeeds {
    #[serde(rename = "DataFeed")]
    datafeeds: Vec<PackageDatafeed>,
}

#[derive(Debug, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct PackageDatafeed {
    #[serde(rename = "UpdateInformation")]
    pub update_information: UpdateInformation,
    #[serde(rename = "Active")]
    pub active: bool,
    #[serde(rename = "Alias")]
    pub alias: String,
    #[serde(rename = "ConfigurationXml")]
    pub configuration: String,
    #[serde(rename = "DataFeedType")]
    pub datafeed_type: DatafeedType,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "EnableHolding")]
    pub enable_holding: bool,
    #[serde(rename = "GUID")]
    pub guid: String,
    #[serde(rename = "Id")]
    pub id: u64,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ScheduleParentGuid")]
    pub schedule_parent_guid: String,
    #[serde(rename = "ScheduleParentName")]
    pub schedule_parent_name: String,
    #[serde(rename = "ScheduledGuid")]
    pub schedule_guid: String,
    #[serde(rename = "SelectedTarget")]
    pub selected_target: u64,
    #[serde(rename = "Status")]
    pub status: String,
}

#[derive(Debug, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct UpdateInformation {
    #[serde(rename = "CreateDate")]
    pub creation_date: String,
    #[serde(rename = "CreateLogin")]
    pub creation_login: u64,
    #[serde(rename = "UpdateDate")]
    pub update_date: String,
    #[serde(rename = "UpdateLogin")]
    pub update_login: u64,
}

#[derive(Debug, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct Users {
    #[serde(rename = "KeyValuePairOfintint")]
    pub users: Vec<UserPair>,
}

#[derive(Debug, PartialEq, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct UserPair {
    pub key: u64,
    pub value: u64,
}

define_package_string_enum! {
    pub DatafeedType {
        StandardService= "StandardServiceDataFeed"
    },
    other
}

impl PackageDatafeedsFile {
    pub fn datafeeds(&self) -> &Vec<PackageDatafeed> {
        return &self.datafeeds.datafeeds;
    }

    pub fn users(&self) -> &Vec<UserPair> {
        return &self.users.users;
    }
}

impl XMLObject for PackageDatafeedsFile {
    fn from_xml_str(s: &str) -> Result<Self, ProcessingError> {
        return fast_xml::de::from_str(s)
            .map_err(|e| ProcessingErrorType::XMLEventDeserializeError.into_error(e.to_string()));
    }

    fn from_xml_bufread<R: std::io::BufRead>(reader: R) -> Result<Self, ProcessingError> {
        return fast_xml::de::from_reader(reader)
            .map_err(|e| ProcessingErrorType::XMLEventDeserializeError.into_error(e.to_string()));
    }
}

impl IntoDataFormat for PackageDatafeedsFile {
    type Error = ProcessingError;

    fn into_xml_string(&self) -> Result<String, Self::Error> {
        return fast_xml::se::to_string(self)
            .map_err(|e| ProcessingErrorType::XMLExportError.into_error(e.to_string()));
    }

    #[cfg(feature = "json_exporter")]
    fn into_json_string(&self) -> Result<String, Self::Error> {
        return serde_json::to_string(self)
            .map_err(|e| ProcessingErrorType::JSONExportError.into_error(e.to_string()));
    }

    #[cfg(feature = "toml_exporter")]
    fn into_toml_string(&self) -> Result<String, Self::Error> {
        return toml::to_string(self)
            .map_err(|e| ProcessingErrorType::TOMLExportError.into_error(e.to_string()));
    }
}

impl Default for PackageDatafeedsFile {
    fn default() -> Self {
        return Self {
            datafeeds: Default::default(),
            users: Default::default(),
        };
    }
}

impl Default for Users {
    fn default() -> Self {
        Self {
            users: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_string() {
        let s = include_str!("test_files/sample_datafeeds.xml");

        let _datafeeds = PackageDatafeedsFile::from_xml_str(s).unwrap();
    }
}
