# Logical Expression Pest Parser

[![GitHub repo](https://img.shields.io/badge/github-logical--expression--parser-8da0cb?logo=github)](https://github.com/mzabrodin/logical-expression-parser)
[![Crates.io](https://img.shields.io/crates/v/logical-expression-pest-parser.svg)](https://crates.io/crates/logical-expression-pest-parser)

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

After parsing, the resulting three is analyzed and converted into an Abstract Syntax Tree.
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

cargo install logical-expression-parser
logical-expression-pest-parser.exe parse -f .\input.txt --ast
```
---
#### input.txt

In order to analyze multiple expressions, they must be defined in different lines and there must be an empty line at the end.

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
| 0 | 0 | 0 |   0    |
| 1 | 0 | 0 |   1    |
| 0 | 1 | 0 |   0    |
| 1 | 1 | 0 |   0    |
| 0 | 0 | 1 |   1    |
| 1 | 0 | 1 |   1    |
| 0 | 1 | 1 |   1    |
| 1 | 1 | 1 |   1    |

Expression 2
Input: "X !| Y"

AST: Nor(Identifier('X'), Identifier('Y'))

| X | Y | Output |
|---|---|--------|
| 0 | 0 |   1    |
| 1 | 0 |   0    |
| 0 | 1 |   0    |
| 1 | 1 |   0    |

Expression 3
Input: "(K XOR L) AND M"

AST: And(Xor(Identifier('K'), Identifier('L')), Identifier('M'))

| K | L | M | Output |
|---|---|---|--------|
| 0 | 0 | 0 |   0    |
| 1 | 0 | 0 |   0    |
| 0 | 1 | 0 |   0    |
| 1 | 1 | 0 |   0    |
| 0 | 0 | 1 |   0    |
| 1 | 0 | 1 |   1    |
| 0 | 1 | 1 |   1    |
| 1 | 1 | 1 |   0    |
```
---
#### Use this command for help
```shell

logical-expression-pest-parser.exe help
```

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

identifier = @{ ASCII_ALPHA_UPPER ~ !(ASCII_ALPHA_UPPER) }

term       = { not_operator* ~ (identifier | left_parenthesis ~ expression ~ right_parenthesis) }
xor_clause = { term ~ ((xor_operator | xnor_operator) ~ term)* }
and_clause = { xor_clause ~ ((and_operator | nand_operator) ~ xor_clause)* }
expression = { and_clause ~ ((or_operator | nor_operator) ~ and_clause)* }

NEWLINE = _{ "\n" | "\r\n" }

file = { SOI ~ expression ~ NEWLINE ~ (expression ~ NEWLINE)* ~ EOI }
```