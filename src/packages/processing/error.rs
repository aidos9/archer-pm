use std::fmt;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum ProcessingErrorType {
    ManifestDeserializationError,
    FileOpenError,
    FileReadError,
    BaseFileNotFoundError(String),
    #[cfg(feature = "json_exporter")]
    JSONExportError,
    XMLEventDeserializeError,
    XMLAttributeDeserializeError,
    XMLUnexpectedTagError,
    UTF8Error,
    ManifestTagNotFound,
    CannotConvertString(String),
    XMLUnexpectedEOFError,
}

#[derive(Debug, Hash, PartialEq, Clone)]
pub struct ProcessingError {
    tp: ProcessingErrorType,
    debug_msg: String,
    public_msg: String,
}

impl ProcessingErrorType {
    pub fn into_error(self, m: String) -> ProcessingError {
        let msg = self.public_msg();

        return ProcessingError::new(self, m, msg);
    }

    pub fn public_msg(&self) -> String {
        return match self {
            ProcessingErrorType::ManifestDeserializationError => "Manifest Malformed".to_string(),
            #[cfg(feature = "json_exporter")]
            ProcessingErrorType::JSONExportError => "JSON Export Error".to_string(),
            ProcessingErrorType::FileOpenError => "File Open Error".to_string(),
            ProcessingErrorType::FileReadError => "File Read Error".to_string(),
            ProcessingErrorType::BaseFileNotFoundError(f) => {
                format!("Base File ({}) Not Found Error", f)
            }
            ProcessingErrorType::XMLEventDeserializeError => "XML Deserialize Error".to_string(),
            ProcessingErrorType::XMLAttributeDeserializeError => {
                "XML Attribute Deserialize Error".to_string()
            }
            ProcessingErrorType::UTF8Error => "UTF-8 Error".to_string(),
            ProcessingErrorType::ManifestTagNotFound => {
                "Unexpectedly could not locate a tag in the manifest file.".to_string()
            }
            ProcessingErrorType::XMLUnexpectedTagError => "Unexpected XML tag Error".to_string(),
            ProcessingErrorType::CannotConvertString(k) => {
                format!("Could not convert the text in key '{}' to int", k)
            }
            ProcessingErrorType::XMLUnexpectedEOFError => {
                "Unexpected end of XML file error".to_string()
            }
        };
    }
}

impl ProcessingError {
    pub(crate) fn new(tp: ProcessingErrorType, debug_msg: String, public_msg: String) -> Self {
        return Self {
            tp,
            debug_msg,
            public_msg,
        };
    }
}

impl fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Processing Error: {}", self.public_msg);
    }
}
