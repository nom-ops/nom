use clap::Parser;
use nom::cli::{Cli, Commands};
use nom::executor;
use nom::tracker;
use nom::ui;

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Npm(args) => {
            if args.is_empty() {
                let code = executor::run(&args);
                std::process::exit(code);
            }
            
            let subcmd = &args[0];
            
            if nom::cli::parser::is_valid_command(subcmd) 
                || nom::executor::validator::exists_in_npm(subcmd) 
            {
                let code = executor::run(&args);
                std::process::exit(code);
            } else {
                let suggestion = nom::cli::parser::find_suggestion(subcmd);
                let count = tracker::record_mistake(subcmd.clone(), suggestion.clone());
                
                let msg = match suggestion {
                    Some(s) => format!("'{}' → did you mean '{}'?", subcmd, s),
                    None => format!("'{}' is not a valid npm command", subcmd),
                };
                
                println!("{}", ui::format_error_box(&msg, count));
                std::process::exit(1);
            }
        }
    }
}
