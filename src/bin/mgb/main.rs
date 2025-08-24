use clap::{Parser, ValueEnum};
use mule_gb::load;
use serde_json;
use serde_lexpr;
use std::{fs::File, io::Read};

#[derive(Parser)]
struct Cli {
    file: String,
    /// Output format. Defaults to JSON. Possible options:
    /// json|s-expr
    #[arg(short, long)]
    format: Option<Format>,
}

#[derive(Clone, ValueEnum)]
enum Format {
    JSON,
    SExpr,
}

pub fn main() -> Result<(), String> {
    let args = Cli::parse();

    let mut file = File::open(args.file).map_err(|e| e.to_string())?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).map_err(|e| e.to_string())?;

    let gb_binary = load(&buf)?;

    let serialised = match args.format {
        Some(Format::JSON) | None => {
            serde_json::to_string_pretty(&gb_binary).expect("json serialisation")
        }
        Some(Format::SExpr) => serde_lexpr::to_string(&gb_binary).expect("lexpr serialisation"),
    };
    print!("{}", serialised);

    Ok(())
}
