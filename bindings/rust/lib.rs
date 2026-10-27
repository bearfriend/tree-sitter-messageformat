//! This crate provides MessageFormat language support for the [tree-sitter] parsing library.
//!
//! Typically, you will use the [`LANGUAGE`] constant to add this language to a
//! tree-sitter [`Parser`], and then use the parser to parse some code:
//!
//! ```
//! let code = r#"
//! greeting: "Hello {name}!"
//! "#;
//! let mut parser = tree_sitter::Parser::new();
//! let language = tree_sitter_messageformat::LANGUAGE;
//! parser
//!     .set_language(&language.into())
//!     .expect("Error loading messageformat parser");
//! let tree = parser.parse(code, None).unwrap();
//! assert!(!tree.root_node().has_error());
//! ```
//!
//! [`Parser`]: https://docs.rs/tree-sitter/0.25.6/tree_sitter/struct.Parser.html
//! [tree-sitter]: https://tree-sitter.github.io/

use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_messageformat() -> *const ();
    fn tree_sitter_messageformat_message() -> *const ();
}

/// The tree-sitter [`LanguageFn`] for this grammar.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_messageformat) };

/// The tree-sitter [`LanguageFn`] for the bare-message grammar.
///
/// It shares all rules with [`LANGUAGE`] but its root is a single message
/// (text, placeholders, plural/select, tags) instead of a `key: value` resource
/// file, which is what the contents of a string literal in another language is.
/// Use it for language injections.
pub const LANGUAGE_MESSAGE: LanguageFn =
    unsafe { LanguageFn::from_raw(tree_sitter_messageformat_message) };

/// The content of the [`node-types.json`] file for this grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers/6-static-node-types
pub const NODE_TYPES: &str = include_str!("../../src/node-types.json");

pub const HIGHLIGHTS_QUERY: &str = include_str!("../../queries/highlights.scm");
pub const INJECTIONS_QUERY: &str = include_str!("../../queries/injections.scm");
pub const LOCALS_QUERY: &str = include_str!("../../queries/locals.scm");
pub const TAGS_QUERY: &str = include_str!("../../queries/tags.scm");
pub const FOLDS_QUERY: &str = include_str!("../../queries/folds.scm");

/// The content of the `node-types.json` file for the bare-message grammar.
pub const MESSAGE_NODE_TYPES: &str = include_str!("../../message/src/node-types.json");
pub const MESSAGE_HIGHLIGHTS_QUERY: &str = include_str!("../../message/queries/highlights.scm");
pub const MESSAGE_BRACKETS_QUERY: &str = include_str!("../../message/queries/brackets.scm");

#[cfg(test)]
mod tests {
    use tree_sitter::{Parser, Language};

    fn setup_parser() -> Parser {
        let mut parser = Parser::new();
        let language: Language = super::LANGUAGE.into();
        parser.set_language(&language).expect("Error loading messageformat parser");
        parser
    }

    #[test]
    fn test_can_load_grammar() {
        setup_parser();
    }

    fn parse_message(code: &str) -> tree_sitter::Tree {
        let mut parser = Parser::new();
        let language: Language = super::LANGUAGE_MESSAGE.into();
        parser.set_language(&language).expect("Error loading message parser");
        parser.parse(code, None).expect("Failed to parse")
    }

    #[test]
    fn test_message_grammar_parses_bare_messages() {
        for code in [
            "Hello, world!",
            "Hello {name}!",
            "{0} of {1, number, integer}",
            "{n, plural, one {# item} other {# items}} <b>x</b> it''s '{x}'",
            "See http://example.com for details",
        ] {
            let tree = parse_message(code);
            assert!(!tree.root_node().has_error(), "{code}: {}", tree.root_node().to_sexp());
        }
    }

    #[test]
    fn test_message_grammar_is_not_a_resource_file_grammar() {
        let tree = parse_message("greeting: \"Hello {name}!\"");
        assert_eq!(tree.root_node().kind(), "source_file");
        let sexp = tree.root_node().to_sexp();
        assert!(!sexp.contains("(resource"), "{sexp}");
        assert!(sexp.contains("(placeholder"), "{sexp}");
    }

    #[test]
    fn test_message_highlights_capture_expected_nodes() {
        use streaming_iterator::StreamingIterator;
        let code = "Hi {n, plural, one {# it} other {# its}} <b>x</b> it''s";
        let tree = parse_message(code);
        let language: Language = super::LANGUAGE_MESSAGE.into();
        let query = tree_sitter::Query::new(&language, super::MESSAGE_HIGHLIGHTS_QUERY).unwrap();
        let mut cursor = tree_sitter::QueryCursor::new();
        let mut seen = std::collections::BTreeSet::new();
        let mut captures = cursor.captures(&query, tree.root_node(), code.as_bytes());
        while let Some((m, i)) = captures.next() {
            let c = m.captures()[*i];
            seen.insert((query.capture_names()[c.index as usize].to_string(), code[c.node.byte_range()].to_string()));
        }
        for expected in [("variable", "n"), ("keyword", "plural"), ("variable.special", "#"), ("tag", "b"), ("string.escape", "''")] {
            assert!(seen.contains(&(expected.0.to_string(), expected.1.to_string())), "missing {expected:?} in {seen:?}");
        }
    }

