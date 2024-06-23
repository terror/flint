use crate::common::*;

#[derive(Debug)]
pub(crate) struct Guesser {
  path: PathBuf,
  _source: String,
}

impl Guesser {
  pub(crate) fn new(path: PathBuf) -> Result<Self> {
    let source = fs::read_to_string(&path)?;
    Ok(Self {
      path,
      _source: source,
    })
  }

  pub(crate) fn guess(&self) -> Result<Option<Language>> {
    Ok(match self.path.extension() {
      Some(extension) => self.from_extension(
        extension
          .to_str()
          .ok_or("Failed to convert OsStr into str")?,
      ),
      None => None,
    })
  }

  fn from_extension(&self, extension: &str) -> Option<Language> {
    match extension {
      "c" => Some(C),
      "cpp" => Some(Cpp),
      "cs" => Some(CSharp),
      "rs" => Some(Rust),
      "sh" | "bash" => Some(Bash),
      "yaml" | "yml" => Some(Yaml),
      _ => None,
    }
  }
}
