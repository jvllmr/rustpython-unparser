pub mod unparser;

pub use crate::unparser::Unparser;

#[cfg(test)]
mod tests {
    use super::*;
    use rustpython_parser::ast::Suite;
    use rustpython_parser::Parse;

    use std::fs;
    use std::io;
    use std::path::Path;
    #[test]
    fn test_predefined_files() -> io::Result<()> {
        for entry in fs::read_dir("./test_files")? {
            let entry = entry?;

            let entry_path = entry.path();

            if entry_path.is_file() {
                let file_content = fs::read_to_string(&entry_path)?;
                let entry_path_str = entry_path.to_str().unwrap();
                let mut unparser = Unparser::new();
                let stmts = Suite::parse(&file_content, entry_path_str).unwrap();
                for stmt in &stmts {
                    unparser.unparse_stmt(stmt);
                }
                let new_source = unparser.source;
                let old_file_name = entry_path.file_name().unwrap().to_str().unwrap();
                let new_file_name = old_file_name.replace(".py", "_unparsed.py");
                let new_entry_path_str = format!("./test_files_unparsed/{}", new_file_name);
                let new_entry_path = Path::new(&new_entry_path_str);
                fs::write(&new_entry_path, &new_source)?;
                let new_stmts =
                    Suite::parse(&new_source, new_entry_path.to_str().unwrap()).unwrap();
                for (stmt, new_stmt) in stmts.iter().zip(new_stmts.iter()) {
                    assert_eq!(stmt, new_stmt)
                }
            }
        }
        Ok(())
    }
}
