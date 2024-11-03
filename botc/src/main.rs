use anyhow::{Context, Result};
use clap::Parser;
use clap_derive::Parser;
use std::fs::{read_to_string, write};

use botc::{code_packer, compiler};

/// Genetic code compiler
#[derive(Parser)]
struct Options {
    /// Input file
    #[arg(value_name = "file")]
    input: String,

    /// Output file
    #[arg(short, long, value_name = "output")]
    output: Option<String>,

    /// Decompile the input file
    #[arg(short, long, default_value_t = false)]
    decompile: bool,
}

fn main() -> Result<()> {
    let opt = Options::parse();
    let code = read_to_string(&opt.input)
        .context(format!("Failed to open input file \"{}\"", opt.input))?;

    let result: String = if opt.decompile {
        let decoded = code_packer::from_b32(&code).context("Failed to decode input file")?;
        botc::compiler::decompile(decoded)
            .iter()
            .fold(String::new(), |mut acc, cmd| {
                acc.push_str(format!("{}", cmd).as_str());
                acc.push_str("\n");
                acc
            })
    } else {
        let commands = compiler::compile(code)
            .context(format!("Failed to compile input file \"{}\"", opt.input))?;
        code_packer::to_b32(&commands).context("Failed to encode compiled code")?
    };

    if let Some(file) = &opt.output {
        write(file, result).context(format!("Failed to output input file \"{}\"", file))?;
    } else {
        println!("Result:\n{}", result);
    }
    Ok(())
}
