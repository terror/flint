use crate::common::*;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
  pub(crate) checkers: Vec<PathBuf>,
}

impl Config {
  pub(crate) fn load() -> Result<Self> {
    Ok(serde_yaml::from_str::<Self>(&fs::read_to_string(
      &Self::path()?.ok_or("Failed to find configuration file")?,
    )?)?)
  }

  fn filename() -> &'static str {
    ".flint.yaml"
  }

  fn path() -> Result<Option<PathBuf>> {
    Ok(
      BaseDirectories::with_prefix(home_dir().unwrap_or_default())?
        .find_config_file(Self::filename()),
    )
  }
}
