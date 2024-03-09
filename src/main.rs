use tree_sitter::Parser;
fn main() {
    let code = r#"
  echo "Hello, world!"
  cat file
"#;
    let mut parser = Parser::new();
    let bash_language = tree_sitter_bash::language();
    parser
        .set_language(bash_language)
        .expect("Error loading Bash grammar");
    let parsed = match parser.parse(code, None) {
        Some(parsed) => parsed,
        None => {
            println!("Error parsing code");
            std::process::exit(1);
        }
    };
    let query = tree_sitter::Query::new(
        bash_language,
        r#"
(command
    name:
        (command_name
            (word) @command_name))
    "#,
    )
    .unwrap();

    let mut query_cursor = tree_sitter::QueryCursor::new();
    let all_matches = query_cursor.matches(&query, parsed.root_node(), code.as_bytes());

    let command_name_index = query.capture_index_for_name("command_name").unwrap();
    for each_match in all_matches {
        for capture in each_match
            .captures
            .iter()
            .filter(|c| c.index == command_name_index)
        {
            let range = capture.node.byte_range();
            let text = &code[range.start..range.end];
            println!("Command: {:?}", text);
        }
    }
}
