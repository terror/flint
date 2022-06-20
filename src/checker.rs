use crate::common::*;

#[derive(Debug, Deserialize)]
pub(crate) struct Checker {
  pub(crate) name: String,
  pub(crate) language: Language,
  pub(crate) rules: HashMap<String, Rule>,
}

impl Checker {
  pub(crate) fn check(&self, path: &PathBuf) -> Result {
    let mut parser = Parser::new(self.language.clone())?;

    self.rules.iter().try_for_each(|rule| {
      parser.query(QueryConfig {
        name: rule.0,
        path: path.to_owned(),
        rule: rule.1.to_owned(),
      })
    })
  }
}
