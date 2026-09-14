mod ast;
mod diagnostics;
mod lexer;
mod module;
mod parser;
// mod resolver;
mod session;
mod source;
mod utils;

use std::{env, path::PathBuf, str::FromStr};

use crate::{module::ModuleCollector, session::Session};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();

    let _program = args.next();

    let path = args.next().ok_or("missing input file")?;

    let mut session = Session::new();

    ModuleCollector::collect(&mut session, PathBuf::from_str(&path)?);

    Ok(())
}
