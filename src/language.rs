use crate::common::*;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) enum Language {
  Rust,
}

impl Into<TreeSitterLanguage> for Language {
  fn into(self) -> TreeSitterLanguage {
    match self {
      Rust => unsafe { tree_sitter_rust() },
    }
  }
}
