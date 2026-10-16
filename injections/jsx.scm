; MessageFormat injections for JSX attributes. Use with the JavaScript and TSX
; hosts only: the plain TypeScript grammar has no JSX nodes, so this file would
; not compile there. Combine with javascript.scm.

; <FormattedMessage defaultMessage="..." />
(jsx_attribute
  (property_identifier) @_attr
  (string (string_fragment) @injection.content)
  (#eq? @_attr "defaultMessage")
  (#set! injection.language "messageformat_message"))

; Any attribute whose value is shaped like an ICU argument
(jsx_attribute
  (property_identifier)
  (string (string_fragment) @injection.content)
  (#match? @injection.content "[{]\\s*\\w+\\s*,\\s*(plural|select|selectordinal|number|date|time)\\s*[,}]")
  (#set! injection.language "messageformat_message"))
