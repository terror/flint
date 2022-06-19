use crate::common::*;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub(crate) enum Language {
  Bash,
  C,
  CSharp,
  Cpp,
  Rust,
  Yaml,
}

impl Into<TreeSitterLanguage> for Language {
  fn into(self) -> TreeSitterLanguage {
    match self {
      Bash => unsafe { tree_sitter_bash() },
      C => unsafe { tree_sitter_c() },
      CSharp => unsafe { tree_sitter_c_sharp() },
      Cpp => unsafe { tree_sitter_cpp() },
      Rust => unsafe { tree_sitter_rust() },
      Yaml => unsafe { tree_sitter_yaml() },
    }
  }
}
