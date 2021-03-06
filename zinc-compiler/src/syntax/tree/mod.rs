//!
//! The syntax tree.
//!

pub mod expression;
pub mod field;
pub mod identifier;
pub mod literal;
pub mod pattern_binding;
pub mod pattern_match;
pub mod statement;
pub mod tuple_index;
pub mod r#type;
pub mod variant;

use self::statement::local_mod::Statement as ModuleLocalStatement;

#[derive(Debug, Clone, PartialEq)]
pub struct Tree {
    pub statements: Vec<ModuleLocalStatement>,
}
