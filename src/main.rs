use rustop::opts;
use tree_sitter::{Language, Node, Parser, Range};

fn run_query(
    query_str: &str,
    match_name: &str,
    lang: &Language,
    node: &Node,
    code: &[u8],
) -> Vec<(std::ops::Range<usize>, Range)> {
    let query = tree_sitter::Query::new(*lang, query_str).unwrap();

    let mut query_cursor = tree_sitter::QueryCursor::new();
    let all_matches = query_cursor.matches(&query, *node, code);

    let command_name_index = query.capture_index_for_name(match_name).unwrap();
    let mut commands = Vec::new();
    for each_match in all_matches {
        for capture in each_match
            .captures
            .iter()
            .filter(|c| c.index == command_name_index)
        {
            let byte_range = capture.node.byte_range();
            let range = capture.node.range();
            commands.push((byte_range, range));
        }
    }
    commands
}

fn main() {
    let (args, _) = opts! {
        synopsis r#"This program extracts all command in a given Bash script.
The output format is {command} {line} {column}.
"#;
        opt var_cmd:bool=false, desc:"Extract variables storing command names";
        opt version:bool=false, desc:"Show version";
        param file:Vec<String>, desc:"The file to extract commands from.";
    }
    .parse_or_exit();

    if args.version {
        println!("{}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }

    let mut parser = Parser::new();
    let bash_language = tree_sitter_bash::language();
    parser
        .set_language(bash_language)
        .expect("Error loading Bash grammar");

    for file in &args.file {
        let code = match std::fs::read_to_string(file) {
            Ok(code) => code,
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                std::process::exit(1);
            }
        };
        let code_bytes = code.as_bytes();

        let parsed = match parser.parse(&code, None) {
            Some(parsed) => parsed,
            None => {
                eprintln!("Error parsing code");
                std::process::exit(1);
            }
        };

        let mut command_ranges = run_query(
            r#"
    (command
        name:
            (command_name
                (word) @command_name))
        "#,
            "command_name",
            &bash_language,
            &parsed.root_node(),
            code_bytes,
        );
        if args.var_cmd {
            command_ranges.append(&mut run_query(
                r#"
        (command
            name: (command_name
              (simple_expansion
                (variable_name)) @variable))
                "#,
                "variable",
                &bash_language,
                &parsed.root_node(),
                code_bytes,
            ));
        }

        command_ranges.sort_by(|(byte_range1, _), (byte_range2, _)| {
            if byte_range1.start != byte_range2.start {
                byte_range1.start.cmp(&byte_range2.start)
            } else {
                byte_range1.end.cmp(&byte_range2.end)
            }
        });

        for (byte_range, range) in command_ranges {
            let text = &code[byte_range.start..byte_range.end];
            let line = range.start_point.row + 1;
            let col = range.start_point.column + 1;
            println!("{} {} {}", text, line, col);
        }
    }
}
