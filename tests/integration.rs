use executable_path::executable_path;

// Load example linters from /examples
//
// For each linter, we can write a test
// that targets certain rules, then check
// the stdout.
//
// The binary looks for config files

#[derive(Debug)]
struct Test {}

impl Test  {
  fn new() -> Self {
    Test {}
  }
}

/*
  Test::new()
    .source(
      'main.rs',
      '<source>'
     )
    .expected_stdout()
    .expected_status()
    .expected_stderr()
    .run()?;
 */
