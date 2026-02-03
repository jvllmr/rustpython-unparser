# rustpython-unparser

[![Crates.io Downloads (latest version)](https://img.shields.io/crates/dv/rustpython-unparser)](https://crates.io/crates/rustpython-unparser)
[![Crates.io Version](https://img.shields.io/crates/v/rustpython-unparser)](https://crates.io/crates/rustpython-unparser)
![CI Badge](https://codeberg.org/jvllmr/rustpython-unparser/badges/workflows/test.yaml/badge.svg)

A complete unparser for rustpython-parser ASTs.

## Acknowledgement

I created this library because I quickly needed a way to turn rustpython-parser ASTs back to some form of Python source code. Since the unparser contained within rustpython-ast only supports expressions and not statements, I started implementing my own unparser by taking heavy inspiration from rustpython's unparser and the unparser in the python standard library. Therefore, most credit goes to the respective teams behind the ast standard library module and RustPython. I only glued the parts I needed together to a complete unparser in this repository.

I personally don't use this crate anymore since I switched over to using ruff's internal AST, but I will still accept PRs.
Feel free to contribute if you find out that you are missing something.

## Contributing

You can already help a lot by reporting issues with generated code. If you find an issue,please provide example code of the original source and the unparsed source with a concise explanation.

If you would like to fix the issue yourself, you can create an example in one of the matching files in `test_files` or create a new file with your example if you think your example does not fit into any of the existing files. Then you can run the tests with `cargo t` and verify your changes by viewing your unparse example in the `test_files_unparsed` directory. After verifying your changes you can create a Pull Request. If your change is linked to a GitHub issue, please provide a comment in the form of `# <link to github issue>` next to your example.

## Simple usage example

```rust
use rustpython_unparser::Unparser;
use rustpython_parser::ast::Suite;
use rustpython_parser::Parse;

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

## Transformer

This crate also contains a transformer trait for easy transformation of ASTs with the possibility of removing nodes. Enable the `transformer` feature to use it. It is similar to rustpython-ast's visitor, with the difference that a visit functions always return `Option<...>` and statements/expressions are passed as mutable.
