use crate::common::*;

pub(crate) struct QueryConfig<'a> {
  pub(crate) name: &'a str,
  pub(crate) rule: Rule,
  pub(crate) source: &'a str,
}
