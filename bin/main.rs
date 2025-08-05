use std::env;
use std::io;
use std::fs;
use std::io::prelude::*;

use kilate::*;

fn main() -> io::Result<()> {
  // Open file
  let filename = env::args().nth(1).expect("Please provide a file to be parsed!");
  let mut file = fs::File::open(&filename)?;

  // Read file content
  let mut content = String::new();
  file.read_to_string(&mut content)?;

  let mut lexer = lexer::Lexer::new(&content);
  let tokens = lexer.lex();

  // jus display tokens for now
  // for token in &tokens {
    // println!("Token {{");
    // println!("  kind: {:?}", token.kind);
    // println!("  value: {}", token.value);
    // println!("  start: {}", token.start);
    // println!("  end: {}", token.end);
    // println!("}}");
  // }

  // parse
  let mut parser = parser::Parser::new(tokens);
  let nodes = parser.parse();
  println!("Nodes:");
  for node in &nodes {
    println!("{}", node);
  }

  Ok(())
}