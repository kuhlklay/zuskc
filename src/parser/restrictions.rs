use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Restrictions: u16 {
        const STMT_EXPR = 1 << 0;
        const NO_STRUCT_LITERAL = 1 << 1;
        const CONST_EXPR = 1 << 2;
        const ALLOW_DEC = 1 << 3;
        const IN_IF_GUARD = 1 << 4;
        const IS_PATTERN = 1 << 5;
        const IN_TRAIT = 1 << 6;
    }
}
