use crate::config::{Config, HostConfig};
use crate::ssh;
use crate::commands::types::{AddArgs, RemoveArgs, ConnectArgs};
use crate::utils::encryption::{encrypt_password, decrypt_password};
mod interactive;

pub fn handle_add(args: AddArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::load()?;
    
    // Check if host with the same name already exists
    if config.get_host(&args.name).is_some() {
        println!("Error: Host '{}' already exists. Please use a different name or remove the existing host first.", args.name);
        return Ok(());
    }
    
    // Encrypt password if provided
    let encrypted_password = match args.password {
        Some(password) => {
            match encrypt_password(&password) {
                Ok(encrypted) => Some(encrypted),
                Err(e) => {
                    println!("Error encrypting password: {}", e);
                    return Ok(());
                }
            }
        }
        None => None,
    };
    
    let host = HostConfig {
        name: args.name.clone(),
        hostname: args.hostname,
        port: args.port,
        username: args.username,
        password: encrypted_password,
    };
    config.add_host(host);
    config.save()?;
    println!("Host '{}' added successfully", args.name);
    Ok(())
}

pub fn handle_remove(args: RemoveArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::load()?;
    if config.remove_host(&args.name) {
        config.save()?;
        println!("Host '{}' removed successfully", args.name);
        Ok(())
    } else {
        println!("Host '{}' not found", args.name);
        Ok(())
    }
}

pub fn handle_list() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;
    let hosts = config.list_hosts();
    
    if hosts.is_empty() {
        println!("No hosts configured");
    } else {
        println!("Configured hosts:");
        for host in hosts {
            let password_status = if host.password.is_some() { "(encrypted password)" } else { "" };
            println!("  - {}: {}@{}:{} {}", host.name, host.username, host.hostname, host.port, password_status);
        }
    }
    Ok(())
}

pub fn handle_connect(args: ConnectArgs) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load()?;
    let hosts = config.list_hosts();
    
    // If no hosts configured, show message and return
    if hosts.is_empty() {
        println!("No hosts configured. Use 'add' command to add a host first.");
        return Ok(());
    }
    
    // Determine which host to connect to
    let host_to_connect = if let Some(host_name) = args.name {
        // Direct mode - user provided host name
        match config.get_host(&host_name) {
            Some(host) => host.clone(),
            None => {
                println!("Host '{}' not found", host_name);
                return Ok(());
            }
        }
    } else {
        // Interactive mode - let user select from list
        match interactive::select_host_interactively(hosts) {
            Some(selected_host_name) => {
                match config.get_host(&selected_host_name) {
                    Some(host) => host.clone(),
                    None => {
                        println!("Selected host '{}' not found", selected_host_name);
                        return Ok(());
                    }
                }
            }
            None => {
                println!("No host selected");
                return Ok(());
            }
        }
    };
    
    // Decrypt password if it exists
    let decrypted_password = match host_to_connect.password {
        Some(ref encrypted_password) => {
            match decrypt_password(encrypted_password) {
                Ok(decrypted) => Some(decrypted),
                Err(e) => {
                    println!("Warning: Failed to decrypt password: {}", e);
                    None
                }
            }
        }
        None => None,
    };
    
    // Connect to the selected host
    let password = decrypted_password.as_deref();
    ssh::connect_to_host(
        &host_to_connect.hostname, 
        host_to_connect.port, 
        &host_to_connect.username, 
        password
    )?;
    
    Ok(())
}