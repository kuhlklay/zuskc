use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use crate::{
    ast::items::ItemKind,
    module::{ModuleCollector, ModuleID, ModuleTree, ParsedModule},
    parser::Parser,
    session::Session,
    source::FileID,
};

impl ModuleCollector {
    pub fn collect(session: &Session, entry: PathBuf) {
        let entry = match entry.canonicalize() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        };

        let content = std::fs::read_to_string(&entry).expect("read file");
        let root_file_id = session.add_file(entry.clone(), content);

        let mut module_tree = ModuleTree::new(entry.clone(), root_file_id);
        let root_module_id = module_tree.root;

        let root = Self::parse(session, entry.clone(), root_file_id, root_module_id);
        let base_dir = root.base_dir;
        let mut pending = vec![(base_dir, root_module_id, root.modules)];

        let mut seen = HashSet::<PathBuf>::new();
        seen.insert(entry);

        while !pending.is_empty() {
            let current = std::mem::take(&mut pending);
            let mut to_parse = Vec::new();

            for (base_dir, parent_id, includes) in current {
                for (name, _span) in includes {
                    let name_str = session.lookup(name);
                    let Some(path) = Self::resolve_path(&base_dir, &name_str) else {
                        panic!("module {name_str} not found in {}", base_dir.display());
                    };
                    let path = match path.canonicalize() {
                        Ok(p) => p,
                        Err(_) => continue,
                    };
                    if !seen.insert(path.clone()) {
                        continue;
                    }

                    let content = std::fs::read_to_string(&path).expect("read file");
                    let file_id = session.add_file(path.clone(), content);
                    let module_id = module_tree.add_child(parent_id, path.clone(), file_id);
                    to_parse.push((path, file_id, module_id));
                }
            }

            let parsed: Vec<ParsedModule> = to_parse
                .into_iter()
                .map(|(path, file_id, module_id)| Self::parse(session, path, file_id, module_id))
                .collect();

            for file in parsed {
                if !file.modules.is_empty() {
                    pending.push((file.base_dir, file.module_id, file.modules));
                }
            }
        }
    }

    fn parse(
        session: &Session,
        path: PathBuf,
        file_id: FileID,
        module_id: ModuleID,
    ) -> ParsedModule {
        let base_dir = path.parent().unwrap_or(Path::new(".")).to_owned();
        let mut parser = Parser::new(session, file_id);
        let mut modules = Vec::new();

        loop {
            let id = match parser.next() {
                Ok(id) => id,
                Err(()) => panic!("got error of parsed item"),
            };
            let item = &parser.ast().items[id.index()];
            match &item.kind {
                ItemKind::Mod(m) if !m.is_inline() => modules.push((m.ident.id(), m.ident.span())),
                ItemKind::EOF => break,
                _ => {}
            }
        }

        ParsedModule {
            module_id,
            base_dir,
            ast: parser.take_ast(),
            modules,
        }
    }

    fn resolve_path(base_dir: &Path, name: &str) -> Option<PathBuf> {
        let file = base_dir.join(name).with_extension("zk");
        if file.exists() {
            return Some(file);
        }
        let mod_file = base_dir.join(name).join("mod.zk");
        if mod_file.exists() {
            return Some(mod_file);
        }
        None
    }
}
