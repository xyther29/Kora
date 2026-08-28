mod lexer;

use lexer::lexer::Lexer;

fn main() {
    let source = "let x = 42;";
    let _lexer = Lexer::new(source);
    println!("Lexer created succcessfully");
}
