use anyhow::{bail, ensure, Context, Result};
use regex::Regex;
use std::collections::HashMap;

use crate::{
    code::{
        command::{Command, CommandArg, CommandWord, Expr, COMMAND_REGEX},
        Lable, LABLE_REGEX,
    },
    token::{FromTokenStream, Token, TokenStream},
};

pub fn compile(code: String) -> Result<Vec<Command>> {
    let tokens = Compiler::preproc(code).context("Failed to preproc")?;
    Compiler::new()
        .translate(tokens)
        .context("Failed to translate")
}

pub fn decompile(code: Vec<Command>) -> Vec<String> {
    let mut req_lables: Vec<(usize, String)> = Vec::new();
    let mut res: Vec<String> = code
        .into_iter()
        .map(|c| {
            let expr = TryInto::<Expr>::try_into(c).unwrap();
            let mut res: String = format!("{}", expr.cmd);
            for a in expr.args {
                if let CommandArg::Lable(index) = a {
                    res.push_str(format!(" lable_{a}").as_str());
                    req_lables.push((index, format!("lable_{a}:")));
                } else if let CommandArg::Mem(addr) = a {
                    res.push_str(format!(" [{addr}]").as_str());
                } else {
                    res.push_str(format!(" {a}").as_str());
                }
            }
            res
        })
        .collect();
    req_lables.sort_by(|(i1, _), (i2, _)| i2.cmp(i1));
    req_lables.dedup_by(|(i1, _), (i2, _)| i1 == i2);
    for (i, l) in req_lables {
        res.insert(i, l);
    }
    res
}

const DIRECTIVE_REGEX: &str = "^#[a-zA-Z_]*$";
const LINE_COMMENTS_START: &'static str = "//";

struct Compiler {
    exist_lables: HashMap<String, usize>,
    commands: Vec<Expr>,
    gen_len: isize,
    mem_size: isize,
}

impl Compiler {
    fn new() -> Self {
        Self {
            exist_lables: HashMap::new(),
            commands: Vec::new(),
            gen_len: -1,
            mem_size: -1,
        }
    }

    fn preproc(code: String) -> Result<TokenStream> {
        // code preparation: removing comments, case alignment, splitting into tokens and numbering
        Ok(TokenStream::from_vec(
            code.lines()
                .map(|mut l| {
                    // removing comments
                    if let Some(i) = l.find(LINE_COMMENTS_START) {
                        l = &l[..i];
                    }
                    // splitting into tokens
                    l.split_whitespace()
                        .map(|t| t.trim().to_string())
                        .filter(|t| !t.is_empty())
                        .collect::<Vec<String>>()
                })
                // numbering lines
                .zip(1..code.lines().count() + 1)
                .filter(|(ts, _)| !ts.is_empty())
                .map(|(ts, ln)| {
                    ts.iter()
                        // numbering words
                        .zip(1..ts.len() + 1)
                        .map(|(t, wn)| Token {
                            line_index: ln,
                            word_index: wn,
                            orign_string: t.to_string(),
                        })
                        .collect()
                })
                .fold(Vec::new(), |mut acc, mut t| {
                    acc.append(&mut t);
                    acc
                }),
        ))
    }

    fn translate(mut self, mut toks: TokenStream) -> Result<Vec<Command>> {
        // parse tokens
        while let Ok((token, _)) = toks.peek() {
            match token.orign_string.as_str() {
                s if Regex::new(DIRECTIVE_REGEX)?.is_match(s) => self
                    .parse_directive(&mut toks)
                    .context("Failed to translate directive")?,
                s if Regex::new(COMMAND_REGEX)?.is_match(s) => self
                    .parse_command(&mut toks)
                    .context("Failed to translate command")?,
                s if Regex::new(LABLE_REGEX)?.is_match(s) => self
                    .parse_lable(&mut toks)
                    .context("Failed to translate lable")?,
                _ => bail!("Unexpected token {}", token),
            };
        }

        // checking genome length
        // if the length is specified then the genome is added to the specified length
        if self.gen_len < 0 {
            self.gen_len = self.commands.len() as isize;
        } else {
            ensure!(
                self.gen_len as usize >= self.commands.len(),
                "The generated code does not fit into the specified size. Code len: {}, expect: {}",
                self.commands.len(),
                self.gen_len
            );
            for _ in self.commands.len()..(self.gen_len as usize) {
                self.commands.push(Expr {
                    cmd: CommandWord::Nop,
                    args: Vec::new(),
                });
            }
        }

        // mem size check
        if self.mem_size >= 0 {
            if let Some(addr) = self
                .commands
                .iter()
                .map(|c| c.args.iter())
                .flatten()
                .find(|a| {
                    if let CommandArg::Mem(m) = a {
                        *m >= (self.mem_size as u64)
                    } else {
                        false
                    }
                })
            {
                bail!(
                    "The memory address argument of one of the commands is outside the \
                     specified limits: address [{}] not in 0..{}",
                    addr,
                    self.mem_size
                );
            }
        }

        // label resoling
        for c in self.commands.iter_mut() {
            for a in c.args.iter_mut() {
                if let CommandArg::Lable(token_index) = a {
                    let (lable_tok, _) = toks.get(*token_index as usize)?;
                    let lable_pos = self
                        .exist_lables
                        .get(&lable_tok.orign_string.to_lowercase())
                        .context(format!("Label {} not found", lable_tok))?;
                    let lable_pos = *lable_pos % self.gen_len as usize;
                    *a = CommandArg::Lable(lable_pos as Lable);
                }
            }
        }

        // conversion to command vector
        self.commands
            .into_iter()
            .map(|c| Ok(c.try_into()?))
            .collect::<Result<Vec<Command>>>()
    }

    fn parse_directive(&mut self, toks: &mut TokenStream) -> Result<()> {
        let (directive, _) = toks.next()?;
        match &directive.orign_string.to_lowercase().as_str()[1..] {
            "len" => {
                ensure!(
                    self.gen_len == -1,
                    "len directive redefined at {}",
                    directive
                );
                let (len, _) = toks.next()?;
                self.gen_len = len
                    .orign_string
                    .parse::<usize>()
                    .context(format!("Failed to parse {} as usize", len))?
                    as isize;
            }
            "mem_size" => {
                ensure!(
                    self.mem_size == -1,
                    "mem_size directive redefined at {}",
                    directive
                );
                let (size, _) = toks.next()?;
                self.mem_size = size
                    .orign_string
                    .parse::<usize>()
                    .context(format!("Failed to parse {} as usize", size))?
                    as isize;
            }
            _ => bail!("Unexpected directive {}", directive),
        }
        Ok(())
    }

    fn parse_lable(&mut self, toks: &mut TokenStream) -> Result<()> {
        let (lable, _) = toks.next()?;
        ensure!(
            self.exist_lables
                .insert(
                    lable.orign_string.as_str()[..lable.orign_string.len() - 1].to_lowercase(),
                    self.commands.len()
                )
                .is_none(),
            "label override {}",
            lable
        );
        Ok(())
    }

    fn parse_command(&mut self, toks: &mut TokenStream) -> Result<()> {
        Ok(self.commands.push(Expr::from_toks(toks)?))
    }
}
