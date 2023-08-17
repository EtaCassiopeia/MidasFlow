use crate::ingestion_types::{LocalStorage, S3Storage};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, ::prost::Message, Hash)]
pub struct Connection {
    #[prost(oneof = "ConnectionConfig", tags = "1,2")]
    pub config: Option<ConnectionConfig>,
    #[prost(string, tag = "3")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, ::prost::Oneof, Hash)]
pub enum ConnectionConfig {
    #[prost(message, tag = "1")]
    /// In yaml, present as tag: `!LocalStorage`
    LocalStorage(LocalStorage),
    #[prost(message, tag = "2")]
    /// In yaml, present as tag: `!S3Storage`
    S3Storage(S3Storage),
}
