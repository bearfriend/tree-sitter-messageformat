; Comments
(comment) @comment

; Keys in resources
(key (identifier) @property)
(key (quoted_string) @property)

; String literals (keys are captured separately as @property)
(value (quoted_string) @string)
(value (template_string) @string)
(plural_case_body (quoted_string) @string)
(plural_case_body (template_string) @string)
(select_case_body (quoted_string) @string)
(select_case_body (template_string) @string)

; Identifiers (variables) -- scoped by parent so they never overlap
; with keys (@property), select keys (@constant) or tags (@tag)
(complex_message (identifier) @variable)
(selector (identifier) @variable)

; Placeholders
(placeholder (identifier) @variable.parameter)

; Complex message format keywords
(plural) @keyword
(select) @keyword
(selectordinal) @keyword

; Format types
["number" "date" "time"] @type

; Plural case keywords
(plural_key ["zero" "one" "two" "few" "many" "other"]) @constant.builtin

; Select keys (`other` is the required fallback, like in plural)
(select_key (identifier) @constant)
(select_key "other" @constant.builtin)

; Numbers
(number) @number

; Offset keyword and value
"offset:" @keyword

; Style formats
(style_format) @constant.builtin

; Skeleton format
(skeleton_format "::" @operator)
(skeleton_pattern) @string.special

; Module wrapper (`export default { ... };`)
["export" "default"] @keyword
(module_start "{" @punctuation.bracket)
(module_end "}" @punctuation.bracket)
(module_end ";" @punctuation.delimiter)

; Pound placeholder
(pound_placeholder) @variable.builtin

; Tags
(tag (identifier) @tag)

; Escaped characters (`'{'`, `''`, ...)
(escaped_char) @string.escape

; Operators and punctuation
"=" @operator
":" @punctuation.delimiter
"," @punctuation.delimiter
"{" @punctuation.bracket
"}" @punctuation.bracket
"<" @punctuation.bracket
">" @punctuation.bracket
"/" @operator

; String delimiters (a bare `'` is plain text or part of an escape, never a delimiter)
"\"" @punctuation.delimiter
"`" @punctuation.delimiter
