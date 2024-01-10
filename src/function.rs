use crate::identifier::Identifier;

#[derive(Debug)]
pub enum PatternElement {
    Identifier(Identifier),
}

impl PatternElement {
    pub fn ident(name: Identifier) -> Self {
        Self::Identifier(name)
    }
}

#[derive(Debug)]
struct Pattern {
    elements: Vec<PatternElement>,
}

impl Pattern {
    fn new() -> Self {
        Self { elements: vec![] }
    }

    fn add(&mut self, arg: PatternElement) {
        self.elements.push(arg)
    }
}

#[derive(Debug)]
pub struct FunctionDescriptor {
    name: Identifier,
    arguments: Pattern,
}

impl FunctionDescriptor {
    pub fn new(name: Identifier) -> Self {
        Self {
            name,
            arguments: Pattern::new(),
        }
    }

    pub fn add_arg(&mut self, arg: PatternElement) {
        self.arguments.add(arg)
    }
}
