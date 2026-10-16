; MessageFormat injections for Java (java.text.MessageFormat, ICU4J and
; android.icu.text.MessageFormat all take strings in this syntax).

; 1. Anything shaped like an ICU argument
(string_literal
  [(string_fragment) (multiline_string_fragment)] @injection.content
  (#match? @injection.content "[{]\\s*\\w+\\s*,\\s*(plural|select|selectordinal|number|date|time)\\s*[,}]")
  (#set! injection.language "messageformat_message"))

; 2. new MessageFormat("...")
(object_creation_expression
  type: (type_identifier) @_type
  arguments: (argument_list
    . (string_literal
        [(string_fragment) (multiline_string_fragment)] @injection.content))
  (#any-of? @_type "MessageFormat" "IntlMessageFormat" "MessageFormatter")
  (#set! injection.language "messageformat_message"))

; 3. MessageFormat.format("...", args)
(method_invocation
  object: (identifier) @_object
  name: (identifier) @_name
  arguments: (argument_list
    . (string_literal
        [(string_fragment) (multiline_string_fragment)] @injection.content))
  (#eq? @_object "MessageFormat")
  (#any-of? @_name "format" "applyPattern")
  (#set! injection.language "messageformat_message"))
