mod lexer;

use lexer::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new("123");
    let token = lexer.next_token();
    println!("{:?}", token);
}
