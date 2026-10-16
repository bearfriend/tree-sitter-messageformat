//! Tests for the host-language injection queries in `injections/`.
//!
//! Each query is run against its real host grammar over a fixture in
//! `test/host-injections/`. The set of injected strings must match exactly (so the
//! "not injected" lines in the fixtures are checked too), and every injected
//! string must parse without errors with the bare-message grammar.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Parser, Query, QueryCursor};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(rel: &str) -> String {
    fs::read_to_string(root().join(rel)).unwrap_or_else(|e| panic!("cannot read {rel}: {e}"))
}

/// Returns the (trimmed) text of every `@injection.content` capture whose
/// pattern sets `injection.language` to `messageformat_message`.
fn injected(host: Language, query_file: &str, fixture: &str) -> BTreeSet<String> {
    let source = read(&format!("test/host-injections/{fixture}"));
    // `a.scm+b.scm` concatenates query files, the way an editor layers them.
    let query_source: String = query_file
        .split('+')
        .map(|f| read(&format!("injections/{f}")))
        .collect::<Vec<_>>()
        .join("\n");
    let query = Query::new(&host, &query_source)
        .unwrap_or_else(|e| panic!("{query_file} does not compile: {e}"));

    let mut parser = Parser::new();
    parser.set_language(&host).unwrap();
    let tree = parser.parse(&source, None).unwrap();
    assert!(!tree.root_node().has_error(), "{fixture} has syntax errors: {}", tree.root_node().to_sexp());

    let content = query.capture_index_for_name("injection.content").expect("@injection.content");
    let mut out = BTreeSet::new();
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), source.as_bytes());
    while let Some(m) = matches.next() {
        let language = query
            .property_settings(m.pattern_index)
            .iter()
            .find(|p| &*p.key == "injection.language")
            .and_then(|p| p.value.as_deref());
        assert_eq!(language, Some("messageformat_message"), "pattern {} in {query_file}", m.pattern_index);
        for capture in m.captures().iter().filter(|c| c.index == content) {
            out.insert(source[capture.node.byte_range()].trim().to_string());
        }
    }
    out
}

fn assert_injects(host: Language, query_file: &str, fixture: &str, expected: &[&str]) {
    let got = injected(host, query_file, fixture);
    let want: BTreeSet<String> = expected.iter().map(|s| s.to_string()).collect();
    assert_eq!(got, want, "{fixture}: injected strings differ");

    // Whatever we inject must be a valid message.
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_messageformat::LANGUAGE_MESSAGE.into()).unwrap();
    for text in &got {
        let tree = parser.parse(text, None).unwrap();
        assert!(!tree.root_node().has_error(), "{fixture}: `{text}` -> {}", tree.root_node().to_sexp());
    }
}

const JS_EXPECTED: &[&str] = &[
    "You have {count, plural, one {# item} other {# items}}",
    "Hello {name, select, a {A} other {B}}",
    "Plain {n} text",
    "Hi {name}!",
    "Welcome, {user}!",
    "Bye {who}",
    "Welcome to {site}!",
    "{n, number} items",
];

#[test]
fn javascript() {
    assert_injects(tree_sitter_javascript::LANGUAGE.into(), "javascript.scm+jsx.scm", "sample.js", JS_EXPECTED);
}

#[test]
fn typescript() {
    assert_injects(
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        "javascript.scm",
        "sample.ts",
        &[
            "You have {count, plural, one {# item} other {# items}}",
            "Plain {n} text",
            "Welcome, {user}!",
        ],
    );
}

#[test]
fn tsx() {
    assert_injects(
        tree_sitter_typescript::LANGUAGE_TSX.into(),
        "javascript.scm+jsx.scm",
        "sample.tsx",
        &["Welcome to {site}, {name}!", "{n, number} items"],
    );
}

#[test]
fn json() {
    assert_injects(
        tree_sitter_json::LANGUAGE.into(),
        "json.scm",
        "sample.json",
        &[
            "{count, plural, one {# file} other {# files}}",
            "Plain {name} here",
            "Hi {who}",
            "{n, number} items",
        ],
    );
}

#[test]
fn html() {
    assert_injects(
        tree_sitter_html::LANGUAGE.into(),
        "html.scm",
        "sample.html",
        &[
            "{n, plural, one {a} other {b}}",
            "You have {count, plural, one {# item} other {# items}} and plain {x}",
        ],
    );
}

#[test]
fn java() {
    assert_injects(
        tree_sitter_java::LANGUAGE.into(),
        "java.scm",
        "Sample.java",
        &[
            "Hello {0}",
            "Bye {0}",
            "{0, plural, one {# file} other {# files}}",
            "{0, select, a {A} other {B}}",
        ],
    );
}

#[test]
fn kotlin() {
    assert_injects(
        tree_sitter_kotlin_ng::LANGUAGE.into(),
        "kotlin.scm",
        "sample.kt",
        &[
            "Hello {0}",
            "Bye {0}",
            "{0, plural, one {# file} other {# files}}",
            "{0, select, a {A} other {B}}",
        ],
    );
}

#[test]
fn swift() {
    assert_injects(
        tree_sitter_swift::LANGUAGE.into(),
        "swift.scm",
        "sample.swift",
        &[
            "{count, plural, one {# file} other {# files}}",
            "{n, select, a {A} other {B}}",
        ],
    );
}

#[test]
fn every_injection_query_has_a_test() {
    let tested: BTreeSet<&str> = ["javascript.scm", "jsx.scm", "json.scm", "html.scm", "java.scm", "kotlin.scm", "swift.scm"].into();
    let present: BTreeSet<String> = fs::read_dir(root().join("injections"))
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    let present: BTreeSet<&str> = present.iter().map(String::as_str).collect();
    assert_eq!(present, tested, "add a fixture and test for each file in injections/");
}

#[test]
fn jsx_queries_do_not_compile_for_plain_typescript() {
    // Documents why jsx.scm is a separate file.
    let host: Language = tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into();
    assert!(Query::new(&host, &read("injections/jsx.scm")).is_err());
    assert!(Query::new(&host, &read("injections/javascript.scm")).is_ok());
}
