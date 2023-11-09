mod divy;
use clap::{Args, Parser, Subcommand};
use color_eyre::owo_colors::OwoColorize;
use colored::*;

#[derive(Parser)]
#[clap(name = "Divy", version = "0.1.0", author = "Elton Mpinyuri")]
#[clap(about = "A CLI tool for dividing numbers")]
struct Cli {
    /// Optional name to used
    name: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Divide the two numbers
    Divide(Divide),
}

#[derive(Args)]
struct Divide {
    /// the dividend
    dividend: i32,
    /// the divisor
    divisor: i32,
}

fn main() {
    let args = Cli::parse();
    if let Some(name) = args.name {
        if name == "arith".to_string() {
            match &args.command {
                Some(Commands::Divide(divide)) => {
                    match divy::division::division(divide.dividend, divide.divisor) {
                        Ok(result) => println!(
                            "{} / {} = {}",
                            &divide.dividend.green(),
                            &divide.divisor.green(),
                            &result.yellow()
                        ),
                        Err(err) => {
                            println!("Error: {}", err.to_string().yellow());
                            return;
                        }
                    };
                }
                None => {
                    println!("{}", "No subcommand was used".red());
                }
            }
        } else {
            eprintln!("{}: {}", "Error".red(), "Invalid name".yellow());
        }
    }
}
