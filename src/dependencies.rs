//! Dependency management for the application
//!
//! This module provides functionality to check for required tools
//! and attempt to install them if they're missing.

use std::process::Command;

/// Check if a command exists in the system PATH
fn command_exists(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Check if a command exists on Windows
#[cfg(windows)]
fn command_exists_windows(command: &str) -> bool {
    Command::new("where")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Attempt to install sshpass on Unix systems
#[cfg(unix)]
fn install_sshpass() -> Result<(), Box<dyn std::error::Error>> {
    // Try different package managers
    if command_exists("apt-get") {
        // Debian/Ubuntu
        let status = Command::new("sudo")
            .arg("apt-get")
            .arg("install")
            .arg("-y")
            .arg("sshpass")
            .status()?;
        if status.success() {
            return Ok(());
        }
    } else if command_exists("yum") {
        // RHEL/CentOS
        let status = Command::new("sudo")
            .arg("yum")
            .arg("install")
            .arg("-y")
            .arg("sshpass")
            .status()?;
        if status.success() {
            return Ok(());
        }
    } else if command_exists("dnf") {
        // Fedora
        let status = Command::new("sudo")
            .arg("dnf")
            .arg("install")
            .arg("-y")
            .arg("sshpass")
            .status()?;
        if status.success() {
            return Ok(());
        }
    } else if command_exists("brew") {
        // macOS
        let status = Command::new("brew")
            .arg("install")
            .arg("sshpass")
            .status()?;
        if status.success() {
            return Ok(());
        }
    } else if command_exists("pacman") {
        // Arch Linux
        let status = Command::new("sudo")
            .arg("pacman")
            .arg("-S")
            .arg("--noconfirm")
            .arg("sshpass")
            .status()?;
        if status.success() {
            return Ok(());
        }
    }
    
    Err("Unable to install sshpass. Please install it manually.".into())
}

/// Check if required SSH tools are available
pub fn check_ssh_dependencies(password_auth: bool) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(unix)]
    {
        if password_auth && !command_exists("sshpass") {
            println!("sshpass is required for password-based authentication but not found.");
            println!("Attempting to install sshpass...");
            
            match install_sshpass() {
                Ok(_) => println!("sshpass installed successfully."),
                Err(e) => {
                    eprintln!("Failed to install sshpass: {}", e);
                    eprintln!("Please install sshpass manually:");
                    eprintln!("  - Ubuntu/Debian: sudo apt-get install sshpass");
                    eprintln!("  - RHEL/CentOS: sudo yum install sshpass");
                    eprintln!("  - Fedora: sudo dnf install sshpass");
                    eprintln!("  - macOS: brew install sshpass");
                    eprintln!("  - Arch Linux: sudo pacman -S sshpass");
                    return Err("sshpass is required for password-based authentication".into());
                }
            }
        }
        
        if !command_exists("ssh") {
            return Err("OpenSSH client (ssh) is required but not found. Please install it.".into());
        }
    }
    
    #[cfg(windows)]
    {
        if password_auth && !command_exists_windows("plink.exe") {
            return Err("plink.exe (from PuTTY) is required for password-based authentication on Windows but not found. Please download PuTTY from https://www.chiark.greenend.org.uk/~sgtatham/putty/latest.html and ensure plink.exe is in your PATH.".into());
        }
        
        if !command_exists_windows("ssh.exe") {
            eprintln!("Warning: Windows SSH client not found. You may need to enable the OpenSSH client feature in Windows or install it manually.");
        }
    }
    
    Ok(())
}