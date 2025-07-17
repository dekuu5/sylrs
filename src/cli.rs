use std::string;

use clap::{builder::Str, Parser, Subcommand};


#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command : Commands,
}

#[derive(Subcommand)]
enum Commands {
    Status,
    Login {token : Option<String>} 
}

pub fn run_cli() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Status => get_status(),
        Commands::Login { token } => {
            let token = token.clone();
            match login(token) {
                Ok(x) => println!("{}",x),
                Err(e) => eprintln!("Error : {}",e),
            }
        }
    }   
}


fn get_status() {
    todo!()
}

fn login(token: Option<String>) -> Result<String, String> {
    match token {
        Some(x) => Ok(x),
        None => Err("you need to provide a token".to_owned())
    }
}