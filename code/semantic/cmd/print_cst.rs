#![allow(missing_docs)]
use tree_sitter::{
    Parser,
    TreeCursor,
};

static RUST_SRC: &str = r#"
use std::env;

fn main() {
    something(env::var("HOME").unwrap());
    something(env::var("PATH").unwrap());
}

fn something<T: AsRef<str>>(v: T) {
    // comment
    println!("{}", v.as_ref());
}
"#;

fn main() {
    let mut parser = Parser::new();
    parse_rust(&mut parser, RUST_SRC);
}

fn parse_rust<T: AsRef<[u8]>>(parser: &mut Parser, rust_src: T) {
    let lang = &tree_sitter_rust::LANGUAGE.into();
    parser.set_language(lang).expect("error loading a grammar");

    let cst = parser.parse(&rust_src, None).unwrap();
    let root = cst.root_node();
    println!("{:?}", root);

    let mut cur = root.walk();
    visit(&mut cur, 0, &rust_src);
}

fn visit<T: AsRef<[u8]>>(cursor: &mut TreeCursor, depth: usize, src: &T) {
    let indent = 2 * depth;
    print!("{:>indent$}", "");

    let node = cursor.node();

    println!(
        "{} [{}-{}] \"{}\"",
        node.kind(),
        node.start_position(),
        node.end_position(),
        node.utf8_text(src.as_ref()).unwrap()
    );

    if cursor.goto_first_child() {
        visit(cursor, depth + 1, src);
        while cursor.goto_next_sibling() {
            visit(cursor, depth + 1, src);
        }
        cursor.goto_parent();
    }
}
