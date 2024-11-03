use std::fmt::Display;

use anyhow::{ensure, Result};

#[derive(Clone)]
pub struct Token {
    pub line_index: usize,
    pub word_index: usize,
    pub orign_string: String,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{} \"{}\"",
            self.line_index, self.word_index, self.orign_string
        )
    }
}

pub struct TokenStream {
    toks: Vec<Token>,
    toks_index: usize,
}

impl TokenStream {
    pub fn from_vec(toks: Vec<Token>) -> Self {
        Self {
            toks,
            toks_index: 0,
        }
    }

    pub fn next(&mut self) -> Result<(Token, usize)> {
        ensure!(
            self.toks.len() > self.toks_index,
            "Unexpected end of tokens stream"
        );
        self.toks_index = self.toks_index + 1;
        Ok((self.toks[self.toks_index - 1].clone(), self.toks_index - 1))
    }

    pub fn peek(&mut self) -> Result<(Token, usize)> {
        ensure!(
            self.toks.len() > self.toks_index,
            "Unexpected end of tokens stream"
        );
        Ok((self.toks[self.toks_index].clone(), self.toks_index))
    }

    pub fn get(&mut self, i: usize) -> Result<(Token, usize)> {
        ensure!(
            self.toks.len() > i,
            "{} out of range 0..{}",
            i,
            self.toks.len()
        );
        Ok((self.toks[i].clone(), i))
    }
}

pub trait FromTokenStream {
    fn from_toks(toks: &mut TokenStream) -> anyhow::Result<Self>
    where
        Self: Sized;
}
