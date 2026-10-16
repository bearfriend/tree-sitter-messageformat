/**
 * @file messageformat parser
 * @author Danny Gleckler <daniel.gleckler@gmail.com>
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "messageformat",

  extras: $ => [
    /\s/,
    $.comment
  ],

  rules: {
    source_file: $ => seq(
      optional($.module_start),
      repeat($.resource),
      optional($.module_end)
    ),

    // Optional `export default { ... };` wrapper for .msg.js style files
    module_start: () => seq('export', 'default', '{'),

    module_end: () => seq('}', optional(';')),

    comment: () => token(seq('//', /.*/)),

    resource: $ => seq($.key, ':', $.value, optional(',')),

    key: $ => choice(
      $.identifier,
      $.quoted_string
    ),

    value: $ => choice(
      $.quoted_string,
      $.template_string
    ),

    identifier: () => /[a-zA-Z_][a-zA-Z0-9_]*/,

    quoted_string: $ => seq('"', repeat(choice(
      $.escaped_char,
      prec(-1, "'"),
      /(\\"|[^"{<'])+/,
      $.placeholder,
      $.complex_message,
      $.tag
    )), optional("'"), '"'),

    template_string: $ => seq('`', repeat(choice(
      $.escaped_char,
      prec(-1, "'"),
      /(\\`|[^`{<'])+/,
      $.placeholder,
      $.complex_message,
      $.tag
    )), optional("'"), '`'),

    escaped_char: $ => choice(
      token(seq("'", choice('{', '}', '<', '>', '#'), /[^']*/, "'")),
      "''"  // Two single quotes = literal single quote
    ),

    placeholder: $ => seq('{', $.identifier, '}'),

    tag: $ => seq('<', choice(
      seq('/', $.identifier),
      seq($.identifier, optional('/'))
    ), '>'),

    complex_message: $ => choice(
      $.selector,
      seq('{', $.identifier, ',', choice('number', 'date', 'time'), optional(seq(',', choice($.skeleton_format, $.style_format))), '}')
    ),

    selector: $ => choice(
      seq('{', $.identifier, ',', $.plural, optional(seq(',', $.plural_rules)), '}'),
      seq('{', $.identifier, ',', $.selectordinal, optional(seq(',', $.plural_rules)), '}'),
      seq('{', $.identifier, ',', $.select, optional(seq(',', $.select_rules)), '}'),
    ),

    plural: () => 'plural',

    select: () => 'select',

    selectordinal: () => 'selectordinal',

    skeleton_format: $ => seq('::', $.skeleton_pattern),

    skeleton_pattern: () => /[^}]+/,

    style_format: () => choice(
      'short',
      'medium',
      'long',
      'full',
      'integer',
      'currency',
      'percent'
    ),

    plural_rules: $ => seq(
      optional($.offset),
      repeat1($.plural_case)
    ),

    select_rules: $ => repeat1($.select_case),

    offset: $ => seq('offset:', $.number),

    plural_case: $ => seq(
      $.plural_key,
      $.plural_case_body
    ),

    select_case: $ => seq(
      $.select_key,
      $.select_case_body
    ),

    plural_key: $ => choice(
      seq('=', $.number),
      'zero',
      'one',
      'two',
      'few',
      'many',
      'other'
    ),

    select_key: $ => choice($.identifier, "other"), // todo: anythinng but {}'#<>

    plural_case_body: $ => seq('{', repeat(choice(
      /([^{}#<>"']|'[^{}<>'#])+/,
      $.quoted_string,
      $.template_string,
      $.placeholder,
      $.complex_message,
      $.tag,
      $.pound_placeholder,
      $.escaped_char
    )), '}'),

    select_case_body: $ => seq('{', repeat(choice(
      // `#` is plain text in a select, but `'#'` must still reach escaped_char
      // rather than being swallowed by this regex.
      /([^{}<>"']|'[^{}<>'#])+/,
      prec(-1, "'"),
      $.quoted_string,
      $.template_string,
      $.placeholder,
      $.complex_message,
      $.tag,
      $.escaped_char
    )), '}'),

    pound_placeholder: () => '#',

    number: () => /\d+/
  }
});
