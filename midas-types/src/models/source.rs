use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, ::prost::Message)]
pub struct Source {
    #[prost(string, tag = "1")]
    /// name of the source - to distinguish between multiple sources; Type: String
    pub name: String,
    #[prost(string, tag = "2")]
    /// name of the table in source database; Type: String
    pub table_name: String,
    #[prost(string, repeated, tag = "3")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// list of columns gonna be used in the source table; Type: String[]
    pub columns: Vec<String>,
    #[prost(string, tag = "4")]
    /// reference to pre-defined connection name; Type: String
    pub connection: String,
}