# Logical Expression Pest Parser Documentation

This crate parses strings of logical expression using `pest` crate, such as:
> (A AND B) OR (NOT (A XOR C))

> (A | C) & !B

> (X XOR Y) | A

It returns a truth table showing all input combinations with corresponding output results.

---

It can handle those logic operators:

| Operator | Corresponding symbols |
|----------|-----------------------| 
| NOT      | NOT, not, !           |
| AND      | AND, and, &           |            
| NAND     | NAND, nand, !&        |
| OR       | OR, or, \|            |
| NOR      | NOR, nor, !\|         |
| XOR      | XOR, xor, ^           |
| XNOR     | XNOR, xnor, !^        |

After parsing, the resulting tree is analyzed and converted into an Abstract Syntax Tree.
The next step is to evaluate the logical expression for all possible combinations of input values, allowing the
generation of a complete truth table.

For example, we have an expression:

> (A & B) | C

The resulting truth table is going to be

| A | B | C | Output |
|---|---|---|--------|
| 0 | 0 | 0 | 0      |
| 1 | 0 | 0 | 0      |
| 0 | 1 | 0 | 0      |
| 1 | 1 | 0 | 1      |
| 0 | 0 | 1 | 1      |
| 1 | 0 | 1 | 1      |
| 0 | 1 | 1 | 1      |
| 1 | 1 | 1 | 1      |

## Usage


```shell

cargo install logical-expression-pest-parser
logical-expression-pest-parser.exe parse -f .\input.txt --ast
```
---
#### input.txt

Multiple expressions must be defined on different lines. A newline must follow each expression; if the file's last line is missing, the CLI adds it automatically.

```text
A and !B or C
X !| Y
(K XOR L) AND M
```
---
#### Output
```text
Processing file: .\input.txt

Expression 1
Input: "A and !B or C"

AST: Or(And(Identifier('A'), Not(Identifier('B'))), Identifier('C'))

| A | B | C | Output |
|---|---|---|--------|
| 0 | 0 | 0 | 0      |
| 1 | 0 | 0 | 1      |
| 0 | 1 | 0 | 0      |
| 1 | 1 | 0 | 0      |
| 0 | 0 | 1 | 1      |
| 1 | 0 | 1 | 1      |
| 0 | 1 | 1 | 1      |
| 1 | 1 | 1 | 1      |

Expression 2
Input: "X !| Y"

AST: Nor(Identifier('X'), Identifier('Y'))

| X | Y | Output |
|---|---|--------|
| 0 | 0 | 1      |
| 1 | 0 | 0      |
| 0 | 1 | 0      |
| 1 | 1 | 0      |

Expression 3
Input: "(K XOR L) AND M"

AST: And(Xor(Identifier('K'), Identifier('L')), Identifier('M'))

| K | L | M | Output |
|---|---|---|--------|
| 0 | 0 | 0 | 0      |
| 1 | 0 | 0 | 0      |
| 0 | 1 | 0 | 0      |
| 1 | 1 | 0 | 0      |
| 0 | 0 | 1 | 0      |
| 1 | 0 | 1 | 1      |
| 0 | 1 | 1 | 1      |
| 1 | 1 | 1 | 0      |
```
---
#### Use this command for help
```shell

logical-expression-pest-parser.exe help
```

## Library usage

The parsing/evaluation logic is also available as a library:

```rust
# fn run() -> Result<(), logical_expression_pest_parser::parser::ParserError> {
use logical_expression_pest_parser::parse_expression;
use std::collections::HashMap;

let expr = parse_expression("A AND !B\n")?;
assert!(!expr.evaluate(&HashMap::new())); // A and B both default to false
# Ok(())
# }
# run().unwrap();
```

For multi-line input, `parse_expressions` returns every `(source_line, Expression)` pair instead of just the first:

```rust
# fn run() -> Result<(), logical_expression_pest_parser::parser::ParserError> {
use logical_expression_pest_parser::parse_expressions;

let expressions = parse_expressions("A AND B\nA OR B\n")?;
assert_eq!(expressions.len(), 2);
# Ok(())
# }
# run().unwrap();
```

`parse_expression` returns the [`Expression`](crate::ast::Expression) AST for the first expression in the input, which can then be evaluated directly (`expr.evaluate(&variables)`) or turned into a [`TruthTable`](crate::truth_table::TruthTable) via `TruthTable::from(&expr)`.

## grammar.pest

The grammar is structured in such a way that the parser can determine priorities for boolean operators.

```text
WHITESPACE = _{ " " | "\t" }

not_operator  = { "NOT" | "not" | "!" }
and_operator  = { "AND" | "and" | "&" }
nand_operator = { "NAND" | "nand" | "!&" }
or_operator   = { "OR" | "or" | "|" }
nor_operator  = { "NOR" | "nor" | "!|" }
xor_operator  = { "XOR" | "xor" | "^" }
xnor_operator = { "XNOR" | "xnor" | "!^" }

left_parenthesis  = { "(" }
right_parenthesis = { ")" }

// A single uppercase ASCII letter, so at most 26 distinct variables per expression.
identifier = @{ ASCII_ALPHA_UPPER ~ !(ASCII_ALPHA_UPPER) }

term       = { not_operator* ~ (identifier | left_parenthesis ~ expression ~ right_parenthesis) }
xor_clause = { term ~ ((xor_operator | xnor_operator) ~ term)* }
and_clause = { xor_clause ~ ((and_operator | nand_operator) ~ xor_clause)* }
expression = { and_clause ~ ((or_operator | nor_operator) ~ and_clause)* }

NEWLINE = _{ "\n" | "\r\n" }

file = { SOI ~ expression ~ NEWLINE ~ (expression ~ NEWLINE)* ~ EOI }
```