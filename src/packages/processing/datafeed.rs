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
    update_information: UpdateInformation,
    #[serde(rename = "Active")]
    active: bool,
    #[serde(rename = "Alias")]
    alias: String,
    #[serde(rename = "ConfigurationXml")]
    configuration: String,
    #[serde(rename = "DataFeedType")]
    datafeed_type: DatafeedType,
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "EnableHolding")]
    enable_holding: bool,
    #[serde(rename = "GUID")]
    guid: String,
    #[serde(rename = "Id")]
    id: u64,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "ScheduleParentGuid")]
    schedule_parent_guid: String,
    #[serde(rename = "ScheduleParentName")]
    schedule_parent_name: String,
    #[serde(rename = "ScheduledGuid")]
    schedule_guid: String,
    #[serde(rename = "SelectedTarget")]
    selected_target: u64,
    #[serde(rename = "Status")]
    status: String,
}

#[derive(Debug, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct UpdateInformation {
    #[serde(rename = "CreateDate")]
    creation_date: String,
    #[serde(rename = "CreateLogin")]
    creation_login: u64,
    #[serde(rename = "UpdateDate")]
    update_date: String,
    #[serde(rename = "UpdateLogin")]
    update_login: u64,
}

#[derive(Debug, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct Users {
    #[serde(rename = "KeyValuePairOfintint")]
    users: Vec<UserPair>,
}

#[derive(Debug, PartialEq, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct UserPair {
    key: u64,
    value: u64,
}

define_package_string_enum! {
    pub DatafeedType {
        StandardService= "StandardServiceDataFeed"
    },
    other
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
