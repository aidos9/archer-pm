use super::error::{ProcessingError, ProcessingErrorType};
use super::xml_object::XMLManualObject;

use std::collections::HashMap;
use std::{fmt, io};

use fast_xml::{events::Event, Reader};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Manifest {
    #[serde(rename = "PackageName")]
    pub package_name: String,
    #[serde(rename = "PackageGuid")]
    pub package_guid: String,
    #[serde(rename = "PackageVersion")]
    pub package_version: String,
    #[serde(rename = "PackageProvider")]
    pub package_provider: String,
    #[serde(rename = "PackageDescription")]
    pub package_description: String,
    #[serde(rename = "PackageDate")]
    pub package_date: String,
    #[serde(rename = "ObjectGroup", default)]
    object_groups: Vec<ObjectGroup>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct ObjectGroup {
    #[serde(rename = "fileUri")]
    file_uri: String,
    objects: Vec<ManifestObject>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct ManifestObject {
    #[serde(rename = "Tag")]
    object_tag: ManifestObjectTag,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Guid")]
    guid: String,
    #[serde(rename = "Status")]
    status: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "AdditionalPairs")]
    additional_pairs: Option<HashMap<String, String>>,
}

define_package_string_enum! {
    pub ManifestObjectTag {
        Datafeed = "DataFeed"
    },
    other
}

impl Manifest {
    pub fn new(
        package_name: String,
        package_guid: String,
        package_version: String,
        package_provider: String,
        package_description: String,
        package_date: String,
        object_groups: Vec<ObjectGroup>,
    ) -> Self {
        return Self {
            package_name,
            package_guid,
            package_version,
            package_provider,
            package_description,
            package_date,
            object_groups,
        };
    }

    pub fn new_str(
        package_name: &str,
        package_guid: &str,
        package_version: &str,
        package_provider: &str,
        package_description: &str,
        package_date: &str,
        object_groups: Vec<ObjectGroup>,
    ) -> Self {
        return Self {
            package_name: package_name.to_string(),
            package_guid: package_guid.to_string(),
            package_version: package_version.to_string(),
            package_provider: package_provider.to_string(),
            package_description: package_description.to_string(),
            package_date: package_date.to_string(),
            object_groups,
        };
    }

    pub fn find_object_group_with_tag(&self, tag: ManifestObjectTag) -> Option<&ObjectGroup> {
        for object in &self.object_groups {
            if object.has_object_tag(&tag) {
                return Some(object);
            }
        }

        return None;
    }

    fn xml_parse_object<B: io::BufRead>(
        reader: &mut Reader<B>,
        n: &[u8],
    ) -> Result<ManifestObject, ProcessingError> {
        let mut obj = ManifestObject::default();

        obj.object_tag = ManifestObjectTag::from(
            String::from_utf8(n.to_vec())
                .map_err(|e| ProcessingErrorType::UTF8Error.into_error(e.to_string()))?,
        );

        loop {
            match reader.read_event(&mut Vec::new()).map_err(|e| {
                ProcessingErrorType::XMLEventDeserializeError.into_error(e.to_string())
            })? {
                Event::Start(ref e) => {
                    let text = Self::read_xml_text(reader, e.name())?;

                    match e.name() {
                        b"Name" => obj.name = text,
                        b"Guid" => obj.guid = text,
                        b"Status" => obj.status = text,
                        k => {
                            if obj.additional_pairs.is_none() {
                                obj.additional_pairs = Some(HashMap::new());
                            }

                            obj.additional_pairs.as_mut().unwrap().insert(
                                String::from_utf8(k.to_vec()).map_err(|e| {
                                    ProcessingErrorType::UTF8Error.into_error(e.to_string())
                                })?,
                                text,
                            );
                        }
                    }
                }
                Event::End(_) => break,
                Event::Eof => {
                    return Err(ProcessingErrorType::XMLEventDeserializeError
                        .into_error("Unexepected EOF".to_string()))
                }
                Event::Text(_)
                | Event::Empty(_)
                | Event::Comment(_)
                | Event::DocType(_)
                | Event::PI(_)
                | Event::Decl(_)
                | Event::CData(_) => (),
            }
        }

        return Ok(obj);
    }

