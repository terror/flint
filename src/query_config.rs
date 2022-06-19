use crate::common::*;

pub(crate) struct QueryConfig<'a> {
  pub(crate) name: &'a str,
  pub(crate) path: PathBuf,
  pub(crate) rule: Rule,
}
