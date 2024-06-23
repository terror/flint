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

            println!(
              "{} {} [{}]",
              config.rule.severity.style(),
              config.rule.message,
              config.rule.category
            );

            println!(" --> {}", config.path.display());
            println!("     |");

            let line_start =
              source[..range.start_byte].rfind('\n').map_or(0, |i| i + 1);

            let line_end = source[range.end_byte..]
              .find('\n')
              .map_or(source.len(), |i| i + range.end_byte);

            let full_line = &source[line_start..line_end];

            println!("{:4} | {}", range.start_point.row + 1, full_line);

            let caret_start = range.start_point.column;
            let caret_length = range.end_byte - range.start_byte;

            let mut underline = String::from(" ".repeat(caret_start));
            underline.push_str(&"^".repeat(caret_length));

            println!("     | {}", style(underline).blue());
            println!();
          });
      });

    Ok(())
  }
}
