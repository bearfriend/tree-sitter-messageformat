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
}

/// The tree-sitter [`LanguageFn`] for this grammar.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_messageformat) };

/// The content of the [`node-types.json`] file for this grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers/6-static-node-types
pub const NODE_TYPES: &str = include_str!("../../src/node-types.json");

pub const HIGHLIGHTS_QUERY: &str = include_str!("../../queries/highlights.scm");
pub const INJECTIONS_QUERY: &str = include_str!("../../queries/injections.scm");
pub const LOCALS_QUERY: &str = include_str!("../../queries/locals.scm");
pub const TAGS_QUERY: &str = include_str!("../../queries/tags.scm");
pub const FOLDS_QUERY: &str = include_str!("../../queries/folds.scm");

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
        let code = include_str!("../../test/fixtures/messages.msg");
        let tree = parser.parse(code, None).expect("Failed to parse");
        assert!(!tree.root_node().has_error());
        let sexp = tree.root_node().to_sexp();
        assert!(sexp.starts_with("(source_file (module_start)"));
        assert_eq!(sexp.matches("(resource ").count(), 4);
    }
}
