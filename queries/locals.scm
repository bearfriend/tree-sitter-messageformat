; Define scopes
(source_file) @local.scope
(case_body) @local.scope
(resource) @local.scope

; Variable definitions - identifiers in placeholders and complex messages
(placeholder (identifier) @local.reference)
(complex_message (identifier) @local.reference)

; Selector variables are references to defined variables
(selector (identifier) @local.reference)

; Key definitions
(key (identifier) @local.definition)

; Select keys can be considered local definitions within their scope
(select_key (identifier) @local.definition)