use crate::ast::{BinOp, Exp};
use crate::token::TokenType;
use std::iter::Peekable;
use std::vec::IntoIter;

/*
   Notes for grammar and semantics goes here:
   Program is just expression for now
    - Maybe later list of decls...

    TODO: Fix messy stuff, lots copied from old parser
*/

type TokenIter = Peekable<IntoIter<TokenType>>;
type ErrorType = &'static str;

pub struct Parser {
    tokens: TokenIter,
}

impl Parser {
    pub fn new(tokens: Vec<TokenType>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    fn eat(&mut self, expected: &TokenType) {
        let actual = self.tokens.peek().expect("No TokenType present to eat");
        assert_eq!(
            expected, actual,
            "Received wrong TokenType to eat, expected {}, but got {}",
            expected, actual
        );
        self.tokens.next(); //eat
    }

    pub fn parse_program(&mut self) -> Result<Exp, ErrorType> {
        return self.expr();
    }

    fn expr(&mut self) -> Result<Exp, ErrorType> {
        return match self.tokens.peek().unwrap() {
            TokenType::EOF => Ok(Exp::UnitExp),
            _ => self.relational_expr(),
        };
    }
    fn relational_expr(&mut self) -> Result<Exp, ErrorType> {
        let mut left = self.additive_expr()?;
        /*loop {
            let op = match self.tokens.peek() {
                Some(TokenType::EQEQ) => {
                    self.eat(&TokenType::EQEQ);
                    BinOp::EqualBinOp
                }
                Some(TokenType::NEQ) => {
                    self.eat(&TokenType::NEQ);
                    BinOp::NotEqualBinOp
                }
                Some(TokenType::GE) => {
                    self.eat(&TokenType::GE);
                    BinOp::GreaterThanBinOp
                }
                Some(TokenType::GEQ) => {
                    self.eat(&TokenType::GEQ);
                    BinOp::GreaterThanEqualBinOp
                }
                Some(TokenType::LE) => {
                    self.eat(&TokenType::LE);
                    BinOp::LessThanBinOp
                }
                Some(TokenType::LEQ) => {
                    self.eat(&TokenType::LEQ);
                    BinOp::LessThenEqualBinOp
                }
                _ => break,
            };

            let right = self.additive_expr()?;
            left = Exp::BinOpExp(Box::new(left), op, Box::new(right))
        }*/
        // No relational stuff for now
        Ok(left)
    }
    fn additive_expr(&mut self) -> Result<Exp, ErrorType> {
        let mut left = self.multiplicative_expr()?;

        loop {
            let op = match self.tokens.peek() {
                Some(TokenType::PLUS) => {
                    self.eat(&TokenType::PLUS);
                    BinOp::PlusBinOp
                }
                Some(TokenType::MINUS) => {
                    self.eat(&TokenType::MINUS);
                    BinOp::MinusBinOp
                }
                _ => break,
            };

            let right = self.multiplicative_expr()?;

            left = Exp::BinOpExp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }
    fn multiplicative_expr(&mut self) -> Result<Exp, ErrorType> {
        let mut left = self.primary_expr()?; // unary when implemented, and let unary call primary if non unary

        loop {
            let op = match self.tokens.peek() {
                Some(TokenType::TIMES) => {
                    self.eat(&TokenType::TIMES);
                    BinOp::TimesBinOp
                }
                Some(TokenType::DIVIDE) => {
                    self.eat(&TokenType::DIVIDE);
                    BinOp::DivideBinOp
                }
                _ => break,
            };

            let right = Box::new(self.primary_expr()?);
            left = Exp::BinOpExp {
                left: Box::new(left),
                op,
                right,
            };
        }
        Ok(left)
    }

    fn seq_expr(&mut self) -> Result<Exp, ErrorType> {
        self.eat(&TokenType::OpenBrack);

        let mut expressions = vec![self.expr()?];

        // "While self.tokens.peek destructs into SEMICOLON, do the following..."
        while let TokenType::SEMICOLON = self.tokens.peek().unwrap() {
            self.eat(&TokenType::SEMICOLON);

            expressions.push(self.expr()?);
        }

        self.eat(&TokenType::CloseBrack);

        Ok(Exp::SeqExp { expr: expressions })
    }

    // Maybe fun decls shouldn't be here - find out when testing later...
    fn primary_expr(&mut self) -> Result<Exp, ErrorType> {
        return match self.tokens.peek().unwrap() {
            TokenType::IntLit(v) => self.int_lit(),
            //TokenType::LET => self.let_expr(),
            //TokenType::Identifier(id) => self.var_expr(), // Probably also add call exp to this one later...
            TokenType::OpenBrack => self.seq_expr(),
            //TokenType::TRUE | TokenType::FALSE => self.bool_exp(),
            //TokenType::IF => self.if_expr(),
            //TokenType::WHILE => self.while_expr(),
            //TokenType::OpenParen => self.tuple_expr(),
            //TokenType::FUNDEC => self.fun_dec(),
            _ => Err("Test"),
        };
    }

    fn int_lit(&mut self) -> Result<Exp, ErrorType> {
        let next = self.tokens.peek().expect("Could not read int");
        let mut value = 0;

        let intlit = match next {
            TokenType::IntLit(val) => {
                value = val.clone();
                Ok(Exp::IntExp { value })
            }
            _ => Err("Error"),
        };
        self.eat(&TokenType::IntLit(value));
        return intlit;
    }

    pub fn print_result(&mut self, program: &Exp) {
        println!("AST: \n{}", program);
    }


}
