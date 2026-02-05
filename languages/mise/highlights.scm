; Mise syntax highlighting - Optimized for performance
; Uses #any-of? predicates (faster than regex) where possible

; Section headers
((bare_key) @keyword
  (#any-of? @keyword "tools" "env" "tasks" "vars" "plugins" "settings" "alias" "hooks" "redactions"))

; Task names [tasks.xxx]
(table (dotted_key (bare_key) @keyword (#eq? @keyword "tasks") (bare_key) @function))

; Task properties
(pair (bare_key) @function (#eq? @function "run"))
(pair (bare_key) @keyword (#any-of? @keyword "depends" "wait_for" "description" "file" "sources" "outputs" "dir" "env" "hide" "shell" "raw" "quiet"))

; Hook events
(pair (bare_key) @keyword (#any-of? @keyword "enter" "leave" "cd" "watch"))

; Tool definitions
(pair (bare_key) @type (string) @string.special)

; Special versions
((string) @constant.builtin (#any-of? @constant.builtin "\"latest\"" "\"lts\"" "\"system\""))

; Environment variables (UPPERCASE)
(pair (bare_key) @variable (#match? @variable "^[A-Z_][A-Z0-9_]*$"))

; Tera templates
((string) @string.special (#match? @string.special "\\{\\{"))

; Primitives
(boolean) @constant.builtin
(integer) @number
(float) @number
(comment) @comment

; Punctuation
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"=" @operator
"." @punctuation.delimiter
"," @punctuation.delimiter
