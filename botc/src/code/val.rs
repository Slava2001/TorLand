pub type Val = isize;
impl crate::token::FromTokenStream for Val {
    fn from_toks(toks: &mut crate::token::TokenStream) -> anyhow::Result<Val> {
        use anyhow::Context;
        let (val_tok, _) = toks.next()?;
        Ok(val_tok
            .orign_string
            .parse::<Val>()
            .context(format!("Failed to parse {} as number", val_tok))?)
    }
}
