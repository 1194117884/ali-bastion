use clap::Parser;
use ali_bastion::commands::Commands;
use ali_bastion::handlers;

#[derive(Parser)]
#[command(name = "ali-bastion")]
#[command(about = "A CLI tool for managing and connecting to Aliyun bastion hosts", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add(args) => {
            handlers::handle_add(args)?;
        }
        Commands::Remove(args) => {
            handlers::handle_remove(args)?;
        }
        Commands::List => {
            handlers::handle_list()?;
        }
        Commands::Connect(args) => {
            handlers::handle_connect(args)?;
        }
    }

    Ok(())
}
