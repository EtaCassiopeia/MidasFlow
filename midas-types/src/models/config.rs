use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, prost::Message)]
/// The configuration for the app
pub struct Config {
    #[prost(string, tag = "2")]
    /// name of the app
    pub app_name: String,
}
