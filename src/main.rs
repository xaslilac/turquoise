#![feature(never_type)]

use std::env::args;
use std::fs;
use std::io;
use std::path::PathBuf;

/// TODO: Resumable parsing
/// Tokenizer should support resuming at a given position and a given state, as well as
/// early exiting, if the rest of the input is known/the result can be predicted/memoized.

#[derive(Debug)]
enum Token {
    Name(String),
    OpenParen,
    CloseParen,
    DoubleQuote,
    // SingleQuote,
    Any(String),
}

#[derive(Debug)]
enum StringError {
    NoString,
    NeverEndingString,
    InvalidEscape,
}

struct ParserReader {
    pub src: String,
    pub pos: usize,
}

impl ParserReader {
    fn from(src: String) -> ParserReader {
        ParserReader { src, pos: 0 }
    }

    fn eat_whitespace(&mut self) {
        while self.src[self.pos..]
            .chars()
            .next()
            .filter(|c| c.is_whitespace())
            .is_some()
        {
            self.pos += 1;
        }
    }

    fn eat_string(&mut self) -> Result<String, StringError> {
        let mut chars = self.src[self.pos..].chars();
        let mut expect_next_char_to_be_quote = false;

        if chars.next() == Some('"') {
            self.pos += 1;
            let start = self.pos;

            while let Some(c) = chars.next() {
                match c {
                    '"' => {
                        if !expect_next_char_to_be_quote {
                            return Ok(self.src[start..self.pos].to_string());
                        }
                    }
                    '\\' => {
                        if !expect_next_char_to_be_quote {
                            expect_next_char_to_be_quote = true;
                        }
                    }
                    _ => {
                        if expect_next_char_to_be_quote {
                            return Err(StringError::InvalidEscape);
                        }
                    }
                }

                self.pos += 1;
            }

            Err(StringError::NeverEndingString)
        } else {
            Err(StringError::NoString)
        }
    }

    fn expect(&mut self, next: &str) -> Option<()> {
        let x = &self.src[self.pos..self.pos + next.len()];
        if x == next {
            self.pos += next.len();
            Some(())
        } else {
            None
        }
    }
}

fn parse(mut code: ParserReader) -> Vec<Token> {
    let mut tokens = Vec::new();
    code.eat_whitespace();

    if code.expect("print").is_some() {
        tokens.push(Token::Name("print".to_string()));
    }

    if code.expect("(").is_some() {
        tokens.push(Token::OpenParen);
    }

    let x = code.eat_string().unwrap();
    tokens.push(Token::DoubleQuote);
    tokens.push(Token::Any(x));
    tokens.push(Token::DoubleQuote);

    if code.expect(")").is_some() {
        tokens.push(Token::CloseParen);
    }

    dbg!(tokens)
}

fn main() -> Result<(), io::Error> {
    let args = args().skip(1).collect::<Vec<_>>();

    if args.len() != 1 {
        println!("Usage: take <file>");
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Wrong number of arguments",
        ));
    }

    let package_path = PathBuf::from(&args[0]);
    let package_files = fs::read_dir(&package_path).expect("Could not read file");
    for file in package_files.filter_map(|de| de.ok()) {
        let file_name = {
            let mut pb = package_path.clone();
            pb.push(file.file_name());
            pb
        };

        if file.file_name() != "index.q" {
            continue;
        }

        let file_bytes = fs::read(file_name).expect("Failed to read file");
        let code = String::from_utf8(file_bytes).expect("Could not parse file as UTF-8");
        parse(ParserReader::from(code));
    }

    Ok(())
}
