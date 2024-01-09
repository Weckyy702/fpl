use anyhow::Result;

use fl::{lexer::Lexer, parser::Parser};

fn main() -> Result<()> {
    let program = r#"
        # f:: Number -> Number
        # f: a b -> + a b

       print (+ 1 (+ 2 3));
"#;

    let tokens = Lexer::new(program.chars()).collect::<Result<Vec<_>, _>>()?;

    for node in Parser::new(tokens) {
        let node = node?;
        println!("{node:#?}")
    }

    Ok(())
}
