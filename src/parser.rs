use super::lexer;
use super::token;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Ident<'a>(&'a str);

#[derive(Debug, PartialEq)]
pub struct Expr {
  node: ExprKind,
}

enum LiteralKind {
  Int(i32),
  Char(),
}

#[derive(Debug, PartialEq)]
pub struct ParseErr {
  msg: String,
}

impl ParseErr {
  pub fn new(expected: &token::Token, actual: &token::Token) -> ParseErr {
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

#[derive(Debug, PartialEq)]
struct Stmt<'a> {
  node: StmtKind<'a>,
}

#[derive(Debug, PartialEq)]
pub struct LetStmt<'a> {
  name: Ident<'a>,
  value: Expr,
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum StmtKind<'a> {
  Bad,
  Let(LetStmt<'a>),
}

#[derive(Debug, PartialEq)]
pub struct Program<'a> {
  stmts: Vec<Stmt<'a>>,
}

#[allow(dead_code)]
pub struct Parser<'a> {
  lex: &'a lexer::Lexer<'a>,
  cur: Cell<token::Token<'a>>,
  peek: Cell<token::Token<'a>>,
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
      match self.cur.get() {
        token::Token::Eof => break,
        token::Token::Let => p.stmts.push(self.parse_let_stmt()),
        _ => break,
      }
      self.next_token();
    }
    return p;
  }

  pub fn expect_peek(&self, t: token::Token) -> bool {
    let peeked = self.peek.get();
    let is_match = match (peeked, t) {
      (token::Token::Ident(_), token::Token::Ident(_)) => true,
      (token::Token::Int(_), token::Token::Int(_)) => true,
      (peek, t) => peek == t,
    };

    if is_match {
      self.next_token()
    } else {
      self.errors.borrow_mut().push(ParseErr::new(&t, &peeked))
    }

    is_match
  }

  fn parse_let_stmt(&self) -> Stmt {
    let mut is_bad: bool = false;

    if !self.expect_peek(token::Token::Ident("")) {
      is_bad = true;
    }

    let name = match self.cur.get() {
      token::Token::Ident(name) => Some(name),
      _ => None,
    };

    if !self.expect_peek(token::Token::Assign) {
      is_bad = true;
    }

    while self.cur.get() != token::Token::Semicolon {
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
}

#[cfg(test)]
mod tests {
  use super::*;

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
