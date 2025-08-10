mod platform;

#[cfg(unix)]
use platform::unix::connect_to_host as platform_connect;

#[cfg(windows)]
use platform::windows::connect_to_host as platform_connect;

pub fn connect_to_host(hostname: &str, port: u16, username: &str, password: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    platform_connect(hostname, port, username, password)
}