use crate::common::*;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Rule {
  pub(crate) severity: Severity,
  pub(crate) category: String,
  pub(crate) message: String,
  pub(crate) query: String,
  pub(crate) captures: Vec<String>,
}