    /// The innermost capture name for every byte of `code`, the way an editor resolves
    /// nested captures (smaller ranges win over the ranges that contain them).
    fn message_capture_per_byte(code: &str) -> Vec<Option<String>> {
        use streaming_iterator::StreamingIterator;
        let tree = parse_message(code);
        let language: Language = super::LANGUAGE_MESSAGE.into();
        let query = tree_sitter::Query::new(&language, super::MESSAGE_HIGHLIGHTS_QUERY).unwrap();
        let mut cursor = tree_sitter::QueryCursor::new();
        let mut spans = Vec::new();
        let mut captures = cursor.captures(&query, tree.root_node(), code.as_bytes());
        while let Some((m, i)) = captures.next() {
            let c = m.captures()[*i];
            spans.push((c.node.byte_range(), query.capture_names()[c.index as usize].to_string()));
        }
        spans.sort_by_key(|(r, _)| std::cmp::Reverse(r.len()));
        let mut per_byte = vec![None; code.len()];
        for (range, name) in spans {
            for b in range {
                per_byte[b] = Some(name.clone());
            }
        }
        per_byte
    }

    #[test]
    fn test_message_highlights_plain_text_is_a_string() {
        // Regression: plain text, including text inside plural/select bodies and
        // between tags, had no capture at all and rendered unhighlighted.
        let code = "Hello {name}, you have {n, plural, one {# item} other {# items}} \
                    <b>now</b>! {g, select, male {He} other {They}} left Bob's";
        let per_byte = message_capture_per_byte(code);
        for word in ["Hello", "you have", "item", "items", "now", "!", "He", "They", "left", "Bob", "'", "s"] {
            // `rfind`, so `s` and `'` hit the apostrophe at the end, not earlier letters.
            let start = code.rfind(word).unwrap_or_else(|| panic!("{word} not in sample"));
            for b in start..start + word.len() {
                assert_eq!(
                    per_byte[b].as_deref(),
                    Some("string"),
                    "`{word}` byte {b} ({:?}) is not highlighted as a string",
                    &code[b..b + 1]
                );
            }
        }
        // Inner captures still win over the outer string.
        let at = |needle: &str| per_byte[code.find(needle).unwrap()].as_deref().map(String::from);
        assert_eq!(at("name"), Some("variable".into()));
        assert_eq!(at("#"), Some("variable.special".into()));
        assert_eq!(at("plural"), Some("keyword".into()));
        assert_eq!(at("{n"), Some("punctuation.bracket".into()));
        assert_eq!(at("b>"), Some("tag".into()));
    }

    #[test]
    fn test_message_highlights_query_compiles() {
        let language: Language = super::LANGUAGE_MESSAGE.into();
        tree_sitter::Query::new(&language, super::MESSAGE_HIGHLIGHTS_QUERY)
            .expect("message highlights query should compile");
    }

    #[test]
    fn test_message_brackets_pair_braces_and_tags_but_not_apostrophes() {
        use streaming_iterator::StreamingIterator;
        let code = "it's Bob's {n} <b>x</b>";
        let tree = parse_message(code);
        let language: Language = super::LANGUAGE_MESSAGE.into();
        let query = tree_sitter::Query::new(&language, super::MESSAGE_BRACKETS_QUERY)
            .expect("message brackets query should compile");
        let mut cursor = tree_sitter::QueryCursor::new();
        let mut pairs = Vec::new();
        let mut matches = cursor.matches(&query, tree.root_node(), code.as_bytes());
        while let Some(m) = matches.next() {
            let open = m.captures()[0].node;
            let close = m.captures()[1].node;
            pairs.push((&code[open.byte_range()], &code[close.byte_range()]));
        }
        assert!(pairs.contains(&("{", "}")), "{pairs:?}");
        assert!(pairs.contains(&("<", ">")), "{pairs:?}");
        assert!(pairs.iter().all(|(o, _)| *o != "'"), "{pairs:?}");
    }

    #[test]
    fn test_simple_message() {
        let mut parser = setup_parser();
        let code = "greeting: \"Hello world!\"";
        let tree = parser.parse(code, None).expect("Failed to parse");

        let root = tree.root_node();
        assert_eq!(root.kind(), "source_file");
        assert!(!root.has_error());
        assert!(root.child_count() > 0);
    }

