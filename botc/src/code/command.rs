use anyhow::{bail, Error};

use super::{Dir, Lable, Reg, RwReg, Val};
use crate::{
    decl_tokens_enum,
    token::{FromTokenStream, TokenStream},
};

pub(crate) const COMMAND_REGEX: &str = "^[a-zA-Z]*$";

macro_rules! decl_command_enum {
    (PossibleArgs: ($($pargs:ident),*)
     Commands: $(($str_name:literal, $enum_entry:ident $(, $($args:ident),*)?)),*) => {

        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        pub enum CommandArg {
            $($pargs($pargs)),*
        }

        impl std::fmt::Display for CommandArg {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(CommandArg::$pargs(a) => write!(f, "{a}")),*,
                }
            }
        }

        decl_tokens_enum! {
            CommandWord,
            $(($str_name, $enum_entry)),*
        }

        #[derive(Debug)]
        pub struct Expr {
            pub cmd: CommandWord,
            pub args: Vec<CommandArg>
        }

        impl FromTokenStream for Expr {
            fn from_toks(toks: &mut TokenStream) -> anyhow::Result<Expr> {
                let cmd = CommandWord::from_toks(toks)?;
                let args = match cmd {
                    $(CommandWord::$enum_entry =>
                        vec![$($(CommandArg::$args(<$args>::from_toks(toks)?)),*)?]),*
                };
                Ok(Expr{cmd, args})
            }
        }

        #[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
        pub enum Command {
            $($enum_entry$(($($args),*))?),*
        }

        impl TryFrom<Expr> for Command {
            type Error = anyhow::Error;
            fn try_from(value: Expr) -> Result<Self, Error> {
                match value.cmd {
                    $(CommandWord::$enum_entry => {
                        let mut _iter = value.args.into_iter();
                        Ok(Command::$enum_entry$((
                            $(if let Some(CommandArg::$args(arg)) = _iter.next() {
                                arg
                            } else {
                                bail!("Unexpected argument type");
                            }),*
                        )),*)
                    }),*
                }
            }
        }

        impl TryFrom<Command> for Expr {
            type Error = anyhow::Error;
            fn try_from(value: Command) -> Result<Self, Error> {
                $(Command_to_Expr_convertor!{$enum_entry, value, $($($args),*)?})*
                Err(anyhow::Error::msg("Failed to convert Command to Expr"))
            }
        }
    };
}

macro_rules! Command_to_Expr_convertor {
    ($enum_entry:ident, $value:ident, ) => {
        if let Command::$enum_entry {} = $value {
            return Ok(Expr {
                cmd: CommandWord::$enum_entry,
                args: vec![],
            });
        }
    };
    ($enum_entry:ident, $value:ident, $args1:ident) => {
        if let Command::$enum_entry(a) = $value {
            return Ok(Expr {
                cmd: CommandWord::$enum_entry,
                args: vec![CommandArg::$args1(a)],
            });
        }
    };
    ($enum_entry:ident, $value:ident, $args1:ident, $args2:ident) => {
        if let Command::$enum_entry(a, b) = $value {
            return Ok(Expr {
                cmd: CommandWord::$enum_entry,
                args: vec![CommandArg::$args1(a), CommandArg::$args2(b)],
            });
        }
    };
}

decl_command_enum! {
    PossibleArgs:
        (Dir, Lable, Reg, RwReg, Val)
    Commands:
        ("nop",    Nop                  ),
        ("mov",    Mov,    Dir          ),
        ("rot",    Rot,    Dir          ),
        ("jmp",    Jmp,    Lable        ),
        ("jmg",    Jmg,    Lable        ),
        ("jnl",    Jnl,    Lable        ),
        ("jme",    Jme,    Lable        ),
        ("jne",    Jne,    Lable        ),
        ("jmf",    Jmf,    Lable        ),
        ("jnf",    Jnf,    Lable        ),
        ("jmb",    Jmb,    Lable        ),
        ("jnb",    Jnb,    Lable        ),
        ("jmc",    Jmc,    Lable        ),
        ("jnc",    Jnc,    Lable        ),
        ("jge",    Jge,    Lable        ),
        ("jle",    Jle,    Lable        ),
        ("chk",    Chk,    Dir          ),
        ("cmp",    Cmp,    Reg,   Reg   ),
        ("cmpv",   Cmpv,   Reg,   Val   ),
        ("split",  Split,  Dir,   Lable ),
        ("forc",   Forc,   Dir,   Lable ),
        ("bite",   Bite,   Dir          ),
        ("eatsun", Eatsun               ),
        ("absorb", Absorb               ),
        ("call",   Call,   Lable        ),
        ("ret",    Ret                  ),
        ("ld",   Load,   RwReg, Reg   ),
        ("ldv",  Loadv,  RwReg, Val   )
}
