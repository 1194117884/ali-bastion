pub mod commands;
pub mod config;
pub mod ssh;
pub mod utils;
pub mod handlers;
pub mod dependencies;

#[cfg(test)]
mod tests {
    use super::config::{Config, HostConfig};

    #[test]
    fn test_add_and_retrieve_host() {
        // Test adding a host
        let mut config = Config::new();
        
        let host = HostConfig {
            name: "test_host".to_string(),
            hostname: "192.168.1.1".to_string(),
            port: 60022,
            username: "testuser".to_string(),
            password: None,
        };
        
        config.add_host(host.clone());
        assert_eq!(config.hosts.len(), 1);
        
        // Retrieve the host
        let retrieved_host = config.get_host("test_host");
        assert_eq!(retrieved_host, Some(&host));
    }
}