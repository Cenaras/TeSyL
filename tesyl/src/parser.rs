use crate::{ast::BinOp, Exp};
use core::panic;
use std::iter::Peekable;
use std::vec::IntoIter;
//TODO: Add sequence to grammar?
use crate::{print_tokens, Token};

type TokenIter = Peekable<IntoIter<Token>>;
type ErrorType = &'static str;

/*
    Right now, additive is top level
    It can call mult, which can call primary, i.e. let and int_lit

*/

// Useful reference implementation might be:
// https://github.com/antoyo/tiger-rs/blob/master/tiger/src/parser.rs

pub struct Parser {
    tokens: TokenIter,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        // Debugging
        print_tokens(&tokens);

        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    fn eat(&mut self, expected: &Token) {
        if expected == self.tokens.peek().expect("No token present to eat") {
            self.tokens.next();
            //println!("Ate");
        }
    }

    pub fn parse_program(&mut self) -> Result<Exp, ErrorType> {
        return self.expr();
    }

    // Placeholder for now.
    fn expr(&mut self) -> Result<Exp, ErrorType> {
        return self.additive_expr();
        //return self.primary_expr();
    }

    fn primary_expr(&mut self) -> Result<Exp, ErrorType> {
        return match self.tokens.peek().unwrap() {
            Token::IntLit(v) => self.int_lit(),
            Token::LET => self.let_expr(),
            Token::Identifier(id) => self.var_expr(), // Probably also add call exp to this one later...
            Token::OpenParen => self.seq_expr(),
            Token::TRUE | Token::FALSE => self.bool_exp(),
            _ => Err("Test"),
        };
    }

    // Not tested - maybe not in primary exp?
    fn bool_exp(&mut self) -> Result<Exp, ErrorType> {
        let val = match self.tokens.peek().unwrap() {
            Token::TRUE => true,
            Token::FALSE => false,
            _ => panic!("Could not parse boolean expression"),
        };
        Ok(Exp::BoolExp(val))
    }

    fn var_expr(&mut self) -> Result<Exp, ErrorType> {
        let id = self.identifier();
        let id_temp = id.clone();
        self.eat(&Token::Identifier(id_temp));
        Ok(Exp::VarExp(id))
    }

    fn seq_expr(&mut self) -> Result<Exp, ErrorType> {
        self.eat(&Token::OpenParen);

        let mut expressions = vec![self.expr()?];

        // "While self.tokens.peek destructs into SEMICOLON, do the following..."
        while let Token::SEMICOLON = self.tokens.peek().unwrap() {
            self.eat(&Token::SEMICOLON);
            println!("Ate!\n");
            println!("Next token is: {}", self.tokens.peek().unwrap());
            let test = self.expr()?;
            println!("Current contents: {:?}", expressions);
            expressions.push(test);
        }

        self.eat(&Token::CloseParen);
        Ok(Exp::SeqExp(expressions))
    }

    fn let_expr(&mut self) -> Result<Exp, ErrorType> {
        self.eat(&Token::LET);

        let id = self.identifier();

        self.eat(&Token::Identifier(id.clone())); //TODO: Handle in identifier case
        self.eat(&Token::EQUAL);

        let expr = self.expr().unwrap();

        Ok(Exp::LetExp(id, Box::new(expr)))
    }

    fn identifier(&mut self) -> String {
        let next = self.tokens.peek();
        let id = match next {
            Some(Token::Identifier(x)) => x,
            _ => {
                println!("Token is {}", next.unwrap());
                panic!("Could not parse identifier");
            }
        };
        id.to_string()
    }

    fn int_lit(&mut self) -> Result<Exp, ErrorType> {
        let next = self.tokens.peek().expect("Could not read int");
        let mut value = 0;

        let intlit = match next {
            Token::IntLit(val) => {
                value = val.clone();
                Ok(Exp::IntExp(val.clone()))
            }
            _ => Err("Error"),
        };
        self.eat(&Token::IntLit(value));
        return intlit;
    }

    fn additive_expr(&mut self) -> Result<Exp, ErrorType> {
        let mut left = self.multiplicative_expr()?;

        loop {
            let op = match self.tokens.peek() {
                Some(Token::PLUS) => {
                    self.eat(&Token::PLUS);
                    BinOp::PlusBinOp
                }
                Some(Token::MINUS) => {
                    self.eat(&Token::MINUS);
                    BinOp::MinusBinOp
                }
                _ => break,
            };

            let right = self.multiplicative_expr()?;
            left = Exp::BinOpExp(Box::new(left), op, Box::new(right))
        }
        Ok(left)
    }

    // Takes primary exps or unary

    // Either parse two expressions as a multiplicative expression and combine into one, or simply parse a primary expression if not multiplicative.
    fn multiplicative_expr(&mut self) -> Result<Exp, ErrorType> {
        let mut left = self.primary_expr()?; // unary when implemented, and let unary call primary if non unary

        loop {
            let op = match self.tokens.peek() {
                Some(Token::TIMES) => {
                    self.eat(&Token::TIMES);
                    BinOp::TimesBinOp
                }
                Some(Token::DIVIDE) => {
                    self.eat(&Token::DIVIDE);
                    BinOp::DivideBinOp
                }
                _ => break,
            };

            let right = Box::new(self.primary_expr()?);
            left = Exp::BinOpExp(Box::new(left), op, right)
        }
        Ok(left)
    }
}
