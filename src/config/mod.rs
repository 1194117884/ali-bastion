use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HostConfig {
    pub name: String,
    pub hostname: String,
    pub port: u16,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub hosts: HashMap<String, HostConfig>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            hosts: HashMap::new(),
        }
    }

    pub fn get_config_path() -> PathBuf {
        let mut config_path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        config_path.push(".ali-bastion");
        config_path.push("config.json");
        config_path
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path();
        
        if !config_path.exists() {
            // Create directory if it doesn't exist
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let config = Config::new();
            config.save()?;
            return Ok(config);
        }

        let content = fs::read_to_string(config_path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path();
        
        // Create directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let content = serde_json::to_string_pretty(self)?;
        fs::write(config_path, content)?;
        Ok(())
    }

    pub fn add_host(&mut self, host: HostConfig) {
        self.hosts.insert(host.name.clone(), host);
    }

    pub fn remove_host(&mut self, name: &str) -> bool {
        self.hosts.remove(name).is_some()
    }

    pub fn get_host(&self, name: &str) -> Option<&HostConfig> {
        self.hosts.get(name)
    }

    pub fn list_hosts(&self) -> Vec<&HostConfig> {
        self.hosts.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() {
        let config = Config::new();
        assert!(config.hosts.is_empty());
    }

    #[test]
    fn test_add_host() {
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
        assert_eq!(config.get_host("test_host"), Some(&host));
    }

    #[test]
    fn test_remove_host() {
        let mut config = Config::new();
        let host = HostConfig {
            name: "test_host".to_string(),
            hostname: "192.168.1.1".to_string(),
            port: 60022,
            username: "testuser".to_string(),
            password: None,
        };
        
        config.add_host(host);
        assert_eq!(config.hosts.len(), 1);
        
        let removed = config.remove_host("test_host");
        assert!(removed);
        assert_eq!(config.hosts.len(), 0);
        
        // Try to remove non-existent host
        let removed = config.remove_host("non_existent");
        assert!(!removed);
    }

    #[test]
    fn test_get_host() {
        let mut config = Config::new();
        let host = HostConfig {
            name: "test_host".to_string(),
            hostname: "192.168.1.1".to_string(),
            port: 60022,
            username: "testuser".to_string(),
            password: None,
        };
        
        config.add_host(host.clone());
        assert_eq!(config.get_host("test_host"), Some(&host));
        assert_eq!(config.get_host("non_existent"), None);
    }

    #[test]
    fn test_list_hosts() {
        let mut config = Config::new();
        let host1 = HostConfig {
            name: "host1".to_string(),
            hostname: "192.168.1.1".to_string(),
            port: 60022,
            username: "user1".to_string(),
            password: None,
        };
        let host2 = HostConfig {
            name: "host2".to_string(),
            hostname: "192.168.1.2".to_string(),
            port: 60022,
            username: "user2".to_string(),
            password: None,
        };
        
        config.add_host(host1);
        config.add_host(host2);
        
        let hosts = config.list_hosts();
        assert_eq!(hosts.len(), 2);
    }

    #[test]
    fn test_duplicate_host_names() {
        let mut config = Config::new();
        let host1 = HostConfig {
            name: "test_host".to_string(),
            hostname: "192.168.1.1".to_string(),
            port: 60022,
            username: "user1".to_string(),
            password: None,
        };
        let host2 = HostConfig {
            name: "test_host".to_string(), // Same name as host1
            hostname: "192.168.1.2".to_string(),
            port: 60022,
            username: "user2".to_string(),
            password: None,
        };
        
        config.add_host(host1.clone());
        assert_eq!(config.hosts.len(), 1);
        assert_eq!(config.get_host("test_host"), Some(&host1));
        
        // Adding host with same name should overwrite the existing one
        config.add_host(host2.clone());
        assert_eq!(config.hosts.len(), 1);
        assert_eq!(config.get_host("test_host"), Some(&host2));
    }
}