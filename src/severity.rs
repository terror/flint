use crate::common::*;

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Severity {
  Allow,
  Deny,
  Warn,
}

impl Display for Severity {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Severity::Allow => "allow",
        Severity::Deny => "deny",
        Severity::Warn => "warning",
      }
    )
  }
}

impl Severity {
  pub(crate) fn style(&self) -> StyledObject<String> {
    match self {
      Severity::Allow => style(format!("{}:", self.to_string())).green(),
      Severity::Deny => style(format!("{}:", self.to_string())).red().bold(),
      Severity::Warn => style(format!("{}:", self.to_string())).yellow(),
    }
  }
}