    fn read_xml_text<B: io::BufRead>(
        reader: &mut Reader<B>,
        end: &[u8],
    ) -> Result<String, ProcessingError> {
        return reader
            .read_text(end, &mut Vec::new())
            .map_err(|e| ProcessingErrorType::XMLEventDeserializeError.into_error(e.to_string()));
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

impl XMLManualObject for Manifest {
    fn from_xml_reader<B: io::BufRead>(mut reader: Reader<B>) -> Result<Self, ProcessingError> {
        let mut buf = Vec::new();
        let mut manifest = Manifest::default();

        loop {
            match reader.read_event(&mut buf).map_err(|e| {
                ProcessingErrorType::XMLEventDeserializeError.into_error(e.to_string())
            })? {
                Event::Start(ref e) => match e.name() {
                    b"PackageName" => {
                        manifest.package_name = Self::read_xml_text(&mut reader, e.name())?;
                    }
                    b"PackageDate" => {
                        manifest.package_date = Self::read_xml_text(&mut reader, e.name())?;
                    }
                    b"PackageGuid" => {
                        manifest.package_guid = Self::read_xml_text(&mut reader, e.name())?;
                    }
                    b"PackageVersion" => {
                        manifest.package_version = Self::read_xml_text(&mut reader, e.name())?;
                    }
                    b"PackageProvider" => {
                        manifest.package_provider = Self::read_xml_text(&mut reader, e.name())?;
                    }
                    b"PackageDescription" => {
                        manifest.package_description = Self::read_xml_text(&mut reader, e.name())?;
                    }
                    b"ObjectGroup" => {
                        let mut object_group = ObjectGroup::default();

                        for attribute in e.attributes() {
                            let attribute = attribute.map_err(|e| {
                                ProcessingErrorType::XMLAttributeDeserializeError
                                    .into_error(e.to_string())
                            })?;

                            // Ignore any other keys, if they exist for some reason
                            if attribute.key == b"fileUri" {
                                object_group.file_uri = attribute
                                    .unescape_and_decode_value(&mut reader)
                                    .map_err(|e| {
                                        ProcessingErrorType::XMLAttributeDeserializeError
                                            .into_error(e.to_string())
                                    })?;
                                break;
                            }
                        }

                        loop {
                            match reader.read_event(&mut Vec::new()).map_err(|e| {
                                ProcessingErrorType::XMLEventDeserializeError
                                    .into_error(e.to_string())
                            })? {
                                Event::Start(ref e) => {
                                    object_group
                                        .objects
                                        .push(Self::xml_parse_object(&mut reader, e)?);
                                }
                                Event::End(_) => break,
                                Event::Eof => {
                                    return Err(ProcessingErrorType::XMLEventDeserializeError
                                        .into_error("Unexepected EOF".to_string()))
                                }
                                Event::Text(_)
                                | Event::Empty(_)
                                | Event::Comment(_)
                                | Event::DocType(_)
                                | Event::PI(_)
                                | Event::Decl(_)
                                | Event::CData(_) => (),
                            }
                        }

                        manifest.object_groups.push(object_group);
                    }
                    _ => (),
                },
                Event::Empty(ref e) => {
                    match e.name() {
                        b"ObjectGroup" => {
                            let mut object_group = ObjectGroup::default();

                            for attribute in e.attributes() {
                                let attribute = attribute.map_err(|e| {
                                    ProcessingErrorType::XMLAttributeDeserializeError
                                        .into_error(e.to_string())
                                })?;

                                // Ignore any other keys, if they exist for some reason
                                if attribute.key == b"fileUri" {
                                    object_group.file_uri = attribute
                                        .unescape_and_decode_value(&mut reader)
                                        .map_err(|e| {
                                            ProcessingErrorType::XMLAttributeDeserializeError
                                                .into_error(e.to_string())
                                        })?;
                                    break;
                                }
                            }

                            manifest.object_groups.push(object_group);
                        }
                        _ => (),
                    }
                }
                Event::Eof => break,
                Event::Text(_)
                | Event::Comment(_)
                | Event::End(_)
                | Event::DocType(_)
                | Event::PI(_)
                | Event::Decl(_)
                | Event::CData(_) => (),
            }

            buf.clear();
        }

        return Ok(manifest);
    }
}

impl Default for Manifest {
    fn default() -> Self {
        return Self {
            package_name: Default::default(),
            package_guid: Default::default(),
            package_version: Default::default(),
            package_provider: Default::default(),
            package_description: Default::default(),
            package_date: Default::default(),
            object_groups: Default::default(),
        };
    }
}

impl ObjectGroup {
    pub fn new(file_uri: String, objects: Vec<ManifestObject>) -> Self {
        return Self { file_uri, objects };
    }

    pub fn new_str(file_uri: &str, objects: Vec<ManifestObject>) -> Self {
        return Self::new(file_uri.to_string(), objects);
    }

    pub fn file_uri(&self) -> &String {
        return &self.file_uri;
    }

    pub fn has_object_tag(&self, tag: &ManifestObjectTag) -> bool {
        for object in &self.objects {
            if &object.object_tag == tag {
                return true;
            }
        }

        return false;
    }
}

impl Default for ObjectGroup {
    fn default() -> Self {
        return Self {
            file_uri: Default::default(),
            objects: Default::default(),
        };
    }
}

impl ManifestObject {
    pub fn new(
        object_tag: ManifestObjectTag,
        name: String,
        guid: String,
        status: String,
        additional_pairs: Option<HashMap<String, String>>,
    ) -> Self {
        return Self {
            object_tag,
            name,
            guid,
            status,
            additional_pairs,
        };
    }

    pub fn new_str(
        object_tag: ManifestObjectTag,
        name: &str,
        guid: &str,
        status: &str,
        additional_pairs: Option<HashMap<String, String>>,
    ) -> Self {
        return Self::new(
            object_tag,
            name.to_string(),
            guid.to_string(),
            status.to_string(),
            additional_pairs,
        );
    }
}

impl Default for ManifestObject {
    fn default() -> Self {
        return Self {
            object_tag: Default::default(),
            name: Default::default(),
            guid: Default::default(),
            status: Default::default(),
            additional_pairs: None,
        };
    }
}

impl Default for ManifestObjectTag {
    fn default() -> Self {
        return Self::Other(String::new());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packages::processing::XMLObject;

    #[test]
    pub fn test_sample_manifest_1() {
        let file_contents = include_str!("test_files/sample_manifest_1.xml");
        let manifest = Manifest::from_xml_str(file_contents).unwrap();

        assert_eq!(
            manifest,
            Manifest::new_str(
                "Package Name",
                "aaaaaaaa-2c07-4a38-bb2a-a7a5cb0f753b",
                "6.9.10100.1114",
                "Test Package Provider",
                "",
                "2022-04-14T02:39:35.1480573Z",
                vec![
                    ObjectGroup::new_str(
                        "archerschema.xsd",
                        vec![ManifestObject::new_str(
                            ManifestObjectTag::from("Application"),
                            "Application",
                            "aaaaaaaa-2c07-4a38-afb5-a7a5cb0f753b",
                            "ActiveInProduction",
                            None,
                        )]
                    ),
                    ObjectGroup::new_str("Workspaces.xml", Vec::new()),
                    ObjectGroup::new_str("Dashboards.xml", Vec::new()),
                    ObjectGroup::new_str("AccessRoles.xml", Vec::new()),
                    ObjectGroup::new_str("levelDisplay.xml", Vec::new()),
                    ObjectGroup::new_str("TemplateDefinitions.xml", Vec::new()),
                    ObjectGroup {
                        file_uri: "PackageQuestionnaireData.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "Workpoint.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "ContentSignatures.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "Schedules.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "NavigationMenu.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "DataDrivenEvent.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "ReportDefinitions.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "Workflow.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "Notifications.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "Solutions.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "DataFeeds.xml".to_string(),
                        objects: vec![ManifestObject::new(
                            ManifestObjectTag::from("DataFeed"),
                            "MyDatafeed".to_string(),
                            "aaaaaaaa-bbd3-46bb-bb1e-a7a5cb0f753b".to_string(),
                            "True".to_string(),
                            None,
                        )]
                    },
                    ObjectGroup {
                        file_uri: "IViews.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "PackageContents.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "Links.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "RepositoryFiles.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "UsersGroups.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "Translation.xml".to_string(),
                        objects: vec![ManifestObject::new(
                            ManifestObjectTag::from("Language"),
                            "English".to_string(),
                            "9088ef11-366b-47bd-8462-a7a5cb0f753b".to_string(),
                            "True".to_string(),
                            None,
                        )]
                    }
                ]
            )
        );
    }

    #[test]
    pub fn test_sample_manifest_2() {
        let file_contents = include_str!("test_files/sample_manifest_2.xml");
        let manifest = Manifest::from_xml_str(file_contents).unwrap();

        assert_eq!(
            manifest,
            Manifest::new_str(
                "Package Name",
                "8585b948-2c07-4a38-bb2a-a7a5cb0f753b",
                "6.9.10100.1114",
                "Test Package Provider",
                "My description",
                "2022-04-14T02:39:35.1480573Z",
                vec![
                    ObjectGroup {
                        file_uri: "archerschema.xsd".to_string(),
                        objects: vec![ManifestObject::new(
                            ManifestObjectTag::from("Application"),
                            "Application".to_string(),
                            "aaaaaaaa-2c07-4a38-afb5-a7a5cb0f753b".to_string(),
                            "ActiveInProduction".to_string(),
                            None,
                        )]
                    },
                    ObjectGroup {
                        file_uri: "Workspaces.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "Dashboards.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "AccessRoles.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "levelDisplay.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "TemplateDefinitions.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "PackageQuestionnaireData.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "Workpoint.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "ContentSignatures.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "Schedules.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "NavigationMenu.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "DataDrivenEvent.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "ReportDefinitions.xml".to_string(),
                        objects: vec![ManifestObject::new(
                            ManifestObjectTag::from("GlobalReport"),
                            "Report".to_string(),
                            "aaaaaaaa-4866-40ea-a298-99afb348598d".to_string(),
                            String::new(),
                            Some(
                                hashmap!["ParentGuid".to_string() ; "aaaaaaaa-4b51-404c-91c2-40ade972e95b".to_string()]
                            )
                        )],
                    },
                    ObjectGroup {
                        file_uri: "Workflow.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "Notifications.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "Solutions.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "DataFeeds.xml".to_string(),
                        objects: vec![ManifestObject::new(
                            ManifestObjectTag::from("DataFeed"),
                            "MyDatafeed".to_string(),
                            "aaaaaaaa-bbd3-46bb-bb1e-a7a5cb0f753b".to_string(),
                            "True".to_string(),
                            None,
                        )]
                    },
                    ObjectGroup {
                        file_uri: "IViews.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "PackageContents.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "Links.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "RepositoryFiles.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "UsersGroups.xml".to_string(),
                        objects: Vec::new(),
                    },
                    ObjectGroup {
                        file_uri: "Translation.xml".to_string(),
                        objects: vec![ManifestObject::new(
                            ManifestObjectTag::from("Language"),
                            "English".to_string(),
                            "9088ef11-366b-47bd-8462-a7a5cb0f753b".to_string(),
                            "True".to_string(),
                            None,
                        )]
                    }
                ]
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
                vec![ObjectGroup::new_str(
                    "archerschema.xsd",
                    vec![ManifestObject::new_str(
                        ManifestObjectTag::from("Application"),
                        "Application",
                        "603a48f5-2c07-4a38-afb5-a7a5cb0f753b",
                        "ActiveInProduction",
                        None,
                    )],
                )],
            );

            assert_eq!(manifest.to_json(false).unwrap(), "{\"PackageName\":\"Package Name\",\"PackageGuid\":\"8585b948-2c07-4a38-bb2a-a7a5cb0f753b\",\"PackageVersion\":\"6.9.10100.1114\",\"PackageProvider\":\"Test Package Provider\",\"PackageDescription\":\"\",\"PackageDate\":\"2022-04-14T02:39:35.1480573Z\",\"ObjectGroup\":[{\"fileUri\":\"archerschema.xsd\",\"objects\":[{\"Tag\":\"Application\",\"Name\":\"Application\",\"Guid\":\"603a48f5-2c07-4a38-afb5-a7a5cb0f753b\",\"Status\":\"ActiveInProduction\"}]}]}");
        }

        #[test]
        pub fn test_simple_manifest_export_2() {
            let manifest = Manifest::new_str(
                "Package Name",
                "8585b948-2c07-4a38-bb2a-a7a5cb0f753b",
                "6.9.10100.1114",
                "Test Package Provider",
                "",
                "2022-04-14T02:39:35.1480573Z",
                vec![ObjectGroup::new_str(
                    "archerschema.xsd",
                    vec![ManifestObject::new_str(
                        ManifestObjectTag::from("Application"),
                        "Application",
                        "603a48f5-2c07-4a38-afb5-a7a5cb0f753b",
                        "ActiveInProduction",
                        Some(
                            hashmap!("ParentGuid".to_string() ; "aaaaaaaa-2c07-4da2-afb5-839144efc58a".to_string()),
                        ),
                    )],
                )],
            );

            assert_eq!(manifest.to_json(false).unwrap(), "{\"PackageName\":\"Package Name\",\"PackageGuid\":\"8585b948-2c07-4a38-bb2a-a7a5cb0f753b\",\"PackageVersion\":\"6.9.10100.1114\",\"PackageProvider\":\"Test Package Provider\",\"PackageDescription\":\"\",\"PackageDate\":\"2022-04-14T02:39:35.1480573Z\",\"ObjectGroup\":[{\"fileUri\":\"archerschema.xsd\",\"objects\":[{\"Tag\":\"Application\",\"Name\":\"Application\",\"Guid\":\"603a48f5-2c07-4a38-afb5-a7a5cb0f753b\",\"Status\":\"ActiveInProduction\",\"AdditionalPairs\":{\"ParentGuid\":\"aaaaaaaa-2c07-4da2-afb5-839144efc58a\"}}]}]}");
        }
    }
}
