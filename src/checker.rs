use crate::common::*;

#[derive(Debug, Deserialize)]
pub(crate) struct Checker {
  pub(crate) name: String,
  pub(crate) language: Language,
  pub(crate) rules: HashMap<String, Rule>,
}
