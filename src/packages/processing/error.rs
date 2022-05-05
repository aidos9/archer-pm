#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum ProcessingErrorType {
    ManifestDeserializationError,
    FileOpenError,
    FileReadError,
    BaseFileNotFoundError(String),
    #[cfg(feature = "json_exporter")]
    JSONExportError,
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
        };
    }
}

impl ProcessingError {
    pub fn new(tp: ProcessingErrorType, debug_msg: String, public_msg: String) -> Self {
        return Self {
            tp,
            debug_msg,
            public_msg,
        };
    }
}
