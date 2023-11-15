#[derive(PartialEq, Debug, Hash, Clone, Copy)]
pub enum DataFormat {
    XML,
    #[cfg(feature = "json_exporter")]
    JSON,
    #[cfg(feature = "toml_exporter")]
    TOML,
}

pub trait IntoDataFormat
where
    Self: Sized,
{
    type Error;

    fn into_data_format_string(&self, df: DataFormat) -> Result<String, Self::Error> {
        return match df {
            DataFormat::XML => self.into_xml_string(),
            #[cfg(feature = "json_exporter")]
            DataFormat::JSON => self.into_json_string(),
            #[cfg(feature = "toml_exporter")]
            DataFormat::TOML => self.into_toml_string(),
        };
    }

    fn into_xml_string(&self) -> Result<String, Self::Error>;

    #[cfg(feature = "json_exporter")]
    fn into_json_string(&self) -> Result<String, Self::Error>;

    #[cfg(feature = "toml_exporter")]
    fn into_toml_string(&self) -> Result<String, Self::Error>;
}
