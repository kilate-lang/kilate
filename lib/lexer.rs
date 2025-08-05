use std::str::Chars;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Token<'a> {
  // token type
  pub kind: TokenKind,

  // token value
  pub value: &'a str,

  // start offset at source
  pub start: usize,

  // end offset at source
  pub end: usize
}

impl<'a> Token<'a> {
  pub fn is_modifier(&self) -> bool {
    matches!(self.kind, TokenKind::Keyword)
      && ["pub"].contains(&self.value)
  }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TokenKind {
  Eof,         // end of file
  Keyword,     // public, static....
  Identifier,  // name
  Type,        // int, float ...
  LParen,      // (
  RParen,      // )
  LBrace,      // {
  RBrace,      // }
  LBracket,    // [
  RBracket,    // ]
  Comma,       // ,
  Colon,       // :
  Semicolon,   // ;
  Ampersand,   // &
  Pipe,        // |
  GreaterThan, // >
  LessThan,    // <
  Dot,         // .
  LArrow,      // <-
  RArrow,      // ->
}

// lexer struct borrows a string slice with lifetime 'a.
// 'a means "the lifetime of the borrowed string must live at least as long as the Lexer".
pub struct Lexer<'a> {
  // a reference to the full source text
  // &'a str means: a string slice that lives at least as long as lifetime 'a.
  source: &'a str,

  // iterator over the remaining characters in the source.
  // Also tied to lifetime 'a because it borrows from the same source.
  chars: Chars<'a>,

  // the current char being iterated
  current: Option<char>
}

impl<'a> Lexer<'a> {
  pub fn new(source: &'a str) -> Self {
    let mut chars = source.chars();
    let current = chars.next();
    Self {
      source,
      chars,
      current
    }
  }

  fn read_word(&mut self, start: usize) -> (TokenKind, usize) {
    while let Some(c) = self.peek() {
      if c.is_alphanumeric() || c == '_' {
        self.bump();
      } else {
        break;
      }
    }
    let end = self.offset();
    let value = &self.source[start..end];
    let kind = match value {
      // all kilate Keywords
      "work" => TokenKind::Keyword,
      "pub" => TokenKind::Keyword,
      "let" => TokenKind::Keyword,
      "var" => TokenKind::Keyword,
      "true" => TokenKind::Keyword,
      "false" => TokenKind::Keyword,
      "return" => TokenKind::Keyword,
      "import" => TokenKind::Keyword,

      // all kilate types
      "int" => TokenKind::Type,
      "float" => TokenKind::Type,
      "long" => TokenKind::Type,
      "any" => TokenKind::Type,
      "string" => TokenKind::Type,
      "bool" => TokenKind::Type,

      // others
      "<-" => TokenKind::LArrow,
      "->" => TokenKind::RArrow,
      _ => TokenKind::Identifier
    };
    (kind, end)
  }

  fn read_next_token(&mut self) -> Option<Token<'a>> {
    while let Some(c) = self.peek() {
      if c.is_whitespace() {
        self.bump();
      } else {
        break;
      }
    }
    let start = self.offset();
    if let Some(c) = self.bump() {
      if c == ';' { return None; }
      match c {
        '{' | '}' | '(' | ')' | '[' | ']' | ',' | ':' | '&' | '|' | '>' | '<' | '.' => {
          let end = self.offset();
          let value = &self.source[start..end];
          let kind = match c {
              '{' => TokenKind::LBrace,
              '}' => TokenKind::RBrace,
              '(' => TokenKind::LParen,
              ')' => TokenKind::RParen,
              '[' => TokenKind::LBracket,
              ']' => TokenKind::RBracket,
              ',' => TokenKind::Comma,
              ':' => TokenKind::Colon,
              '&' => TokenKind::Ampersand,
              '|' => TokenKind::Pipe,
              '>' => TokenKind::GreaterThan,
              '<' => TokenKind::LessThan,
              '.' => TokenKind::Dot,
              _ => unreachable!(),
          };
          Some(Token { kind, value, start, end })
        },
        _ch if c.is_alphabetic() => {
          let (kind, end) = self.read_word(start);
          let value = &self.source[start..end];
          Some(Token { kind, value, start, end })
        },
        _ => {
          self.chars.next();
          let end = self.offset();
          let value = &self.source[start..end];
          Some(Token { kind: TokenKind::Identifier, value, start, end })
        }
      }
    } else {
      Some(Token { kind: TokenKind::Eof, value: "", start, end: start })
    }
  }

  fn offset(&self) -> usize {
    self.source.len() - self.chars.as_str().len() - self.current.map_or(0, |c| c.len_utf8())
  }

  fn bump(&mut self) -> Option<char> {
    let next = self.chars.next();
    std::mem::replace(&mut self.current, next)
  }

  fn peek(&self) -> Option<char> {
    self.current
  }

  pub fn lex(&mut self) -> Vec<Token> {
    let mut tokens = Vec::new();
    loop {
      if let Some(token) = self.read_next_token() {
        tokens.push(token);
        if token.kind == TokenKind::Eof {
          break
        }
      } else {
        continue
      }
    }
    tokens
  }
}