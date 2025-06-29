; Identifier naming conventions; these "soft conventions" should stay at the top of the file as they're often overridden
(identifier) @variable
(attribute attribute: (identifier) @property)

; ALL_CAPS for constants:
((identifier) @constant
  (#match? @constant "^_*[A-Z][A-Z0-9_]*$"))

(type (identifier) @type)
(generic_type (identifier) @type)
(comment) @comment
(string) @string
(escape_sequence) @string.escape

; TypeVar with constraints in type parameters
(type
  (tuple (identifier) @type)
)

; Forward references
(type
  (string) @type
)


; Function calls

(call
  function: (attribute attribute: (identifier) @function.method.call))
(call
  function: (identifier) @function.call)

(decorator "@" @punctuation.special)
(decorator
  "@" @punctuation.special
  [
    (identifier) @function.decorator
    (attribute attribute: (identifier) @function.decorator)
    (call function: (identifier) @function.decorator.call)
    (call (attribute attribute: (identifier) @function.decorator.call))
  ])

; Function definitions

(function_definition
  name: (identifier) @function.definition)

((call
  function: (identifier) @_isinstance
  arguments: (argument_list
    (_)
    (identifier) @type))
  (#eq? @_isinstance "isinstance"))

((call
  function: (identifier) @_issubclass
  arguments: (argument_list
    (identifier) @type
    (identifier) @type))
  (#eq? @_issubclass "issubclass"))

; Function arguments
(function_definition
  parameters: (parameters
  [
      (identifier) @variable.parameter; Simple parameters
      (typed_parameter
        (identifier) @variable.parameter) ; Typed parameters
      (default_parameter
        name: (identifier) @variable.parameter) ; Default parameters
      (typed_default_parameter
        name: (identifier) @variable.parameter) ; Typed default parameters
  ]))

; Keyword arguments
(call
  arguments: (argument_list
    (keyword_argument
      name: (identifier) @function.kwargs)))

; Builtins

((call
  function: (identifier) @function.builtin)
 (#any-of?
   @function.builtin
   "abs" "all" "any" "bin" "bool" "breakpoint" "bytes" "dict" "dir" "enumerate" "float" "format" "getattr" "hasattr" "hash" "help" "int" "len" "list" "map" "max" "min" "print" "range" "repr" "reversed" "set" "sorted" "str" "tuple" "type" "zip"))

; Literals

[
  (true)
  (false)
] @boolean

[
  (none)
  (ellipsis)
] @constant.builtin

[
  (integer)
  (float)
] @number


[
  "."
  ","
  ":"
] @punctuation.delimiter

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

(interpolation
  "{" @punctuation.special
  "}" @punctuation.special) @embedded

; Docstrings.
([
  (expression_statement (assignment))
]
. (expression_statement (string) @string.doc)+)

(module
  .(expression_statement (string) @string.doc)+)

(function_definition
  "async"?
  "def"
  name: (_)
  (parameters)?
  body: (block .(expression_statement (string) @string.doc)+))

(module
  . (comment) @comment*
  . (expression_statement (string) @string.doc)+)

[
  "-"
  "-="
  "!="
  "*"
  "**"
  "**="
  "*="
  "/"
  "//"
  "//="
  "/="
  "&"
  "%"
  "%="
  "@"
  "^"
  "+"
  "->"
  "+="
  "<"
  "<<"
  "<="
  "<>"
  "="
  ":="
  "=="
  ">"
  ">="
  ">>"
  "|"
  "~"
] @operator

[
  "and"
  "in"
  "not"
  "or"
  "del"
] @keyword.operator

[
  "as"
  "async"
  "await"
  "break"
  "class"
  "continue"
  "def"
  "elif"
  "else"
  "except"
  "exec"
  "finally"
  "for"
  "from"
  "global"
  "if"
  "import"
  "lambda"
  "nonlocal"
  "pass"
  "print"
  "raise"
  "return"
  "try"
  "while"
  "with"
  "yield"
] @keyword

; Definition keywords def, class, async def, lambda
[
  "async"
  "def"
  "class"
  "lambda"
] @keyword.definition

(decorator (identifier) @attribute.builtin
  (#any-of? @attribute.builtin "classmethod" "staticmethod" "property"))

; Builtin types as identifiers
[
  (call
    function: (identifier) @type.builtin)
  (type (identifier) @type.builtin)
  (generic_type (identifier) @type.builtin)
  ; also check if type binary operator left identifier for union types
  (type
    (binary_operator
      left: (identifier) @type.builtin))
  (#any-of? @type.builtin "bool" "bytes" "complex" "dict" "float" "int" "list" "range" "set" "str" "tuple")
]
