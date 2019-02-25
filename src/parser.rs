use super::lexer;
use super::token::*;
use std::cell::{Cell, RefCell};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Ident<'a>(&'a str);
impl<'a> fmt::Display for Ident<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, PartialEq)]
pub struct Expr {
  node: ExprKind,
}
impl<'a> fmt::Display for Expr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.node)
  }
}

#[allow(dead_code)]
enum LiteralKind {
  Int(i32),
  Char(),
}

#[derive(Debug, PartialEq)]
pub struct ParseErr {
  msg: String,
}

impl ParseErr {
  pub fn new(expected: &TokenKind, actual: &TokenKind) -> ParseErr {
    ParseErr {
      msg: format!(
        "expected next token to be {:?}, but got {:?}",
        expected, actual
      ),
    }
  }
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum ExprKind {
  Literal(i32),
  Ident(String),
}
impl<'a> fmt::Display for ExprKind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ExprKind::Literal(num) => write!(f, "{}", num),
      ExprKind::Ident(s) => write!(f, "{}", s),
    }
  }
}

#[derive(Debug, PartialEq)]
struct Stmt<'a> {
  node: StmtKind<'a>,
}

impl<'a> fmt::Display for Stmt<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.node)
  }
}

#[derive(Debug, PartialEq)]
pub struct LetStmt<'a> {
  name: Ident<'a>,
  value: Expr,
}

#[derive(Debug, PartialEq)]
pub struct ReturnStmt {
  value: Expr,
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum StmtKind<'a> {
  Bad,
  // Expr(ExprStmt<'a>),
  Let(LetStmt<'a>),
  Return(ReturnStmt),
}

impl<'a> fmt::Display for StmtKind<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      StmtKind::Bad => write!(f, "Bad!"),
      StmtKind::Let(let_stmt) => write!(f, "let {} = {};", let_stmt.name, let_stmt.value),
      StmtKind::Return(ret_stmt) => write!(f, "return {};", ret_stmt.value),
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct Program<'a> {
  stmts: Vec<Stmt<'a>>,
}

impl<'a> fmt::Display for Program<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for stmt in &self.stmts {
      write!(f, "{}", stmt)?
    }
    Ok(())
  }
}

#[allow(dead_code)]
pub struct Parser<'a> {
  lex: &'a lexer::Lexer<'a>,
  cur: Cell<Token<'a>>,
  peek: Cell<Token<'a>>,
  pub errors: RefCell<Vec<ParseErr>>,
}

#[allow(dead_code)]
impl<'a> Parser<'a> {
  pub fn new(lex: &'a lexer::Lexer) -> Parser<'a> {
    let cur = lex.next_token();
    let peek = lex.next_token();
    Parser {
      lex,
      cur: Cell::new(cur),
      peek: Cell::new(peek),
      errors: RefCell::new(vec![]),
    }
  }

  pub fn next_token(&self) {
    self.cur.set(self.peek.get());
    self.peek.set(self.lex.next_token());
  }

  pub fn parse(&self) -> Program {
    let mut p = Program { stmts: vec![] };

    loop {
      match self.cur.get().kind {
        TokenKind::Eof => break,
        TokenKind::Let => p.stmts.push(self.parse_let_stmt()),
        TokenKind::Return => p.stmts.push(self.parse_return_stmt()),
        _ => break,
      }
      self.next_token();
    }
    return p;
  }

  pub fn expect_peek(&self, t: TokenKind) -> bool {
    let peeked = self.peek.get();
    let is_match = peeked.kind == t;

    if is_match {
      self.next_token()
    } else {
      self
        .errors
        .borrow_mut()
        .push(ParseErr::new(&t, &peeked.kind))
    }

    is_match
  }

  fn parse_let_stmt(&self) -> Stmt {
    let mut is_bad: bool = false;

    if !self.expect_peek(TokenKind::Ident) {
      is_bad = true;
    }

    let name = match self.cur.get() {
      Token {
        kind: TokenKind::Ident,
        literal: name,
      } => Some(name),
      _ => None,
    };

    if !self.expect_peek(TokenKind::Assign) {
      is_bad = true;
    }

    while self.cur.get().kind != TokenKind::Semicolon {
      self.next_token();
    }

    if is_bad {
      Stmt {
        node: StmtKind::Bad,
      }
    } else {
      Stmt {
        node: StmtKind::Let(LetStmt {
          name: Ident(name.unwrap()),
          value: Expr {
            node: ExprKind::Literal(5),
          },
        }),
      }
    }
  }

  fn parse_return_stmt(&self) -> Stmt {
    self.next_token();

    // parseExpression
    while self.cur.get().kind != TokenKind::Semicolon {
      self.next_token();
    }

    return Stmt {
      node: StmtKind::Return(ReturnStmt {
        value: Expr {
          node: ExprKind::Literal(1337),
        },
      }),
    };
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_simple_return_statement() {
    let lexer = lexer::Lexer::new("return = 1. 89- {}{!!!}!@#$%^&* 5;");
    let parser = Parser::new(&lexer);
    let s = parser.parse();

    assert_eq!(
      s,
      Program {
        stmts: vec![Stmt {
          node: StmtKind::Return(ReturnStmt {
            value: Expr {
              node: ExprKind::Literal(1337)
            }
          })
        }]
      }
    );
  }
  #[test]
  fn test_simple_let_statement() {
    let lexer = lexer::Lexer::new("let x = 5;");
    let parser = Parser::new(&lexer);
    let s = parser.parse();
    assert_eq!(
      s,
      Program {
        stmts: vec![Stmt {
          node: StmtKind::Let(LetStmt {
            name: Ident("x"),
            value: Expr {
              node: ExprKind::Literal(5)
            }
          })
        }]
      }
    );
  }
}
