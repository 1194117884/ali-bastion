use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new host
    Add(AddArgs),
    /// Remove a host
    Remove(RemoveArgs),
    /// List all hosts
    List,
    /// Connect to a host (if no host provided, interactive selection will be used)
    Connect(ConnectArgs),
}

#[derive(Args)]
pub struct AddArgs {
    /// Host name
    #[arg(short = 'n', long)]
    pub name: String,
    
    /// Hostname or IP address
    #[arg(short = 'H', long)]
    pub hostname: String,
    
    /// Port number
    #[arg(short = 'p', long, default_value_t = 60022)]
    pub port: u16,
    
    /// Username
    #[arg(short = 'u', long)]
    pub username: String,
    
    /// Password
    #[arg(short = 'P', long)]
    pub password: Option<String>,
}

#[derive(Args)]
pub struct RemoveArgs {
    /// Host name to remove
    pub name: String,
}

#[derive(Args)]
pub struct ConnectArgs {
    /// Host name to connect to (if not provided, interactive selection will be used)
    pub name: Option<String>,
}