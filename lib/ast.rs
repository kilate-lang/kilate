use super::lexer;
use std::fmt;

#[derive(Debug, Clone)]
pub enum AstNode {
  Function(FunctionNode),
  Param(ParamNode),
  Call(Box<CallNode>),
}

#[derive(Debug, Clone)]
pub struct FunctionNode {
  pub name: String,
  pub modifiers: Vec<String>,
  pub params: Vec<AstNode>,
  pub body: Vec<AstNode>,
  pub return_type: String,
}

#[derive(Debug, Clone)]
pub struct ParamNode {
  pub name: String,
  pub kind: String,
  pub is_array: bool,
}

#[derive(Debug, Clone)]
pub struct CallNode {
  pub function: Box<AstNode>,
  pub params: Vec<AstNode>,
}

impl AstNode {
  pub fn fmt_with_indent(&self, f: &mut fmt::Formatter, indent: usize) -> fmt::Result {
    let pad = "  ".repeat(indent);
    match self {
      AstNode::Function(m) => {
        writeln!(f, "{}Function: {}", pad, m.name)?;
        writeln!(f, "{}  Modifiers: {:?}", pad, m.modifiers)?;
        writeln!(f, "{}  Return: {}", pad, m.return_type)?;
        writeln!(f, "{}  Params:", pad)?;
        for param in &m.params {
          param.fmt_with_indent(f, indent + 2)?;
        }
        writeln!(f, "{}  Body:", pad)?;
        for node in &m.body {
          node.fmt_with_indent(f, indent + 2)?;
        }
        Ok(())
      }
      AstNode::Param(p) => {
        let array_suffix = if p.is_array { "[]" } else { "" };
        writeln!(f, "{}Param: {} {}{}", pad, p.kind, p.name, array_suffix)
      },
      AstNode::Call(c) => {
        c.function.fmt_with_indent(f, indent + 2)?;
        for param in &c.params {
          param.fmt_with_indent(f, indent + 2)?;
        }
        Ok(())
      }
    }
  }
}

impl fmt::Display for AstNode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.fmt_with_indent(f, 0)
  }
}