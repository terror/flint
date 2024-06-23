use crate::common::*;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Severity {
  Allow,
  Deny,
  Warn,
}
