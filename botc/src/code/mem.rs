use anyhow::{bail, ensure};

pub const BOT_MEM_SIZE: usize = 10;

pub type Mem = u64;
impl crate::token::FromTokenStream for Mem {
    fn from_toks(toks: &mut crate::token::TokenStream) -> anyhow::Result<Mem> {
        use anyhow::Context;
        let (val_tok, _) = toks.next()?;

        let string = val_tok.orign_string.clone();
        ensure!(
            string.len() > 2,
            format!("Failed to parse {} as memory address", string)
        );
        let mut chars: Vec<_> = string.chars().collect();
        ensure!(
            chars.remove(0) == '[' && chars.pop().unwrap() == ']',
            format!("Failed to parse {} as memory address", string)
        );
        let addr = chars.into_iter().collect::<String>().parse::<Mem>()
            .context(format!("Failed to parse {} as number", val_tok))?;
        if addr as usize >= BOT_MEM_SIZE {
            bail!(format!(
                "Memory address out of range: {} not in 0..{}",
                addr, BOT_MEM_SIZE
            ));
        }
        Ok(addr)
    }
}
