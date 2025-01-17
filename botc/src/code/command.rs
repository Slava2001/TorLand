use anyhow::{bail, Error};

use super::{Dir, Lable, Mem, Reg, RwReg, Val};
use crate::{
    decl_tokens_enum,
    token::{FromTokenStream, TokenStream},
};

pub(crate) const COMMAND_REGEX: &str = "^[a-zA-Z]*$";

macro_rules! decl_command_enum {
    (PossibleArgs: ($($pargs:ident),*)
     Commands: $(($str_name:literal, $enum_entry:ident $(, $($args:ident),*)?)),*) => {

        #[derive(Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
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

        #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq)]
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

        impl Command {
            pub fn rand<R: rand::Rng + ?Sized>(rng: &mut R, len: usize, max_val: isize) -> Command {
                let cmd: CommandWord = rng.gen();
                Command::try_from(
                    match cmd {
                        $(
                            CommandWord::$enum_entry => {
                                #[allow(unused_mut)]
                                let mut args: Vec<CommandArg> = Vec::new();
                                $($(args.push({
                                    let arg = CommandArg::$args(rng.gen());
                                    match arg {
                                        CommandArg::Lable(l) => CommandArg::Lable(l % len),
                                        CommandArg::Val(v) => CommandArg::Val(v % max_val),
                                        arg => arg
                                    }
                                });)*)*
                                Expr {
                                    cmd: CommandWord::$enum_entry,
                                    args
                                }
                            }
                        )*
                    }
                ).unwrap()
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
        (Dir, Lable, Reg, RwReg, Val, Mem)
    Commands:
        ("nop",    Nop                 ),
        ("mov",    Mov,    Dir         ),
        ("rot",    Rot,    Dir         ),
        ("jmp",    Jmp,    Lable       ),
        ("cmp",    Cmp,    Reg,   Reg  ),
        ("jme",    Jme,    Lable       ),
        ("jne",    Jne,    Lable       ),
        ("jmg",    Jmg,    Lable       ),
        ("jml",    Jml,    Lable       ),
        ("jle",    Jle,    Lable       ),
        ("jge",    Jge,    Lable       ),
        ("jmb",    Jmb,    Lable       ),
        ("jnb",    Jnb,    Lable       ),
        ("jmc",    Jmc,    Lable       ),
        ("jnc",    Jnc,    Lable       ),
        ("jmf",    Jmf,    Lable       ),
        ("jnf",    Jnf,    Lable       ),
        ("chk",    Chk,    Dir         ),
        ("cmpv",   Cmpv,   Reg,   Val  ),
        ("split",  Split,  Dir,   Lable),
        ("forc",   Forc,   Dir,   Lable),
        ("bite",   Bite,   Dir         ),
        ("eatsun", Eatsun              ),
        ("absorb", Absorb              ),
        ("call",   Call,   Lable       ),
        ("ret",    Ret                 ),
        ("ld",     Ld,     RwReg, Reg  ),
        ("ldv",    Ldv,    RwReg, Val  ),
        ("ldr",    Ldr,    Mem,   Reg  ),
        ("ldm",    Ldm,    RwReg, Mem  )
}
