# Noz Language
StandardML with refinement types + Affirmation -based language

# Summary
- [How to contribute](#contribute)
    - [Requirements](#requirements)
    - [Git Workflow](#git-workflow)
- [Technical specificities](#technical-specificities)
    - [Examples](#examples)

# Contribute

## Requirements
- `rustc >= 1.75.0`
- `cargo >= 1.75.0`

## Git workflow
To submit your code, create a PR on the `dev` branch, which will be reviewed later on and merged on main if the feature is stable and legitimate.

Please document all your code, especially the structs fields and the functions by providing their specification. You can get inspirations from the actual code.

# Technical specificities
> [!CAUTION]
> This section, especially the grammar, is subject to changes.
Here is the BNF of the language's grammar :
```ebnf
program = expr;

expr = let | app | if-expr | fun;

app = app term | term;

let = 'let' definition 'in' expr;

definition = id [ ':' id ] '=' ( type_def | expr );

type_def = 'type' type_expr;

type_expr = ( type_id | product_type | sum_type | function_type );

type_id = id;

product_type = '{' { id ':' id ';' } '}';

sum_type = '|' type_expr { '|' type_expr };

function_type = type_expr '->' type_expr;

fun = [ 'ifx' ] 'fun' { id } '->' expr;

if-expr = 'if' expr 'then' expr 'else' expr;

term = id | literal | '(' expr ')';

literal = numbers | strings | booleans | chars;
``` 

## Examples

### Usage for question exprs

```ocaml
(* Here it's annoying cause `x: Str` should be an affirmation but it has little sense to define something as `let x: Str in` if x is not defined. Or it could be empty instance. *)
let show x =
    if (? x: Str) then print(x)
    else if (? x: Int) then printf("{i}", x)
    else if (? x: Bool) then printf("{b}", x);
```
