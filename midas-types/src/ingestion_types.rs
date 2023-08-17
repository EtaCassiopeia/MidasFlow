use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, ::prost::Message, Hash)]
pub struct Table {
    #[prost(oneof = "TableConfig", tags = "1,2,3")]
    pub config: Option<TableConfig>,
    #[prost(string, tag = "4")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, ::prost::Oneof, Hash)]
pub enum TableConfig {
    #[prost(message, tag = "1")]
    CSV(CsvConfig),
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, ::prost::Message, Hash)]
pub struct CsvConfig {
    #[prost(string, tag = "1")]
    pub path: String,
    #[prost(string, tag = "2")]
    pub extension: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, ::prost::Message, Hash)]
pub struct LocalDetails {
    #[prost(string, tag = "1")]
    pub path: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, ::prost::Message, Hash)]
pub struct LocalStorage {
    #[prost(message, optional, tag = "1")]
    pub details: Option<LocalDetails>,
    #[prost(message, repeated, tag = "2")]
    pub tables: Vec<Table>,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, ::prost::Message, Hash)]
pub struct S3Details {
    #[prost(string, tag = "1")]
    pub access_key_id: String,
    #[prost(string, tag = "2")]
    pub secret_access_key: String,
    #[prost(string, tag = "3")]
    pub region: String,
    #[prost(string, tag = "4")]
    pub bucket_name: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, ::prost::Message, Hash)]
pub struct S3Storage {
    #[prost(message, optional, tag = "1")]
    pub details: Option<S3Details>,
    #[prost(message, repeated, tag = "2")]
    pub tables: Vec<Table>,
}
