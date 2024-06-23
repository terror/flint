use crate::common::*;

#[derive(Clap)]
pub(crate) struct Arguments {
  #[clap(flatten)]
  walker_options: WalkerOptions,
}

impl Arguments {
  pub(crate) fn run(self) -> Result {
    Walker::new(self.walker_options)?
      .files()
      .try_for_each(|path| {
        let guesser = Guesser::new(path.clone())?;

        if let Some(language) = guesser.guess()? {
          let sources = Config::load()?
            .checkers
            .iter()
            .map(|path| fs::read_to_string(&path.expand()))
            .collect::<Result<Vec<_>, _>>()?;

          let checkers = sources
            .iter()
            .map(|source| serde_yaml::from_str::<Checker>(source))
            .collect::<Result<Vec<Checker>, _>>()?;

          return checkers
            .iter()
            .filter(|checker| checker.language == language)
            .try_for_each(|checker| checker.check(&path));
        }

        Ok(())
      })
  }
}
