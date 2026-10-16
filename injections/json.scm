; MessageFormat injections for JSON hosts (FormatJS extracts, Chrome
; messages.json, Flutter .arb, generic i18n catalogs).

; 1. Values shaped like an ICU argument
([
  (pair value: (string (string_content) @injection.content))
  (array (string (string_content) @injection.content))
 ]
 (#match? @injection.content "[{]\\s*\\w+\\s*,\\s*(plural|select|selectordinal|number|date|time)\\s*[,}]")
 (#set! injection.language "messageformat_message"))

; 2. "message" / "defaultMessage" members, the conventional message keys
(pair
  key: (string (string_content) @_key)
  value: (string (string_content) @injection.content)
  (#any-of? @_key "message" "defaultMessage")
  (#set! injection.language "messageformat_message"))
