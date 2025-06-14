; Comments
(comment) @comment

; Keys in resources
(key (identifier) @property)
(key (quoted_string) @property)

; String literals
(quoted_string) @string
(template_string) @string

; Identifiers (variables)
(identifier) @variable

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

; Select keys
(select_key (identifier) @constant)

; Numbers
(number) @number

; Offset keyword and value
"offset:" @keyword
(offset (number) @number)

; Style formats
(style_format) @constant.builtin

; Skeleton format
(skeleton_format "::" @operator)
(skeleton_pattern) @string.special

; Pound placeholder
(pound_placeholder) @variable.builtin

; Tags
(tag (identifier) @tag)

; Escaped characters
(escaped_char) @string.escape
(quoted_literal) @string.escape

; Operators and punctuation
"=" @operator
":" @punctuation.delimiter
"," @punctuation.delimiter
"{" @punctuation.bracket
"}" @punctuation.bracket
"<" @punctuation.bracket
">" @punctuation.bracket
"/" @operator
"::" @operator

; String delimiters
"\"" @punctuation.delimiter
"`" @punctuation.delimiter
"'" @punctuation.delimiter