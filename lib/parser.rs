use super::lexer;
use super::ast;
use colored::*;

macro_rules! parsing_error {
  ($($arg:tt)*) => {{
    eprintln!("{}: {}", "Parsing error".red(), format!($($arg)*));
    std::process::exit(1);
  }};
}

pub struct Parser<'a> {
  tokens: Vec<lexer::Token<'a>>,
  nodes: Vec<ast::AstNode>,
  allocated_strings: Vec<String>,
  pos: usize,
}

impl<'a> Parser<'a> {
  pub fn new(tokens: Vec<lexer::Token<'a>>) -> Self {
    Self {
      tokens,
      nodes: Vec::new(),
      allocated_strings: Vec::new(),
      pos: 0
    }
  }

  pub fn parse(&mut self) -> Vec<ast::AstNode> {
    while let Some(token) = self.current_token().cloned() {
      match token.kind {
        lexer::TokenKind::Keyword => {
          let node = self.parse_pub_or_keyword(&token);
          self.nodes.push(node);
        },
        lexer::TokenKind::Identifier => {
          let node = self.parse_statement(&token).expect(&format!("Failed to parse statement: '{}'", token.value));
          self.nodes.push(node);
        },
        _ => break,
      }
    }

    self.nodes.clone()
  }

  fn parse_pub_or_keyword(&mut self, token: &lexer::Token<'a>) -> ast::AstNode {
    match token.value {
      "work" => self.parse_function().expect("Failed to parse 'work' function"),
      "pub" => {
          if let Some(next) = self.next_token() {
              if next.kind == lexer::TokenKind::Keyword && next.value == "work" {
                  self.parse_function().expect("Failed to parse 'pub work' function")
              } else {
                  parsing_error!("Unexpected token after 'pub': {:?}", next);
              }
          } else {
              parsing_error!("Unexpected end after 'pub'");
          }
      }
      _ => parsing_error!("Unexpected keyword: {:?}", token),
    }
  }

  // get current token
  fn current_token(&self) -> Option<&lexer::Token<'a>> {
    self.tokens.get(self.pos)
  }

  // get next token without advance
  fn next_token(&self) -> Option<&lexer::Token<'a>> {
    self.tokens.get(self.pos + 1)
  }

  // advance and returns the token
  fn consume(&mut self) -> Option<&lexer::Token<'a>> {
    let token = self.tokens.get(self.pos);
    self.pos += 1;
    token
  }

  // returns the current token if its the expected token
  fn expect(&mut self, expected: lexer::TokenKind) -> Option<&lexer::Token<'a>> {
    if let Some(t) = self.current_token() {
      if t.kind == expected {
        return self.consume();
      }
    }
    None
  }

  fn find_function(&mut self, name: &'a str) -> Option<&ast::FunctionNode> {
    if let Some(ast::AstNode::Function(func)) = self.nodes.iter().find(|node| matches!(node, ast::AstNode::Function(f) if f.name == name)) {
      return Some(func)
    }
    None
  }

  fn alloc(&mut self, s: String) -> &str {
    self.allocated_strings.push(s);
    self.allocated_strings.last().unwrap().as_str()
  }

  fn parse_function(&mut self) -> Option<ast::AstNode> {
    // parse modifiers
    let mut modifiers = Vec::new();
    while let Some(t) = self.current_token().cloned() {
      if t.is_modifier() {
        self.consume();
        modifiers.push(t.value.to_string());
        continue;
      }
      break;
    }

    // 'work'
    self.expect(lexer::TokenKind::Keyword)
        .expect("Expected 'work' keyword before function");

    // function name
    let name_token = self.expect(lexer::TokenKind::Identifier)
        .expect("Expected function name identifier");
    let name = name_token.value;

    // open paren
    self.expect(lexer::TokenKind::LParen)
        .expect("Expected '(' after function name");

    let params = self.parse_params()
        .expect("Failed to parse parameters");

    self.expect(lexer::TokenKind::RParen)
        .expect("Expected ')' after function parameters");

    // expect ':'
    let mut return_type = String::new();
    if let Some(t) = self.current_token() {
      if t.kind == lexer::TokenKind::Colon {
        self.expect(lexer::TokenKind::Colon)
            .expect("Expected ':' after function parameters");

        // return type
        let return_kind_token = self.expect(lexer::TokenKind::Type)
            .expect("Expected return type after ':'");
        return_type = return_kind_token.value.to_string();
      }
    }

    // open body
    self.expect(lexer::TokenKind::LBrace)
        .expect("Expected '{' to start function body");

    // body
    let mut body = Vec::new();
    loop {
      if let Some(t) = self.current_token().cloned() {
        if t.kind == lexer::TokenKind::RBrace {
          break
        }
        if let Some(node) = self.parse_statement(&t) {
          body.push(node);
        } else {
          self.consume();
        }
      } else {
        break;
      }
    }

    self.expect(lexer::TokenKind::RBrace)
        .expect("Expected '}' to close function body");

    Some(ast::AstNode::Function(ast::FunctionNode {
        name: name.to_string(),
        modifiers,
        params,
        body,
        return_type: return_type.to_string()
    }))
  }
  
  fn parse_params(&mut self) -> Result<Vec<ast::AstNode>, String> {
    let mut params = Vec::new();

    if let Some(t) = self.current_token() {
        if t.kind == lexer::TokenKind::RParen {
            return Ok(params);
        }
    }

    loop {
      // type
      let mut is_array = false;
      let kind_token = self.expect(lexer::TokenKind::Type)
          .expect("Expected parameter type");

      let kind = kind_token.value;

      // array detection: expects Type followed by [ ]
      if let Some(t) = self.current_token() {
        if t.kind == lexer::TokenKind::LBracket {
          self.consume(); // [
          self.expect(lexer::TokenKind::RBracket)
              .expect("Expected closing ']' after '[' in parameter type");
          is_array = true;
        }
      }

      // expect colon
      self.expect(lexer::TokenKind::Colon)
          .expect("Expected ':' after parameter type");

      // name
      let name_token = self.expect(lexer::TokenKind::Identifier).expect("Expected parameter name after ':'");
      let name = name_token.value;

      params.push(ast::AstNode::Param(ast::ParamNode {
          kind: kind.to_string(),
          name: name.to_string(),
          is_array,
      }));

      match self.current_token() {
        Some(t) if t.kind == lexer::TokenKind::Comma => {
          self.consume(); // consume comma
        }
        Some(t) if t.kind == lexer::TokenKind::RParen => break,
        _ => return Err("Expected ',' or ')' in parameter list".into()),
      }
    }

    Ok(params)
  }

  fn parse_statement(&mut self, token: &lexer::Token<'a>) -> Option<ast::AstNode> {
    if let Some(func) = self.find_function(token.value).cloned() {
      self.consume();
      return self.parse_call(func);
    }
    None
  }

  fn parse_call(&mut self, func: ast::FunctionNode) -> Option<ast::AstNode> {
    if let Some(next) = self.next_token() {
      if next.kind == lexer::TokenKind::LParen {
        self.expect(lexer::TokenKind::LParen);
        self.expect(lexer::TokenKind::RParen);
        println!("found a call: {}", func.name);
        return Some(ast::AstNode::Call(Box::new(ast::CallNode{
          function: Box::new(ast::AstNode::Function(func)),
          params: vec!()
        })));
      }
    }
    None
  }
}