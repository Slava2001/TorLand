pub const LABLE_REGEX: &str = "^[a-zA-Z_][a-zA-Z_0-9]*:$";

pub type Lable = usize;
impl crate::token::FromTokenStream for Lable {
    fn from_toks(toks: &mut crate::token::TokenStream) -> anyhow::Result<Lable> {
        let (_, index) = toks.next()?;
        Ok(index)
    }
}
