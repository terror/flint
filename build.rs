use {cc::Build, std::path::PathBuf};

struct Parser<'a> {
  name: &'a str,
  src: &'a str,
  extra: Vec<&'a str>,
}

impl<'a> Parser<'a> {
  fn build(&self) {
    let path = PathBuf::from(self.src);

    let mut files = vec!["parser.c"];
    files.extend(self.extra.clone());

    let mut build = Build::new();
    build.include(&path).warnings(false);

    files.iter().for_each(|file| {
      build.file(&path.join(file));
    });

    build.compile(self.name);
  }
}

fn main() {
  let parsers = vec![Parser {
    name: "tree-sitter-rust",
    src: "vendor/tree-sitter-rust/src",
    extra: vec!["scanner.c"],
  }];

  for parser in &parsers {
    println!("cargo:rerun-if-changed={}", parser.src);
  }

  parsers.iter().for_each(|parser| parser.build());
}
