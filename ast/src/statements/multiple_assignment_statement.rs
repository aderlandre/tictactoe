use crate::{
    ast::Rule,
    common::{Declare, Identifier, LineEnd, Variable},
    expressions::Expression,
};

use pest::Span;
use pest_ast::FromPest;
use std::fmt;

#[derive(Clone, Debug, FromPest, PartialEq)]
#[pest_ast(rule(Rule::statement_multiple_assignment))]
pub struct MultipleAssignmentStatement<'ast> {
    pub declare: Declare,
    pub variables: Vec<Variable<'ast>>,
    pub function_name: Identifier<'ast>,
    pub arguments: Vec<Expression<'ast>>,
    pub line_end: LineEnd,
    #[pest_ast(outer())]
    pub span: Span<'ast>,
}

impl<'ast> fmt::Display for MultipleAssignmentStatement<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, id) in self.variables.iter().enumerate() {
            write!(f, "{}", id)?;
            if i < self.variables.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, " = {}", self.function_name)
    }
}
