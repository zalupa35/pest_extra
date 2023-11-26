# pest_extra

**pest_extra** provides additional functionality for the
[Pest](https://github.com/pest-parser/pest) parser

## Features

### Grammar

See below to find how to compile this grammar into Pest grammar.

<details>

<summary>Variables</summary>

A variable is a string that you can use in rules:

```
variable = "value" // easier than variable = { "value" }
```

</details>

<details>

<summary>Including other grammar in grammar</summary>

Idea: https://github.com/pest-parser/pest/pull/759

Includes all the rules from one grammar to another.

<details>

<summary>Example</summary>

<details>

<summary>calculator.pest</summary>

```
include!("grammar2.pest")
integer = @{ ASCII_DIGIT+ }
primary = _{ integer | "(" ~ expr ~ ")" }
atom = _{ unary_minus? ~ primary }
expr = { atom ~ (bin_op ~ atom)* }
equation = _{ SOI ~ expr ~ EOI }
```

</details>

<details>

<summary>grammar2.pest</summary>

```
WHITESPACE = _{ " " }
unary_minus = { "-" }
bin_op = _{ add | subtract | multiply | divide | modulo }
	add = { "+" }
	subtract = { "-" }
	multiply = { "*" }
	divide = { "/" }
	modulo = { "%" }
```

</details>

<details>

<summary>compiled.pest</summary>

```
unary_minus =  { "-" }
divide      =  { "/" }
modulo      =  { "%" }
WHITESPACE  = _{ " " }
equation = _{ SOI ~ expr ~ EOI }
bin_op      = _{ add | subtract | multiply | divide | modulo }
atom     = _{ unary_minus? ~ primary }
integer  = @{ ASCII_DIGIT+ }
primary  = _{ integer | "(" ~ expr ~ ")" }
add         =  { "+" }
expr     =  { atom ~ (bin_op ~ atom)* }
multiply    =  { "*" }
subtract    =  { "-" }
```

</details>

</details>

</details>

### Other

#### Installation (for formatting and minifying):

```
[dependencies]
pest_extra = { version = "0.1.0", features = [ "formatter" ] }
```

<details>

<summary>Formatting</summary>

Uses [pest-fmt](https://github.com/pest-parser/pest-fmt).

Usage:

```rust
use pest_extra::formatter;

fn main() {
    println!("{}", formatter::format(include_str!("./grammar.pest").to_string()));
}
```

</details>

<details>

<summary>Minifying</summary>

Minifies Pest grammar.

Usage:

```rust
use pest_extra::formatter;

fn main() {
    println!("{}", formatter::minify(include_str!("./grammar.pest").to_string()));
}
```

Comparsion:

<details>

<summary>calculator.pest</summary>

Size: **556 bytes**

```
// No whitespace allowed between digits
integer = @{ ASCII_DIGIT+ }

unary_minus = { "-" }
primary = _{ integer | "(" ~ expr ~ ")" }
atom = _{ unary_minus? ~ primary }

bin_op = _{ add | subtract | multiply | divide | modulo }
	add = { "+" }
	subtract = { "-" }
	multiply = { "*" }
	divide = { "/" }
	modulo = { "%" }

expr = { atom ~ (bin_op ~ atom)* }

// We can't have SOI and EOI on expr directly, because it is used recursively (e.g. with parentheses)
equation = _{ SOI ~ expr ~ EOI }

WHITESPACE = _{ " " }
```

</details>

<details>

Size: **272 bytes** (saved 48.92%)

<summary>minified-calculator.pest</summary>

```
integer=@{ASCII_DIGIT+}unary_minus={"-"}primary=_{integer|"("~expr~")"}atom=_{unary_minus?~primary}bin_op=_{add|subtract|multiply|divide|modulo}add={"+"}subtract={"-"}multiply={"*"}divide={"/"}modulo={"%"}expr={atom~(bin_op~atom)*}equation=_{SOI~expr~EOI}WHITESPACE=_{" "}
```

</details>

</details>

## Compiling grammar into the Pest grammar

### Installation

```
[dependencies]
pest_extra = "0.1.0"
```

### Usage

```rust
use pest_extra::{
    formatter,
    parser::{compile_grammar, types::Grammar},
};

fn main() {
	// Compiling grammar
    if let Ok(grammar) = compile_grammar(Grammar::from_file("./grammar.pest").unwrap()) {
		// Formatting grammar
        println!("{}", formatter::format(grammar).unwrap());
    }
}
```

## Pest VM

**pest_extra** has built-in [pest_vm](https://crates.io/crates/pest_vm) to
create a parser from your grammar.

### Installation

```
[dependencies]
pest_extra = { version = "0.1.0", features = [ "vm" ] }
```

### Usage

```rust
use pest_extra::{
    parser::{compile_grammar, types::Grammar},
	vm
};

fn main() {
	// Compiling grammar
    if let Ok(grammar) = compile_grammar(Grammar::from_file("./grammar.pest").unwrap()) {
		// Creating parser
        let vm = vm::PestVm::new(&grammar).vm;
        let result = vm.parse("rule as &str", "input as &str");
    }
}
```

## License

MIT
