use anyhow::Result;

use fl::{lexer::Lexer, parser::Parser};

fn main() -> Result<()> {
    let program = r#"
#       f:: Number -> Number -> Number
       f: a -> * 2 a;

        print 1;
        print (+ 1 2);
"#;

    let tokens = Lexer::new(program.chars()).collect::<Result<Vec<_>, _>>()?;
    let nodes = Parser::new(tokens).collect::<Result<Vec<_>, _>>()?;

    println!("{nodes:#?}");

    Ok(())
}
