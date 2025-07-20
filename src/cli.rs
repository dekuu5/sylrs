
use clap::{ Parser, Subcommand};

use crate::status::Status;


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

pub fn run_cli(s : &mut Status) {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Status =>{
            println!("{:?}",s)
        },
        Commands::Login { token } => {
            let token = token.clone();
            match login(token) {
                Ok(x) => {
                    println!("{}",x);
                    s.update_token(x);
            
                },
                Err(e) => eprintln!("Error : {}",e),
            }
        }
    }   
}



fn login(token: Option<String>) -> Result<String, String> {
    match token {
        Some(x) => Ok(x),
        None => Err("you need to provide a token".to_owned())
    }
}