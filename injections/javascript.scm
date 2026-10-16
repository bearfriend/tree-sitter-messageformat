; MessageFormat injections for JavaScript, TypeScript and TSX hosts.
;
; Add these patterns to the host language's injections query (for example with
; Helix's `; inherits` or by appending them). Each string fragment is injected
; with the bare-message grammar (`messageformat_message`).
;
; The regex is deliberately limited to Rust-regex syntax (Zed, Helix, the
; tree-sitter CLI).

; 1. Anything shaped like an ICU argument: {name, plural|select|number|...}
([
  (string (string_fragment) @injection.content)
  (template_string (string_fragment) @injection.content)
 ]
 (#match? @injection.content "[{]\\s*\\w+\\s*,\\s*(plural|select|selectordinal|number|date|time)\\s*[,}]")
 (#set! injection.language "messageformat_message"))

; 2. new IntlMessageFormat("...") / new MessageFormat("...")
(new_expression
  constructor: (identifier) @_ctor
  arguments: (arguments
    . [
      (string (string_fragment) @injection.content)
      (template_string (string_fragment) @injection.content)
    ])
  (#any-of? @_ctor "IntlMessageFormat" "MessageFormat" "MessageFormatter")
  (#set! injection.language "messageformat_message"))

; 3. intl.formatMessage("...") / formatMessage("...") / defineMessage("...")
(call_expression
  function: [
    (identifier) @_fn
    (member_expression property: (property_identifier) @_fn)
  ]
  arguments: (arguments
    . [
      (string (string_fragment) @injection.content)
      (template_string (string_fragment) @injection.content)
    ])
  (#any-of? @_fn "formatMessage" "defineMessage")
  (#set! injection.language "messageformat_message"))

; 4. { defaultMessage: "..." } (FormatJS / react-intl descriptors)
(pair
  key: [
    (property_identifier) @_key
    (string (string_fragment) @_key)
  ]
  value: [
    (string (string_fragment) @injection.content)
    (template_string (string_fragment) @injection.content)
  ]
  (#eq? @_key "defaultMessage")
  (#set! injection.language "messageformat_message"))

; 5. Explicit hint: /* messageformat */ "..." or /* icu */ "..."
((comment) @_hint
 .
 [
   (string (string_fragment) @injection.content)
   (template_string (string_fragment) @injection.content)
 ]
 (#match? @_hint "^/[*]\\s*(messageformat|icu)\\s*[*]/$")
 (#set! injection.language "messageformat_message"))
