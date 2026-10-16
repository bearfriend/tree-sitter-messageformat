/**
 * @file messageformat message parser
 *
 * A second grammar that shares every rule of the main `messageformat` grammar
 * but is rooted at a single bare message instead of a `key: value` resource
 * file. It is meant for injections: the contents of a string literal in
 * another language (JavaScript, JSON, Java, ...) is a bare message.
 *
 * @author Danny Gleckler <daniel.gleckler@gmail.com>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

const base = require("../grammar.js");

module.exports = grammar(base, {
  name: "messageformat_message",

  // Messages have no `//` comments: `http://example.com` is just text.
  extras: () => [/\s/],

  rules: {
    // ICU allows numeric argument names (`{0}`), which is how Java's
    // MessageFormat and most mobile code refer to arguments.
    identifier: () => /[a-zA-Z0-9_][a-zA-Z0-9_]*/,

    source_file: $ => repeat(choice(
      $.escaped_char,
      prec(-1, "'"),
      alias(/[^{<'][^{<']*/, $.text),
      $.placeholder,
      $.complex_message,
      $.tag
    )),
  }
});
