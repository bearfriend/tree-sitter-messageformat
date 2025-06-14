; Inject JavaScript into template strings that might contain expressions
((template_string) @injection.content
 (#set! injection.language "javascript"))

; Inject regex patterns in skeleton formats for advanced formatting
((skeleton_pattern) @injection.content
 (#match? @injection.content "^[yMdHms/:.,#]+$")
 (#set! injection.language "regex"))

; Inject documentation syntax in comments
((comment) @injection.content
 (#match? @injection.content "^//\\s*@")
 (#set! injection.language "jsdoc"))