# rustpython-unparser

> [!IMPORTANT]
> rustpython-unparser is in early development and WIP.

A complete unparser for rustpython-parser ASTs.


## Simple usage example

```rust
use rustpython_unparser::Unparser;
use rustpython_parser::ast::Suite;
use rustpython_parser::Parse;
use std::fs;
fn main() {
    // ...
    let unparser = Unparser::new();
    let stmts = Suite::parse(source_str, file_path);
    for stmt in &stmts {
        unparser.unparse_stmt(stmt);
    }
    let new_source = unparser.source;
    // ...
}



```