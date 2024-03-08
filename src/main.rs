use tree_sitter::{Parser, Tree};
fn main() {
    let code = r#"
  echo "Hello, world!"
  echo "Goodbye, world!"
"#;
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_bash::language())
        .expect("Error loading Bash grammar");
    let parsed = parser.parse(code, None);
}
