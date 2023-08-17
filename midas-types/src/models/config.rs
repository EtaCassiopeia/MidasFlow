use crate::models::connection::Connection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, prost::Message)]
/// The configuration for the app
pub struct Config {
    #[prost(string, tag = "2")]
    /// name of the app
    pub app_name: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[prost(message, repeated, tag = "5")]
    /// connections to databases: Eg: Postgres, Snowflake, etc
    pub connections: Vec<Connection>,
}
