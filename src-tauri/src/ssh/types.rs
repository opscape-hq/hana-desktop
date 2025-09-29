use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{IpAddr, ToSocketAddrs};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSHConnectionConfig {
    pub id: String,
    pub name: String,
    pub hostname: String,
    pub port: u16,
    pub username: String,
    pub auth_method: AuthMethod,
    pub private_key_path: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

impl std::error::Error for ValidationError {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuthMethod {
    Password,
    PublicKey,
    Agent,
}

impl AuthMethod {
    /// Returns a human-readable description of the authentication method
    pub fn description(&self) -> &'static str {
        match self {
            AuthMethod::Password => "Password authentication",
            AuthMethod::PublicKey => "Public key authentication",
            AuthMethod::Agent => "SSH agent authentication",
        }
    }

    /// Returns true if this authentication method requires a password
    pub fn requires_password(&self) -> bool {
        matches!(self, AuthMethod::Password)
    }

    /// Returns true if this authentication method requires a private key file
    pub fn requires_private_key(&self) -> bool {
        matches!(self, AuthMethod::PublicKey)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSHConnectionState {
    pub id: String,
    pub status: ConnectionStatus,
    pub error: Option<String>,
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

impl SSHConnectionState {
    /// Creates a new connection state with the given ID
    pub fn new(id: String) -> Self {
        Self {
            id,
            status: ConnectionStatus::Disconnected,
            error: None,
            last_activity: chrono::Utc::now(),
        }
    }

    /// Updates the connection status and refreshes last activity
    pub fn update_status(&mut self, status: ConnectionStatus) {
        self.status = status;
        self.error = None; // Clear error when status changes
        self.last_activity = chrono::Utc::now();
    }

    /// Sets an error state with the given error message
    pub fn set_error(&mut self, error: String) {
        self.status = ConnectionStatus::Error;
        self.error = Some(error);
        self.last_activity = chrono::Utc::now();
    }

    /// Updates the last activity timestamp
    pub fn update_activity(&mut self) {
        self.last_activity = chrono::Utc::now();
    }

    /// Checks if the connection is currently active (connected or connecting)
    pub fn is_active(&self) -> bool {
        matches!(self.status, ConnectionStatus::Connected | ConnectionStatus::Connecting)
    }

    /// Checks if the connection has an error
    pub fn has_error(&self) -> bool {
        matches!(self.status, ConnectionStatus::Error)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectionStatus {
    Connecting,
    Connected,
    Disconnected,
    Error,
}

impl ConnectionStatus {
    /// Returns true if the status represents an active connection
    pub fn is_active(&self) -> bool {
        matches!(self, ConnectionStatus::Connected | ConnectionStatus::Connecting)
    }

    /// Returns true if the status represents a terminal state (error or disconnected)
    pub fn is_terminal(&self) -> bool {
        matches!(self, ConnectionStatus::Error | ConnectionStatus::Disconnected)
    }

    /// Returns a human-readable description of the status
    pub fn description(&self) -> &'static str {
        match self {
            ConnectionStatus::Connecting => "Establishing connection...",
            ConnectionStatus::Connected => "Connected",
            ConnectionStatus::Disconnected => "Disconnected",
            ConnectionStatus::Error => "Connection error",
        }
    }
}

#[derive(Debug, Clone)]
pub enum SSHEvent {
    Connected(String),
    Disconnected(String),
    Data(String, Vec<u8>),
    Error(String, String),
    TerminalCreated(String, String), // connection_id, terminal_id
    TerminalClosed(String, String),  // connection_id, terminal_id
    TerminalResized(String, String, u16, u16), // connection_id, terminal_id, cols, rows
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalSession {
    pub id: String,
    pub connection_id: String,
    pub size: TerminalSize,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalSize {
    pub cols: u16,
    pub rows: u16,
    pub pixel_width: u16,
    pub pixel_height: u16,
}

impl Default for TerminalSize {
    fn default() -> Self {
        Self {
            cols: 80,
            rows: 24,
            pixel_width: 640,
            pixel_height: 480,
        }
    }
}

impl TerminalSession {
    pub fn new(connection_id: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            connection_id,
            size: TerminalSize::default(),
            is_active: true,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn resize(&mut self, cols: u16, rows: u16, pixel_width: u16, pixel_height: u16) {
        self.size.cols = cols;
        self.size.rows = rows;
        self.size.pixel_width = pixel_width;
        self.size.pixel_height = pixel_height;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
}

pub type ConnectionMap = HashMap<String, crate::ssh::connection::SSHConnection>;

impl SSHConnectionConfig {
    /// Validates the SSH connection configuration
    pub fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        // Validate name
        if self.name.trim().is_empty() {
            errors.push(ValidationError {
                field: "name".to_string(),
                message: "Connection name cannot be empty".to_string(),
            });
        }

        // Validate hostname
        if self.hostname.trim().is_empty() {
            errors.push(ValidationError {
                field: "hostname".to_string(),
                message: "Hostname cannot be empty".to_string(),
            });
        } else if !self.is_valid_hostname(&self.hostname) {
            errors.push(ValidationError {
                field: "hostname".to_string(),
                message: "Invalid hostname format".to_string(),
            });
        }

        // Validate port
        if self.port == 0 {
            errors.push(ValidationError {
                field: "port".to_string(),
                message: "Port must be greater than 0".to_string(),
            });
        }

        // Validate username
        if self.username.trim().is_empty() {
            errors.push(ValidationError {
                field: "username".to_string(),
                message: "Username cannot be empty".to_string(),
            });
        }

        // Validate authentication method specific requirements
        match &self.auth_method {
            AuthMethod::Password => {
                if self.password.is_none() || self.password.as_ref().unwrap().is_empty() {
                    errors.push(ValidationError {
                        field: "password".to_string(),
                        message: "Password is required for password authentication".to_string(),
                    });
                }
            }
            AuthMethod::PublicKey => {
                if let Some(key_path) = &self.private_key_path {
                    if key_path.trim().is_empty() {
                        errors.push(ValidationError {
                            field: "private_key_path".to_string(),
                            message: "Private key path cannot be empty".to_string(),
                        });
                    } else if !Path::new(key_path).exists() {
                        errors.push(ValidationError {
                            field: "private_key_path".to_string(),
                            message: "Private key file does not exist".to_string(),
                        });
                    }
                } else {
                    errors.push(ValidationError {
                        field: "private_key_path".to_string(),
                        message: "Private key path is required for public key authentication".to_string(),
                    });
                }
            }
            AuthMethod::Agent => {
                // SSH agent authentication doesn't require additional validation
                // The agent availability will be checked during connection
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Validates hostname format (basic validation)
    fn is_valid_hostname(&self, hostname: &str) -> bool {
        // Check if it's a valid IP address
        if hostname.parse::<IpAddr>().is_ok() {
            return true;
        }

        // Check if it's a valid domain name (basic check)
        if hostname.len() > 253 {
            return false;
        }

        // Check for valid characters and structure
        hostname
            .split('.')
            .all(|label| {
                !label.is_empty()
                    && label.len() <= 63
                    && label.chars().all(|c| c.is_alphanumeric() || c == '-')
                    && !label.starts_with('-')
                    && !label.ends_with('-')
            })
    }

    /// Validates that the hostname and port combination is reachable
    pub fn validate_connectivity(&self) -> Result<(), ValidationError> {
        let address = format!("{}:{}", self.hostname, self.port);
        
        match address.to_socket_addrs() {
            Ok(mut addrs) => {
                if addrs.next().is_some() {
                    Ok(())
                } else {
                    Err(ValidationError {
                        field: "hostname".to_string(),
                        message: "Unable to resolve hostname".to_string(),
                    })
                }
            }
            Err(_) => Err(ValidationError {
                field: "hostname".to_string(),
                message: "Invalid hostname or port combination".to_string(),
            })
        }
    }

    /// Creates a new SSH connection configuration with validation
    pub fn new(
        name: String,
        hostname: String,
        port: u16,
        username: String,
        auth_method: AuthMethod,
        private_key_path: Option<String>,
        password: Option<String>,
    ) -> Result<Self, Vec<ValidationError>> {
        let config = Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            hostname,
            port,
            username,
            auth_method,
            private_key_path,
            password,
        };

        config.validate()?;
        Ok(config)
    }
}
#[cfg
(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ssh_config_password_auth() {
        let config = SSHConnectionConfig::new(
            "Test Server".to_string(),
            "example.com".to_string(),
            22,
            "testuser".to_string(),
            AuthMethod::Password,
            None,
            Some("password123".to_string()),
        );

        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.name, "Test Server");
        assert_eq!(config.hostname, "example.com");
        assert_eq!(config.port, 22);
        assert_eq!(config.username, "testuser");
        assert_eq!(config.auth_method, AuthMethod::Password);
    }

    #[test]
    fn test_invalid_ssh_config_empty_name() {
        let config = SSHConnectionConfig::new(
            "".to_string(),
            "example.com".to_string(),
            22,
            "testuser".to_string(),
            AuthMethod::Password,
            None,
            Some("password123".to_string()),
        );

        assert!(config.is_err());
        let errors = config.unwrap_err();
        assert!(errors.iter().any(|e| e.field == "name"));
    }

    #[test]
    fn test_invalid_ssh_config_empty_hostname() {
        let config = SSHConnectionConfig::new(
            "Test Server".to_string(),
            "".to_string(),
            22,
            "testuser".to_string(),
            AuthMethod::Password,
            None,
            Some("password123".to_string()),
        );

        assert!(config.is_err());
        let errors = config.unwrap_err();
        assert!(errors.iter().any(|e| e.field == "hostname"));
    }

    #[test]
    fn test_invalid_ssh_config_zero_port() {
        let config = SSHConnectionConfig::new(
            "Test Server".to_string(),
            "example.com".to_string(),
            0,
            "testuser".to_string(),
            AuthMethod::Password,
            None,
            Some("password123".to_string()),
        );

        assert!(config.is_err());
        let errors = config.unwrap_err();
        assert!(errors.iter().any(|e| e.field == "port"));
    }

    #[test]
    fn test_invalid_ssh_config_password_auth_no_password() {
        let config = SSHConnectionConfig::new(
            "Test Server".to_string(),
            "example.com".to_string(),
            22,
            "testuser".to_string(),
            AuthMethod::Password,
            None,
            None,
        );

        assert!(config.is_err());
        let errors = config.unwrap_err();
        assert!(errors.iter().any(|e| e.field == "password"));
    }

    #[test]
    fn test_invalid_ssh_config_publickey_auth_no_key() {
        let config = SSHConnectionConfig::new(
            "Test Server".to_string(),
            "example.com".to_string(),
            22,
            "testuser".to_string(),
            AuthMethod::PublicKey,
            None,
            None,
        );

        assert!(config.is_err());
        let errors = config.unwrap_err();
        assert!(errors.iter().any(|e| e.field == "private_key_path"));
    }

    #[test]
    fn test_valid_hostname_formats() {
        let config = SSHConnectionConfig {
            id: "test".to_string(),
            name: "Test".to_string(),
            hostname: "192.168.1.1".to_string(),
            port: 22,
            username: "user".to_string(),
            auth_method: AuthMethod::Agent,
            private_key_path: None,
            password: None,
        };
        assert!(config.is_valid_hostname("192.168.1.1"));
        assert!(config.is_valid_hostname("example.com"));
        assert!(config.is_valid_hostname("sub.example.com"));
        assert!(config.is_valid_hostname("localhost"));
    }

    #[test]
    fn test_invalid_hostname_formats() {
        let config = SSHConnectionConfig {
            id: "test".to_string(),
            name: "Test".to_string(),
            hostname: "".to_string(),
            port: 22,
            username: "user".to_string(),
            auth_method: AuthMethod::Agent,
            private_key_path: None,
            password: None,
        };
        assert!(!config.is_valid_hostname(""));
        assert!(!config.is_valid_hostname("-invalid.com"));
        assert!(!config.is_valid_hostname("invalid-.com"));
        assert!(!config.is_valid_hostname("toolongdomainname".repeat(20).as_str()));
    }

    #[test]
    fn test_connection_state_management() {
        let mut state = SSHConnectionState::new("test-id".to_string());
        
        assert_eq!(state.id, "test-id");
        assert_eq!(state.status, ConnectionStatus::Disconnected);
        assert!(!state.is_active());
        assert!(!state.has_error());

        state.update_status(ConnectionStatus::Connecting);
        assert_eq!(state.status, ConnectionStatus::Connecting);
        assert!(state.is_active());

        state.update_status(ConnectionStatus::Connected);
        assert_eq!(state.status, ConnectionStatus::Connected);
        assert!(state.is_active());

        state.set_error("Connection failed".to_string());
        assert_eq!(state.status, ConnectionStatus::Error);
        assert!(!state.is_active());
        assert!(state.has_error());
        assert_eq!(state.error, Some("Connection failed".to_string()));
    }

    #[test]
    fn test_auth_method_properties() {
        assert!(AuthMethod::Password.requires_password());
        assert!(!AuthMethod::Password.requires_private_key());

        assert!(!AuthMethod::PublicKey.requires_password());
        assert!(AuthMethod::PublicKey.requires_private_key());

        assert!(!AuthMethod::Agent.requires_password());
        assert!(!AuthMethod::Agent.requires_private_key());
    }

    #[test]
    fn test_connection_status_properties() {
        assert!(ConnectionStatus::Connected.is_active());
        assert!(ConnectionStatus::Connecting.is_active());
        assert!(!ConnectionStatus::Disconnected.is_active());
        assert!(!ConnectionStatus::Error.is_active());

        assert!(ConnectionStatus::Disconnected.is_terminal());
        assert!(ConnectionStatus::Error.is_terminal());
        assert!(!ConnectionStatus::Connected.is_terminal());
        assert!(!ConnectionStatus::Connecting.is_terminal());
    }
}