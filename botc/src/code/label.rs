pub const LABEL_REGEX: &str = "^[a-zA-Z_][a-zA-Z_0-9]*:$";

pub type Label = usize;
impl crate::token::FromTokenStream for Label {
    fn from_toks(toks: &mut crate::token::TokenStream) -> anyhow::Result<Label> {
        let (_, index) = toks.next()?;
        Ok(index)
    }
}
