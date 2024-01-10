use crate::{function::FunctionDescriptor, Identifier};

pub mod untyped;

#[derive(Debug)]
enum NodeKind<Node> {
    Empty,
    Number(f64),
    Atom(Identifier),
    Call {
        function: Box<Node>,
        arg: Box<Node>,
    },
    Define {
        function: FunctionDescriptor,
        body: Box<Node>,
    },
}

struct AST<Node> {
    nodes: Vec<Node>,
}
