/**
 * @file messageformat parser
 * @author Danny Gleckle <daniel.gleckler@gmail.com>
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
    source_file: $ => repeat(choice(
      $._entry,
      $.comment
    )),

    comment: () => token(seq('//', /.*/)),

    _entry: $ => seq($.key, ':', $.value, optional(',')),

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
      /[^"{}<>']+/,
      $.placeholder,
      $.complex_message,
      $.escaped_char
    )), '"'),

    template_string: $ => seq('`', repeat(choice(
      /[^`{}<>']+/,
      $.placeholder,
      $.complex_message,
      $.escaped_char
    )), '`'),

    escaped_char: $ => choice(
      $.quoted_literal,
      "''"  // Two single quotes = literal single quote
    ),

    quoted_literal: $ => seq("'", choice('{', '}', '<', '>', '#', $.placeholder, $.complex_message), "'"),

    placeholder: $ => seq('{', $.identifier, '}'),

    complex_message: $ => seq(
      '{',
      $.identifier,
      ',',
      $.message_type,
      optional(seq(',', $.format_spec)),
      '}'
    ),

    message_type: () => choice(
      'number',
      'date',
      'time',
      'plural',
      'selectordinal',
      'select'
    ),

    format_spec: $ => choice(
      $.skeleton_format,
      $.style_format,
      $.plural_rules,
      $.select_rules
    ),

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
      $.case_body
    ),

    select_case: $ => seq(
      $.select_key,
      $.case_body
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

    select_key: $ => $.identifier,

    case_body: $ => seq('{', repeat(choice(
      /[^{}#<>'"`]+/,
      $.quoted_string,
      $.template_string,
      $.placeholder,
      $.complex_message,
      $.pound_placeholder,
      $.escaped_char
    )), '}'),

    pound_placeholder: () => '#',

    number: () => /\d+/
  }
});
