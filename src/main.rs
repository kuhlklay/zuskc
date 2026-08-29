mod ast;
mod lexer;
mod parser;
mod source;

use std::{env, fs, path::PathBuf, str::FromStr};

use bumpalo::Bump;

use crate::{ast::Item, parser::Parser, source::SourceFile};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();

    let _program = args.next();
    let path = args.next().ok_or("missing input file")?;

    let source = fs::read_to_string(&path)?;

    let src_file = SourceFile::new(PathBuf::from_str(&path)?, 0);

    let bump = Bump::new();

    let mut parser = Parser::new(&src_file, &bump);

    while let Ok(stmt) = parser.next()
        && stmt != Item::EOF
    {
        println!("\n---- Statement ----\n{:?}\n", stmt);
    }

    println!("---- File contents ----\n{}", source);

    Ok(())
}
