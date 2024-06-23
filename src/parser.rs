use crate::common::*;

pub(crate) struct Parser {
  parser: TreeSitterParser,
}

impl Parser {
  pub(crate) fn new(language: Language) -> Result<Self> {
    let mut parser = TreeSitterParser::new();

    parser.set_language(language.into())?;

    Ok(Self { parser })
  }

  fn parse(&mut self, source: &str, old_tree: Option<&Tree>) -> Option<Tree> {
    self.parser.parse(source, old_tree)
  }

  pub(crate) fn query(&mut self, config: QueryConfig) -> Result {
    let source = fs::read_to_string(config.path.clone())?;

    let tree = self.parse(&source, None).ok_or("Failed to parse source")?;

    let query = Query::new(
      self
        .parser
        .language()
        .ok_or("Failed to retrieve parser language")?,
      &config.rule.query,
    )?;

    let captures = config
      .rule
      .captures
      .iter()
      .map(|capture| {
        query
          .capture_index_for_name(capture)
          .ok_or("Couldn't find capture index for name")
      })
      .collect::<Result<Vec<u32>, &str>>()?;

    QueryCursor::new()
      .matches(&query, tree.root_node(), source.as_bytes())
      .for_each(|item| {
        item
          .captures
          .iter()
          .filter(|capture| captures.contains(&capture.index))
          .for_each(|capture| {
            let range = capture.node.range();

            let padding = range.start_point.row.to_string().len() + 1;

            println!(
              "{}: {}\n --> {}\n{}|\n{} | {}{}\n{}|",
              "warning".yellow(),
              config.rule.message,
              config.path.display(),
              format_args!("{:1$}", " ", padding),
              range.start_point.row,
              format_args!("{:1$}", " ", range.start_point.column),
              &source[range.start_byte..range.end_byte],
              format_args!("{:1$}", " ", padding),
            );
          });
      });

    Ok(())
  }
}
