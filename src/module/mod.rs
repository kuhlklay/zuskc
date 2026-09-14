mod collect;

use std::{collections::HashMap, path::PathBuf};

use crate::{
    ast::{AST, Publicity, items::Item, string::StringID},
    define_id,
    source::{FileID, Span},
    utils::arena::Arena,
};

define_id!(ModuleID);

pub struct ParsedModule {
    module_id: ModuleID,
    base_dir: PathBuf,
    ast: AST,
    modules: Vec<(StringID, Span)>,
}

pub struct ModuleCollector;

#[derive(Debug, PartialEq)]
pub struct Module {
    pub id: ModuleID,
    pub parent: Option<ModuleID>,
    pub children: Vec<ModuleID>,
    pub file_id: Option<FileID>, // None for inline mods
    pub path: Option<PathBuf>,   // None for inline mods
    pub scope: ModuleScope,
}

impl Module {
    pub fn new(
        id: ModuleID,
        parent: Option<ModuleID>,
        file_id: Option<FileID>,
        path: Option<PathBuf>,
    ) -> Self {
        Self {
            id,
            parent,
            children: Vec::new(),
            file_id,
            path,
            scope: ModuleScope::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ModuleSource {
    File { file_id: FileID, path: PathBuf },
    Inline { span: Span },
}

pub struct ModuleTree {
    modules: Arena<ModuleID, Module>,
    root: ModuleID,
    file_to_module: HashMap<FileID, ModuleID>,
}

impl ModuleTree {
    pub fn new(root_path: PathBuf, root_file_id: FileID) -> Self {
        let mut modules: Arena<ModuleID, Module> = Arena::new();

        let root_id = modules.push(Module::new(
            ModuleID::from(0),
            None,
            Some(root_file_id),
            Some(root_path),
        ));

        let mut file_to_module = HashMap::new();
        file_to_module.insert(root_file_id, root_id);

        Self {
            modules,
            root: root_id,
            file_to_module,
        }
    }

    pub fn add_child(&mut self, parent: ModuleID, path: PathBuf, file_id: FileID) -> ModuleID {
        let id = ModuleID::from(self.modules.len());
        let module = Module::new(id, Some(parent), Some(file_id), Some(path));
        let child_id = self.modules.push(module);

        self.modules[parent.index()].children.push(child_id);
        self.file_to_module.insert(file_id, child_id);

        child_id
    }

    pub fn get(&self, id: ModuleID) -> &Module {
        &self.modules[id.index()]
    }

    pub fn get_mut(&mut self, id: ModuleID) -> &mut Module {
        &mut self.modules[id.index()]
    }

    pub fn module_of(&self, file_id: FileID) -> Option<ModuleID> {
        self.file_to_module.get(&file_id).copied()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Namespace {
    Type,
    Value,
    Macro,
}

#[derive(Debug, PartialEq)]
pub struct ScopeEntry {
    // pub kind: DefKind,
    pub publicity: Publicity,
    pub span: Span,           // for diagnostics
    pub defined_in: ModuleID, // for "defined here" notes; often == owning module
}

#[derive(Debug, Default, PartialEq)]
pub struct ModuleScope {
    pub types: HashMap<usize, ScopeEntry>,
    pub values: HashMap<usize, ScopeEntry>,
    pub macros: HashMap<usize, ScopeEntry>,
}

impl ModuleScope {
    pub fn namespace(&mut self, nsp: Namespace) -> &mut HashMap<usize, ScopeEntry> {
        match nsp {
            Namespace::Type => &mut self.types,
            Namespace::Value => &mut self.values,
            Namespace::Macro => &mut self.macros,
        }
    }
    // pub fn get(&self, ns: Namespace, name: usize) -> Option<&ScopeEntry> {  }
}
