; MessageFormat injections for Swift (iOS / macOS).
;
; Swift has no built-in ICU MessageFormat API, so the only reliable signal is
; the shape of the string itself.

([
  (line_string_literal text: (line_str_text) @injection.content)
  (multi_line_string_literal text: (multi_line_str_text) @injection.content)
 ]
 (#match? @injection.content "[{]\\s*\\w+\\s*,\\s*(plural|select|selectordinal|number|date|time)\\s*[,}]")
 (#set! injection.language "messageformat_message"))
