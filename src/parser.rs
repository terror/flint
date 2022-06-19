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
    let query = Query::new(
      self
        .parser
        .language()
        .ok_or("Failed to retrieve parser language")?,
      &config.rule.query,
    )?;

    let tree = self
      .parse(&config.source, None)
      .ok_or("Failed to parse source")?;

    let mut cursor = QueryCursor::new();

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

    for item in
      cursor.matches(&query, tree.root_node(), config.source.as_bytes())
    {
      for capture in item
        .captures
        .iter()
        .filter(|capture| captures.contains(&capture.index))
      {
        let range = capture.node.range();

        println!(
          "[Line: {}, Col: {}] {} - {}: `{}`",
          range.start_point.row,
          range.start_point.column,
          config.name,
          config.rule.message,
          &config.source[range.start_byte..range.end_byte]
        );
      }
    }

    Ok(())
  }
}
