use rustop::opts;
use tree_sitter::Parser;

fn main() {
    let (args, _) = opts! {
        synopsis r#"This program extracts all command in a given Bash script.
The output format is {command} {line} {column}.
"#;
        param file:String, desc:"The file to extract commands from.";
    }
    .parse_or_exit();

    let code = std::fs::read_to_string(args.file).expect("Error reading file");

    let mut parser = Parser::new();
    let bash_language = tree_sitter_bash::language();
    parser
        .set_language(bash_language)
        .expect("Error loading Bash grammar");
    let parsed = match parser.parse(&code, None) {
        Some(parsed) => parsed,
        None => {
            eprintln!("Error parsing code");
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
            let byte_range = capture.node.byte_range();
            let range = capture.node.range();
            let text = &code[byte_range.start..byte_range.end];
            let line = range.start_point.row + 1;
            let col = range.start_point.column + 1;
            println!("{} {} {}", text, line, col);
        }
    }
}
