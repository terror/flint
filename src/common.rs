pub(crate) use std::{collections::HashMap, fs, path::PathBuf, process};

pub(crate) use {
  clap::Parser as Clap,
  colored::*,
  console::{style, StyledObject},
  dirs::home_dir,
  serde::Deserialize,
  std::fmt::{self, Display, Formatter},
  tree_sitter::{
    Language as TreeSitterLanguage, Parser as TreeSitterParser, Query,
    QueryCursor, Tree,
  },
  walkdir::WalkDir,
  xdg::BaseDirectories,
};

extern "C" {
  pub(crate) fn tree_sitter_bash() -> TreeSitterLanguage;
  pub(crate) fn tree_sitter_c() -> TreeSitterLanguage;
  pub(crate) fn tree_sitter_c_sharp() -> TreeSitterLanguage;
  pub(crate) fn tree_sitter_cpp() -> TreeSitterLanguage;
  pub(crate) fn tree_sitter_rust() -> TreeSitterLanguage;
  pub(crate) fn tree_sitter_yaml() -> TreeSitterLanguage;
}

pub(crate) use crate::{
  arguments::Arguments,
  checker::Checker,
  config::Config,
  guesser::Guesser,
  language::{Language, Language::*},
  parser::Parser,
  path_ext::PathExt,
  query_config::QueryConfig,
  rule::Rule,
  severity::Severity,
  walker::{Walker, WalkerOptions},
};

pub(crate) type Error = Box<dyn std::error::Error>;
pub(crate) type Result<T = (), E = Error> = std::result::Result<T, E>;
