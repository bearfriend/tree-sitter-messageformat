; MessageFormat injections for Kotlin (Android and Kotlin Multiplatform).

; 1. Anything shaped like an ICU argument
([
  (string_literal (string_content) @injection.content)
  (multiline_string_literal (string_content) @injection.content)
 ]
 (#match? @injection.content "[{]\\s*\\w+\\s*,\\s*(plural|select|selectordinal|number|date|time)\\s*[,}]")
 (#set! injection.language "messageformat_message"))

; 2. MessageFormat("...")
(call_expression
  (identifier) @_fn
  (value_arguments . (value_argument
    [
      (string_literal (string_content) @injection.content)
      (multiline_string_literal (string_content) @injection.content)
    ]))
  (#any-of? @_fn "MessageFormat" "IntlMessageFormat" "MessageFormatter")
  (#set! injection.language "messageformat_message"))

; 3. MessageFormat.format("...", args)
(call_expression
  (navigation_expression (identifier) @_object (identifier) @_name)
  (value_arguments . (value_argument
    [
      (string_literal (string_content) @injection.content)
      (multiline_string_literal (string_content) @injection.content)
    ]))
  (#eq? @_object "MessageFormat")
  (#any-of? @_name "format" "applyPattern")
  (#set! injection.language "messageformat_message"))
