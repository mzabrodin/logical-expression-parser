# Logical Expression Pest Parser

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
