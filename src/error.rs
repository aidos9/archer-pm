use std::fmt;

#[derive(Copy, Clone, PartialEq, Hash, Debug)]
pub enum APMErrorType {
    FileOpenError,
    FileReadError,
    FileWriteError,
    ZIPOpenError,
    ZIPArchiveOpenError,
    ZIPCreationError,
    ZIPFinishError,
    ZIPModificationError,
    ZIPArchiveHiddenNotFoundError,
    ZIPArchiveReadError,
    ZIPAddDirectoryError,
    ZIPStartFileError,
    ZIPFileReadError,
    ZIPFileWriteError,
    HashUTF8Error,
}

#[derive(Clone, PartialEq, Hash, Debug)]
pub struct APMError {
    tp: APMErrorType,
    msg: String,
}

impl APMErrorType {
    pub fn into_apm_error(self, m: String) -> APMError {
        return APMError::new(self, m);
    }
}

impl fmt::Display for APMErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            APMErrorType::FileOpenError => "File Open Error",
            APMErrorType::FileReadError => "File Read Error",
            APMErrorType::FileWriteError => "File Write Error",
            APMErrorType::ZIPOpenError => "ZIP Open Error",
            APMErrorType::ZIPArchiveOpenError => "ZIP Archive Open Error",
            APMErrorType::ZIPCreationError => "ZIP Creation Error",
            APMErrorType::ZIPFinishError => "ZIP Finish Error",
            APMErrorType::ZIPModificationError => "ZIP Modification Error",
            APMErrorType::ZIPArchiveHiddenNotFoundError => {
                "ZIP Archive, Hidden File Not Found Error"
            }
            APMErrorType::ZIPArchiveReadError => "ZIP Archive Read Error",
            APMErrorType::ZIPAddDirectoryError => "ZIP Add Directory Error",
            APMErrorType::ZIPStartFileError => "ZIP Start File Error",
            APMErrorType::ZIPFileReadError => "ZIP File Read Error",
            APMErrorType::ZIPFileWriteError => "ZIP File Write Error",
            APMErrorType::HashUTF8Error => "Hash was not UTF-8 Error",
        };

        return write!(f, "{}", s);
    }
}

impl APMError {
    pub fn new(tp: APMErrorType, msg: String) -> Self {
        return Self { tp, msg };
    }

    pub fn error_type(&self) -> String {
        return self.tp.to_string();
    }

    pub fn description(&self) -> &str {
        return &self.msg;
    }
}
