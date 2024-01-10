use super::NodeKind;
use crate::{function::FunctionDescriptor, Identifier};

#[derive(Debug)]
pub struct ASTNode {
    kind: NodeKind<ASTNode>,
}

impl ASTNode {
    pub fn is_empty(&self) -> bool {
        matches!(self.kind, NodeKind::Empty)
    }

    pub fn empty() -> Self {
        Self {
            kind: NodeKind::Empty,
        }
    }

    pub fn atom(name: Identifier) -> Self {
        Self {
            kind: NodeKind::Atom(name),
        }
    }

    pub fn number(x: f64) -> Self {
        Self {
            kind: NodeKind::Number(x),
        }
    }

    pub fn call(function: ASTNode, arg: ASTNode) -> Self {
        Self {
            kind: NodeKind::Call {
                function: function.into(),
                arg: arg.into(),
            },
        }
    }

    pub fn define_function(function: FunctionDescriptor, body: ASTNode) -> Self {
        Self {
            kind: NodeKind::Define {
                function,
                body: body.into(),
            },
        }
    }
}
