; Resource key definitions
(resource
  (key (identifier) @name) @definition.var
  (#set! "definition.var.scope" "parent"))

(resource
  (key (quoted_string) @name) @definition.var
  (#set! "definition.var.scope" "parent"))

; Placeholder variable references
(placeholder
  (identifier) @name @reference.var)

; Complex message variable references
(complex_message
  (identifier) @name @reference.var)

; Selector variable references
(selector
  (identifier) @name @reference.var)

; Select case keys as local definitions
(select_case
  (select_key (identifier) @name) @definition.var
  (#set! "definition.var.scope" "local"))

; Tag definitions
(tag
  (identifier) @name @definition.function)