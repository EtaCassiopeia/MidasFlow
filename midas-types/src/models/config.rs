use crate::models::connection::Connection;
use serde::{Deserialize, Serialize};
use crate::models::source::Source;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, prost::Message)]
/// The configuration for the app
pub struct Config {
    #[prost(string, tag = "2")]
    /// name of the app
    pub app_name: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[prost(message, repeated, tag = "3")]
    /// connections data sources
    pub connections: Vec<Connection>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[prost(message, repeated, tag = "4")]
    /// sources to ingest data related to particular connection
    pub sources: Vec<Source>,
}
