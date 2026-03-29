use std::env;
use std::fs;
use std::io::Read;

mod codegen;
mod ir;
mod lexer;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    match &args.as_slice() {
        [binary] => {
            eprintln!("Usage: {} <binary>", &binary);
        }
        [_, source] => {
            let mut source = fs::File::open(&source)?;
            let mut src = String::new();
            source.read_to_string(&mut src)?;
            let tokens = lexer::lex(&src);
            let ir = ir::generate_ir(&tokens);
            let mut interpreter = codegen::interpreter::Interpreter::new();
            interpreter.run(&ir);
        }
        [binary, ..] => {
            eprintln!("Usage: {} <filename>", &binary);
        }
        [] => {
            eprintln!("Usage: <binary> <filename>")
        }
    }

    Ok(())
}
