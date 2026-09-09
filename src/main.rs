mod ast;
mod diagnostics;
mod lexer;
mod parser;
mod source;
mod symbols;
mod utils;

use std::{env, fs, path::PathBuf, str::FromStr};

use bumpalo::Bump;

use crate::{ast::items::ItemKind, parser::Parser, source::SourceFile};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();

    let _program = args.next();
    let path = args.next().ok_or("missing input file")?;

    let source = fs::read_to_string(&path)?;

    let src_file = SourceFile::new(PathBuf::from_str(&path)?, 0);

    let bump = Bump::new();

    let mut parser = Parser::new(&src_file, &bump);

    while let Ok(item) = parser.next()
        && parser.ast_mut().items()[item.index()].kind != ItemKind::EOF
    {
        println!("\n---- Item ----\n{:?}\n", item);
    }

    println!("---- File contents ----\n{}", source);

    Ok(())
}
