//! Platform-specific SSH implementations
//!
//! This module provides cross-platform SSH connection functionality.
//! It uses conditional compilation to provide the appropriate implementation
//! for each target platform.

#[cfg(unix)]
pub mod unix {
    use std::process::Command;
    use std::os::unix::process::CommandExt;

    pub fn connect_to_host(hostname: &str, port: u16, username: &str, password: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        // Check dependencies before attempting connection
        crate::dependencies::check_ssh_dependencies(password.is_some())?;
        
        let mut cmd;
        if let Some(pw) = password {
            // Use sshpass to provide the password non-interactively
            cmd = Command::new("sshpass");
            cmd.arg("-p").arg(pw)
               .arg("ssh")
               .arg("-p").arg(port.to_string())
               .arg(format!("{}@{}", username, hostname));
            println!("Launching SSH connection to {}@{}:{} with password...", username, hostname, port);
        } else {
            cmd = Command::new("ssh");
            cmd.arg("-p").arg(port.to_string())
               .arg(format!("{}@{}", username, hostname));
            println!("Launching SSH connection to {}@{}:{}...", username, hostname, port);
        }
        // This will replace the current process with SSH or sshpass
        let error = cmd.exec();
        Err(format!("Failed to execute SSH command: {}", error).into())
    }
}

#[cfg(windows)]
pub mod windows {
    use std::process::Command;

    pub fn connect_to_host(hostname: &str, port: u16, username: &str, password: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        // Check dependencies before attempting connection
        crate::dependencies::check_ssh_dependencies(password.is_some())?;
        
        if let Some(pw) = password {
            // On Windows, we'll use PowerShell with plink (from PuTTY suite)
            // This requires plink to be installed and available in PATH
            let mut cmd = Command::new("plink");
            cmd.arg("-P").arg(port.to_string())
               .arg("-pw").arg(pw)
               .arg(format!("{}@{}", username, hostname));
            println!("Launching SSH connection to {}@{}:{} with password...", username, hostname, port);
            
            let output = cmd.output()?;
            if !output.status.success() {
                return Err(format!("SSH command failed with status: {}", output.status).into());
            }
        } else {
            // Without password, use Windows SSH client if available
            let mut cmd = Command::new("ssh");
            cmd.arg("-p").arg(port.to_string())
               .arg(format!("{}@{}", username, hostname));
            println!("Launching SSH connection to {}@{}:{}...", username, hostname, port);
            
            let output = cmd.output()?;
            if !output.status.success() {
                return Err(format!("SSH command failed with status: {}", output.status).into());
            }
        }
        Ok(())
    }
}