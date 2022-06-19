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
        let language = Guesser::new(path.clone())?
          .guess()?
          .ok_or("Unsupported file type")?;

        let config = Config::load()?;

        let checkers = config
          .checkers
          .iter()
          .map(|path| {
            serde_yaml::from_str::<Checker>(
              &fs::read_to_string(&path.expand()).unwrap(),
            )
            .unwrap()
          })
          .filter(|checker| checker.language == language)
          .collect::<Vec<Checker>>();

        checkers.iter().try_for_each(|checker| {
          let mut parser = Parser::new(checker.language.clone())?;

          let source = fs::read_to_string(path.clone())?;

          checker.rules.iter().try_for_each(|rule| {
            parser.query(QueryConfig {
              name: rule.0,
              rule: rule.1.clone(),
              source: source.as_ref(),
            })
          })
        })
      })
  }
}