    #[test]
    fn test_placeholder_message() {
        let mut parser = setup_parser();
        let code = "greeting: \"Hello, {name}!\"";
        let tree = parser.parse(code, None).expect("Failed to parse");

        assert!(!tree.root_node().has_error());
    }

    #[test]
    fn test_plural_message() {
        let mut parser = setup_parser();
        let code = "items: \"{count, plural, =0 {no items} =1 {one item} other {# items}}\"";
        let tree = parser.parse(code, None).expect("Failed to parse");

        assert!(!tree.root_node().has_error());
    }

    #[test]
    fn test_select_message() {
        let mut parser = setup_parser();
        let code = "gender: \"{person, select, male {He} female {She} other {They}}\"";
        let tree = parser.parse(code, None).expect("Failed to parse");

        assert!(!tree.root_node().has_error());
    }

    #[test]
    fn test_selectordinal_message() {
        let mut parser = setup_parser();
        let code = "place: \"{position, selectordinal, =1 {1st} =2 {2nd} =3 {3rd} other {#th}}\"";
        let tree = parser.parse(code, None).expect("Failed to parse");

        assert!(!tree.root_node().has_error());
    }

    #[test]
    fn test_number_formatting() {
        let mut parser = setup_parser();
        let code = "price: \"Cost: {amount, number, currency}\"";
        let tree = parser.parse(code, None).expect("Failed to parse");

        assert!(!tree.root_node().has_error());
    }

    #[test]
    fn test_date_formatting() {
        let mut parser = setup_parser();
        let code = "birthday: \"Born on {date, date, ::MMM d, y}\"";
        let tree = parser.parse(code, None).expect("Failed to parse");

        assert!(!tree.root_node().has_error());
    }

    #[test]
    fn test_escaped_characters() {
        let mut parser = setup_parser();
        let code = "escaped: \"Use '{' and '}' for braces\"";
        let tree = parser.parse(code, None).expect("Failed to parse");

        assert!(!tree.root_node().has_error());
    }

    #[test]
    fn test_template_string() {
        let mut parser = setup_parser();
        let code = "multiline: `Hello {name}!\nWelcome back.`";
        let tree = parser.parse(code, None).expect("Failed to parse");

        assert!(!tree.root_node().has_error());
    }

    #[test]
    fn test_quoted_string() {
        let mut parser = setup_parser();
        let code = "name: \"{user}\"";
        let tree = parser.parse(code, None).expect("Failed to parse");

        assert!(!tree.root_node().has_error());
    }

    #[test]
    fn test_multiple_entries() {
        let mut parser = setup_parser();
        let code = "greeting: \"Hello!\",\nfarewell: \"Goodbye!\"";
        let tree = parser.parse(code, None).expect("Failed to parse");

        assert!(!tree.root_node().has_error());
        assert!(tree.root_node().child_count() >= 2);
    }

    #[test]
    fn test_comment() {
        let mut parser = setup_parser();
        let code = "// This is a comment\ngreeting: \"Hello!\"";
        let tree = parser.parse(code, None).expect("Failed to parse");

        assert!(!tree.root_node().has_error());
    }

    #[test]
    fn test_nested_complex_message() {
        let mut parser = setup_parser();
        let code = "nested: \"{count, plural, =0 {no {type, select, email {emails} sms {messages} other {items}}} other {# {type, select, email {emails} sms {messages} other {items}}}}\"";
        let tree = parser.parse(code, None).expect("Failed to parse");

        assert!(!tree.root_node().has_error());
    }

    #[test]
    fn test_node_types_available() {
        let node_types = super::NODE_TYPES;
        assert!(!node_types.is_empty());
        assert!(node_types.contains("source_file"));
        assert!(node_types.contains("plural"));
        assert!(node_types.contains("select"));
        assert!(node_types.contains("placeholder"));
        assert!(node_types.contains("template_string"));
        assert!(node_types.contains("quoted_string"));
    }

    #[test]
    fn test_empty_input() {
        let mut parser = setup_parser();
        let code = "";
        let tree = parser.parse(code, None).expect("Failed to parse");

        assert_eq!(tree.root_node().kind(), "source_file");
        assert_eq!(tree.root_node().child_count(), 0);
    }

    #[test]
    fn test_offset_in_plural() {
        let mut parser = setup_parser();
        let code = "items: \"{count, plural, offset:1 =0 {no items} =1 {one item} other {# items}}\"";
        let tree = parser.parse(code, None).expect("Failed to parse");

        assert!(!tree.root_node().has_error());
    }

    fn sexp(code: &str) -> String {
        let tree = setup_parser().parse(code, None).expect("Failed to parse");
        assert!(!tree.root_node().has_error(), "unexpected error in: {code}");
        tree.root_node().to_sexp()
    }

