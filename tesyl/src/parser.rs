use crate::ast::bin_op_exp_from_token;
use crate::tokens::bin_op_precedence;
use crate::{ast::BinOp, Exp};
use std::vec::IntoIter;
use std::{collections::binary_heap::Iter, iter::Peekable};
//TODO: Add sequence to grammar?
use crate::{print_tokens, Token};

type TokenIter = Peekable<IntoIter<Token>>;
type ErrorType = &'static str;
//TODO: Probably result types instead - also nice way of parsing.

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
        //self.parse_bin_op() // can only handle bin op for now
        let result =  self.parse_exp()?;
        Ok(result)
    }

    fn parse_bin_op(&mut self) -> Exp {
        let left = self.parse_int_lit().unwrap(); // can only handle simple bin ops
        self.tokens.next(); // eat peeked token
        let op = match self.tokens.peek() {
            Some(Token::PLUS) => BinOp::PlusBinOp,
            _ => BinOp::Undefined,
        };
        self.tokens.next();
        let right = self.parse_int_lit().unwrap();
        self.tokens.next();
        Exp::BinOpExp(Box::new(left), op, Box::new(right))
    }

    fn parse_int_lit(&mut self) -> Result<Exp, ErrorType> {
        match self.tokens.peek().unwrap() {
            Token::IntLit(v) => Ok(Exp::IntExp(v.clone())),
            _ => Err("Unable to parse expression"),
        }
    }

    // ##### TESTING STUFF #####
    fn parse_exp(&mut self) -> Result<Exp, ErrorType> {
        let mut left = self.parse_base_exp()?;
        self.parse_exp_left_to_right(left, 0)
    }

    fn parse_base_exp(&mut self) -> Result<Exp, ErrorType> {
        match self.tokens.peek() {
            Some(Token::IntLit(_)) => self.parse_int_lit(),
            _ => Err("Error"),
        }
    }

    fn parse_exp_left_to_right(
        &mut self,
        mut left: Exp,
        min_precedence: i32,
    ) -> Result<Exp, ErrorType> {
        let mut lookahead = self.tokens.next().unwrap();
        while(bin_op_precedence(&lookahead) >= min_precedence) {
            let op = self.tokens.next().unwrap();
            let mut right = self.parse_base_exp()?;
            lookahead = self.tokens.next().unwrap();
            while(bin_op_precedence(&lookahead) > bin_op_precedence(&op)) {
                right = self.parse_exp_left_to_right(right, bin_op_precedence(&op) + 1)?;
                lookahead = self.tokens.next().unwrap();
            }
            left = Exp::BinOpExp(Box::new(left), bin_op_exp_from_token(&op), Box::new(right));
        }

        Ok(Exp::Undefined)
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
