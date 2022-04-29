use crate::ast::bin_op_exp_from_token;
use crate::tokens::bin_op_precedence;
use crate::{ast::BinOp, Exp};
use core::panic;
use std::vec::IntoIter;
use std::{iter::Peekable};
//TODO: Add sequence to grammar?
use crate::{print_tokens, Token};

type TokenIter = Peekable<IntoIter<Token>>;
type ErrorType = &'static str;
//TODO: Probably result types instead - also nice way of parsing.
// Also: Eat tokens when they're peeked, in functions
// Implement "eat" function that throws error if ate token is not expected.


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

    pub fn parse_program(&mut self) -> Result<Exp, ErrorType> {
        let result =  self.parse_exp()?;
        println!("Result: {}\n", result);
        match self.tokens.peek() {
            Some(t) => println!("Next token is {}", t),
            None => println!("No more tokens!")
        };


        Ok(result)
    }

    fn parse_int_lit(&mut self) -> Result<Exp, ErrorType> {
        let lit = match self.tokens.peek().unwrap() {
            Token::IntLit(v) => {
                Ok(Exp::IntExp(v.clone()))
            },
            _ => Err("Unable to parse expression"),
        };
        self.tokens.next(); // eat
        lit
    }

    // Not tested
    fn parse_let_exp(&mut self) -> Result<Exp, ErrorType> {
        self.tokens.next(); // consume "LET" token
        let id = match self.tokens.next().unwrap() {
            Token::Identifier(x) => x,
            _ => panic!("Token after LET was not an identifier")
        };
        self.tokens.next(); //eat "=" token
        let val = self.parse_exp().unwrap();
        println!("Let Exp is parsed as: {}", val);
        //println!("Next after LetExp parsed is {}", self.tokens.peek().unwrap());
        Ok(Exp::LetExp(id, Box::new(val)))
    }

    // ##### TESTING STUFF #####
    fn parse_exp(&mut self) -> Result<Exp, ErrorType> {
        let left = self.parse_base_exp()?;
        println!("Base is {}", left);


        self.parse_exp_left_to_right(left, 0)
    }

    // Base exps is a non-binop expression
    fn parse_base_exp(&mut self) -> Result<Exp, ErrorType> {
        let base = match self.tokens.peek() {
            Some(Token::IntLit(_)) => {
                self.parse_int_lit()
            }, // int lit already eats
            Some(Token::LET) => {
                self.parse_let_exp()
            },
            _ => Err("Error"),
        };
        base
    }

    // TODO: Allow base exps to be parsed such as LetExp: Break out of parse_left_to_right if the precedence if below current?
    // Break out = just return left exp



    // ERROR IN LOGIC HERE - Right now next token should be ; but it says PLUS...
    // Maybe the ; gets eaten here by mistake...
    // Even with ; in let.tsl, next is EOF...


    //TODO: Restructure this, to possibly only parse binops...
    fn parse_exp_left_to_right(
        &mut self,
        mut left: Exp,
        min_precedence: i32,
    ) -> Result<Exp, ErrorType> {

        // If no token is present, return left. Is some is present, proceed
        match self.tokens.peek() {
            Some(t) => {
                println!("From LTR: {}\n", self.tokens.peek().unwrap());
                ()
            },
            None => return Ok(left)
        };

        let mut lookahead = self.tokens.next().unwrap();
        let pred = bin_op_precedence(&lookahead);
        while(bin_op_precedence(&lookahead) >= min_precedence) {
            println!("Entered loop");
            let op = lookahead;
            let mut right = self.parse_base_exp()?;

            lookahead = self.tokens.next().unwrap();
            while(bin_op_precedence(&lookahead) > bin_op_precedence(&op)) {
                right = self.parse_exp_left_to_right(right, bin_op_precedence(&op) + 1)?;
                lookahead = self.tokens.next().unwrap();
            }
            left = Exp::BinOpExp(Box::new(left), bin_op_exp_from_token(&op), Box::new(right));
        }
        Ok(left)
    }




    // ##### TESTING STUFF #####

    /* JAVA PARSER WORK
        //https://en.wikipedia.org/wiki/Operator-precedence_parser

        private Exp parseExpLeftToRight(Exp base, int minPrecedence) throws IOException {
            TokenType lookahead = nextToken.getType();
            while (precedence.getOrDefault(lookahead, -1) >= minPrecedence ) {
                TokenType op = nextToken.getType();
                eat(op);
                Exp right = parseBaseExp();
                lookahead = nextToken.getType();
                while (precedence.getOrDefault(lookahead, -1) > precedence.get(op)) { //If right associative then >= else just >: When adding ^ to language, get a "isRightAssoc()" and make a || case with >=
                     right = parseExpLeftToRight(right, precedence.get(op) + 1);
                     lookahead = nextToken.getType();
                }
                base = new BinOpExp(base, op, right);
            }
            return base;
        }


    */
}

/*
#[macro_export]
macro_rules! peek_op_or_err {
    ($self:ident) => {
        match $self.tokens.peek() {
            Some(Token::PLUS) => BinOp::PlusBinOp,
            _ => return Err("Expected a symbol"),
        }
    };
}
use crate::peek_literal_or_err;

#[macro_export]
macro_rules! peek_literal_or_err {
    ($self:ident) => {
        match $self.tokens.peek() {
            Some(Token::IntLit(value)) => Exp::IntExp(value.clone()),
            _ => return Err("Expected a literal"),
        }
    };
}

*/

