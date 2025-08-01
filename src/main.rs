use std::fs;
use std::env;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Literal(char),
    CharClass(Vec<char>),
    Alternation,
    Concatenation,
    ZeroOrMore,
    OneOrMore,
    ZeroOrOne,
    LeftParen,
    RightParen,
}

#[derive(Debug)]
pub enum Error {
    Parse,
    File,
}

pub struct Tokenizer {
    chars: Vec<char>,
    pos: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Self { chars: input.chars().collect(), pos: 0 }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();
        
        while self.pos < self.chars.len() {
            match self.chars[self.pos] {
                '\\' => {
                    self.pos += 1;
                    if self.pos < self.chars.len() {
                        tokens.push(Token::Literal(self.chars[self.pos]));
                    }
                }
                '[' => {
                    self.pos += 1;
                    let mut chars = Vec::new();
                    while self.pos < self.chars.len() && self.chars[self.pos] != ']' {
                        chars.push(self.chars[self.pos]);
                        self.pos += 1;
                    }
                    tokens.push(Token::CharClass(chars));
                }
                '|' => tokens.push(Token::Alternation),
                '*' => tokens.push(Token::ZeroOrMore),
                '+' => tokens.push(Token::OneOrMore),
                '?' => tokens.push(Token::ZeroOrOne),
                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                c => tokens.push(Token::Literal(c)),
            }
            self.pos += 1;
        }

        // Insert concatenation
        let mut result = Vec::new();
        for i in 0..tokens.len() {
            if i > 0 && self.needs_concat(&tokens[i-1], &tokens[i]) {
                result.push(Token::Concatenation);
            }
            result.push(tokens[i].clone());
        }
        
        Ok(result)
    }

    fn needs_concat(&self, prev: &Token, curr: &Token) -> bool {
        matches!(prev, Token::Literal(_) | Token::CharClass(_) | Token::ZeroOrMore | Token::OneOrMore | Token::ZeroOrOne | Token::RightParen) &&
        matches!(curr, Token::Literal(_) | Token::CharClass(_) | Token::LeftParen)
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Token>, Error> {
        let mut output = Vec::new();
        let mut stack = Vec::new();

        for token in &self.tokens {
            match token {
                Token::Literal(_) | Token::CharClass(_) => output.push(token.clone()),
                Token::ZeroOrMore | Token::OneOrMore | Token::ZeroOrOne => output.push(token.clone()),
                Token::LeftParen => stack.push(token.clone()),
                Token::RightParen => {
                    while let Some(op) = stack.pop() {
                        if matches!(op, Token::LeftParen) { break; }
                        output.push(op);
                    }
                }
                Token::Concatenation => {
                    while let Some(top) = stack.last() {
                        if matches!(top, Token::LeftParen) { break; }
                        output.push(stack.pop().unwrap());
                    }
                    stack.push(token.clone());
                }
                Token::Alternation => {
                    while let Some(top) = stack.last() {
                        if matches!(top, Token::LeftParen) { break; }
                        output.push(stack.pop().unwrap());
                    }
                    stack.push(token.clone());
                }
            }
        }

        while let Some(op) = stack.pop() {
            output.push(op);
        }

        Ok(output)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file.txt>", args[0]);
        return;
    }

    let content = fs::read_to_string(&args[1]).unwrap();
    
    for line in content.lines() {
        if line.trim().is_empty() { continue; }
        
        println!("Input: {}", line);
        
        let mut tokenizer = Tokenizer::new(line);
        let tokens = tokenizer.tokenize().unwrap();
        
        let mut parser = Parser::new(tokens);
        let postfix = parser.parse().unwrap();
        
        println!("Postfix: {:?}", postfix);
        println!();
    }
}