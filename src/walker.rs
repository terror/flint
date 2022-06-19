use crate::common::*;

#[derive(Debug)]
pub(crate) struct Walker {
  path: PathBuf,
  recursive: bool,
}

#[derive(Debug, Clap, Clone)]
pub(crate) struct WalkerOptions {
  #[clap(help = "A file or directory path to check.")]
  pub(crate) path: Option<PathBuf>,
  #[clap(long, help = "Recursively check files.")]
  pub(crate) recursive: bool,
}

impl Walker {
  pub(crate) fn new(options: WalkerOptions) -> Result<Self> {
    Ok(Self {
      path: options
        .path
        .unwrap_or(std::env::current_dir()?)
        .validate()?,
      recursive: options.recursive,
    })
  }

  pub(crate) fn files(&self) -> impl Iterator<Item = PathBuf> {
    let mut walker = WalkDir::new(&self.path);

    if !self.recursive {
      walker = walker.max_depth(1);
    }

    walker
      .into_iter()
      .filter_map(|entry| entry.ok())
      .map(|entry| entry.into_path())
      .filter(|entry| entry.is_file())
  }
}
