use crate::common::*;

pub(crate) trait PathExt {
  fn expand(&self) -> PathBuf;
  fn validate(self) -> Result<Self>
  where
    Self: Sized;
}

impl PathExt for PathBuf {
  fn expand(&self) -> PathBuf {
    PathBuf::from(
      shellexpand::tilde(&self.to_str().unwrap_or_default()).to_string(),
    )
  }

  fn validate(self) -> Result<Self> {
    match self.exists() {
      true => Ok(self),
      _ => Err(format!("Path `{}` does not exist", self.display()))?,
    }
  }
}
