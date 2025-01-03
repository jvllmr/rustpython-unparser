# rustpython-unparser

A complete unparser for rustpython-parser ASTs.

## Acknowledgement

I created this library because I quickly needed a way to turn rustpython-parser ASTs back to some form of Python source code. Since the unparser contained within rustpython-ast only supports expressions and not statements, I started implementing my own unparser by taking heavy inspiration from rustpython's unparser and the unparser in the python standard library. Therefore, most of credit goes to the teams behind the ast standard library module and the RustPython team. I only glued the parts I needed together to a complete unparser in this repository.

## Contributing

You can already help a lot by reporting issues with generated code. If you find an issue,please provide example code of the original source and the unparsed source with a concise explanation.

If you would like to fix the issue yourself, you can create an example in one of the matching files in `test_files` or create a new file with your example if you think your example does not fit into any of the existing files. Then you can run the tests with `cargo t` and verify your changes by viewing your unparse example in the `test_files_unparsed` directory. After verifying your changes you can create a Pull Request. If your change is linked to a GitHub issue, please provide a comment in the form of `# <link to github issue>` next to your example.

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
