use super::TyID;
use crate::{define_id, source::Span, utils::arena::Arena};

define_id!(StructID);
define_id!(EnumID);
define_id!(UnionID);
define_id!(FnID);
define_id!(TypeAliasID);
define_id!(ConstID);

pub struct DefTable {
    pub structs: Arena<StructID, StructDef>,
    pub enums: Arena<EnumID, EnumDef>,
    pub unions: Arena<UnionID, UnionDef>,
    pub functions: Arena<FnID, FunctionDef>,
    pub type_aliases: Arena<TypeAliasID, TypeAliasDef>,
    pub consts: Arena<ConstID, ConstDef>,
}

impl DefTable {
    pub fn new() -> Self { /* ... */
    }

    pub fn add_struct(&mut self, def: StructDef) -> StructID { /* ... */
    }
    pub fn add_enum(&mut self, def: EnumDef) -> EnumID { /* ... */
    }
    // ... etc.
}