    #[test]
    fn test_plural_structure() {
        assert_eq!(
            sexp(r#"a: "{n, plural, one {x} other {y}}""#),
            "(source_file (resource (key (identifier)) (value (quoted_string (complex_message \
             (selector (identifier) (plural) (plural_rules \
             (plural_case (plural_key) (plural_case_body)) \
             (plural_case (plural_key) (plural_case_body)))))))))"
        );
    }

    #[test]
    fn test_placeholder_structure() {
        assert_eq!(
            sexp(r#"a: "Hi {name}""#),
            "(source_file (resource (key (identifier)) (value (quoted_string (placeholder (identifier))))))"
        );
    }

    #[test]
    fn test_self_closing_tag() {
        assert_eq!(
            sexp(r#"a: "x<br/>y""#),
            "(source_file (resource (key (identifier)) (value (quoted_string (tag (identifier))))))"
        );
    }

    #[test]
    fn test_export_default_wrapper() {
        let code = "export default {\n\ta: \"x\",\n\tb: `{n}`\n};\n";
        assert_eq!(
            sexp(code),
            "(source_file (module_start) \
             (resource (key (identifier)) (value (quoted_string))) \
             (resource (key (identifier)) (value (template_string (placeholder (identifier))))) \
             (module_end))"
        );
    }

    #[test]
    fn test_module_wrapper_forms() {
        for head in [
            "export",
            "export default",
            "export const messages =",
            "export let messages =",
        ] {
            let code = format!("{head} {{\n\ta: \"x\"\n}};\n");
            let mut parser = setup_parser();
            let tree = parser.parse(&code, None).expect("Failed to parse");
            assert!(!tree.root_node().has_error(), "unexpected error for: {head}");
            let sexp = tree.root_node().to_sexp();
            assert!(sexp.starts_with("(source_file (module_start"), "{head}: {sexp}");
            assert!(sexp.ends_with("(module_end))"), "{head}: {sexp}");
        }
    }

    #[test]
    fn test_unsupported_module_wrappers_report_errors() {
        let mut parser = setup_parser();
        for head in ["export foo", "export var messages =", "export const =", "export const messages"] {
            let code = format!("{head} {{\n\ta: \"x\"\n}};\n");
            let tree = parser.parse(&code, None).expect("Failed to parse");
            assert!(tree.root_node().has_error(), "expected an error for: {head}");
        }
    }

    #[test]
    fn test_json_wrapper() {
        let code = "{\n\t\"a\": \"x\",\n\t\"b\": \"{n}\"\n}\n";
        assert_eq!(
            sexp(code),
            "(source_file (module_start) \
             (resource (key (quoted_string)) (value (quoted_string))) \
             (resource (key (quoted_string)) (value (quoted_string (placeholder (identifier))))) \
             (module_end))"
        );
    }

    #[test]
    fn test_invalid_inputs_report_errors() {
        let mut parser = setup_parser();
        for code in [
            "a: \"unterminated",
            "a: \"{n, plural, one {x}\"",
            "a: \"{n, plural, one x}\"",
            "a \"missing colon\"",
            "a: \"{}\"",
            "a: unquoted value",
        ] {
            let tree = parser.parse(code, None).expect("Failed to parse");
            assert!(tree.root_node().has_error(), "expected an error for: {code}");
        }
    }

    #[test]
    fn test_queries_compile() {
        let language: Language = super::LANGUAGE.into();
        for (name, source) in [
            ("highlights", super::HIGHLIGHTS_QUERY),
            ("injections", super::INJECTIONS_QUERY),
            ("locals", super::LOCALS_QUERY),
            ("tags", super::TAGS_QUERY),
            ("folds", super::FOLDS_QUERY),
        ] {
            tree_sitter::Query::new(&language, source)
                .unwrap_or_else(|e| panic!("{name}.scm failed to compile: {e}"));
        }
    }

    #[test]
    fn test_fixture_parses_without_errors() {
        let mut parser = setup_parser();
        let code = include_str!("../../test/fixtures/messages.mf.js");
        let tree = parser.parse(code, None).expect("Failed to parse");
        assert!(!tree.root_node().has_error());
        let sexp = tree.root_node().to_sexp();
        assert!(sexp.starts_with("(source_file (module_start)"));
        assert_eq!(sexp.matches("(resource ").count(), 4);
    }

    #[test]
    fn test_json_fixture_parses_without_errors() {
        let mut parser = setup_parser();
        let code = include_str!("../../test/fixtures/messages.mf.json");
        let tree = parser.parse(code, None).expect("Failed to parse");
        assert!(!tree.root_node().has_error());
        let sexp = tree.root_node().to_sexp();
        assert!(sexp.starts_with("(source_file (module_start)"));
        assert_eq!(sexp.matches("(resource ").count(), 4);
    }
}
