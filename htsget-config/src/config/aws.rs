use serde;
use serde::{Deserialize, Serialize};

/// Configuration for the htsget server.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(default)]
pub struct AwsS3DataServer {
  pub bucket: String,
}
