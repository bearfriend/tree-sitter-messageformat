; MessageFormat injections for HTML hosts. Angular, Vue i18n and similar
; template systems put ICU expressions directly in text and attributes.

([
  (text) @injection.content
  (attribute_value) @injection.content
 ]
 (#match? @injection.content "[{]\\s*\\w+\\s*,\\s*(plural|select|selectordinal|number|date|time)\\s*[,}]")
 (#set! injection.language "messageformat_message"))
