pub mod code;
pub mod code_packer;
pub mod compiler;
pub(crate) mod token;

macro_rules! decl_tokens_enum {
    ($enum_name:ident, $(($str_name:literal, $enum_entry:ident)),*) => {
        #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, num_derive::FromPrimitive, Eq, PartialEq)]
        pub enum $enum_name {
            $($enum_entry),*
        }

        impl crate::token::FromTokenStream for $enum_name {
            fn from_toks(toks: &mut crate::token::TokenStream) -> anyhow::Result<$enum_name> {
                let (cmd_tok, _) = toks.next()?;
                Ok(match cmd_tok.orign_string.to_lowercase().as_str() {
                    $($str_name => $enum_name::$enum_entry),*,
                    _ => anyhow::bail!("Failed to parse {} as {}", cmd_tok, stringify!($enum_name))
                })
            }
        }

        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match *self {
                    $($enum_name::$enum_entry => write!(f, $str_name)),*,
                }
            }
        }
        
        impl rand::prelude::Distribution<$enum_name> for rand::distributions::Standard {
            fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> $enum_name {
                const ENUM_VARIANT_COUNTL: usize = [
                    $($enum_name::$enum_entry),*
                ].len();
                let i = rng.gen_range(0..ENUM_VARIANT_COUNTL);
                num_traits::FromPrimitive::from_usize(i).unwrap()
            }
        }
        
        impl std::default::Default for $enum_name {
            fn default() -> Self {
                num_traits::FromPrimitive::from_usize(0).unwrap()
            }
        }
    };
}
pub(crate) use decl_tokens_enum;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn compile(input: String) -> String {
    match compiler::compile(input) {
        Ok(code) => {
            match code_packer::to_b32(&code) {
                Ok(res) => format!("Compiled successfully.\nBot Code:\n{}", res),
                Err(err) => format!("Failed to encode compiled code: {err}")
            }
        },
        Err(e) => {
            let mut err = String::new();
            err.push_str(format!("{e}").as_str());
            for e in e.chain() {
                err.push_str(format!("\n    {e}").as_str());
            }
            err
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn decompile(input: String) -> String {
    match code_packer::from_b32(&input) {
        Ok(code) => {
            compiler::decompile(code).iter().fold(String::new(), |mut acc, cmd| {
                acc.push_str(format!("{}", cmd).as_str());
                acc.push_str("\n");
                acc
            })
        },
        Err(err) => format!("Failed to decode code: {err}"),
    }
}
