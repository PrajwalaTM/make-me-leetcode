use anyhow::anyhow;
use anyhow::Error;
use clap::Parser;
use std::str::FromStr;

#[derive(Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl FromStr for Difficulty {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "EASY" => Ok(Difficulty::Easy),
            "MEDIUM" => Ok(Difficulty::Medium),
            "HARD" => Ok(Difficulty::Hard),
            _ => Err(anyhow!(
                "Invalid value: Available variants are: EASY, MEDIUM and HARD",
            )),
        }
    }
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, value_parser, default_value = "EASY")]
    pub difficulty: String,
}
