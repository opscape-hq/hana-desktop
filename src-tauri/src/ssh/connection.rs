use crate::ssh::types::*;
use crate::ssh::terminal::TerminalSessionManager;
use russh::client::{self, Handle, Msg};
use russh::keys::*;
use russh::{Channel, ChannelId};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio::time::Duration;

pub struct SSHConnection {
    pub id: String,
    pub config: SSHConnectionConfig,
    pub event_sender: mpsc::Sender<SSHEvent>,
    state: Arc<RwLock<ConnectionState>>,
    terminal_manager: Arc<TerminalSessionManager>,
}

struct ConnectionState {
    session: Option<Handle<SSHClient>>,
    channels: HashMap<ChannelId, Channel<Msg>>,
    connected: bool,
}

impl ConnectionState {
    fn new() -> Self {
        Self {
            session: None,
            channels: HashMap::new(),
            connected: false,
        }
    }
}

// SSH Client Handler implementation
pub struct SSHClient {
    event_sender: mpsc::Sender<SSHEvent>,
    connection_id: String,
}

#[async_trait::async_trait]
impl client::Handler for SSHClient {
    type Error = russh::Error;

    async fn check_server_key(&mut self, _server_public_key: &key::PublicKey) -> Result<bool, Self::Error> {
        // For now, accept all server keys
        // In production, this should verify against known_hosts
        Ok(true)
    }

    async fn data(&mut self, _channel: ChannelId, data: &[u8], _session: &mut client::Session) -> Result<(), Self::Error> {
        // Send terminal data to frontend
        let _ = self.event_sender.send(SSHEvent::Data(
            self.connection_id.clone(),
            data.to_vec(),
        )).await;
        Ok(())
    }

    async fn extended_data(&mut self, _channel: ChannelId, _ext: u32, data: &[u8], _session: &mut client::Session) -> Result<(), Self::Error> {
        // Send stderr data to frontend
        let _ = self.event_sender.send(SSHEvent::Data(
            self.connection_id.clone(),
            data.to_vec(),
        )).await;
        Ok(())
    }
}

impl SSHConnection {
    pub fn new(config: SSHConnectionConfig, event_sender: mpsc::Sender<SSHEvent>) -> Self {
        let terminal_manager = Arc::new(TerminalSessionManager::new(event_sender.clone()));
        
        Self {
            id: config.id.clone(),
            config,
            event_sender,
            state: Arc::new(RwLock::new(ConnectionState::new())),
            terminal_manager,
        }
    }

    pub async fn connect(&self) -> Result<(), String> {
        // Validate configuration first
        self.config.validate().map_err(|errors| {
            format!("Configuration validation failed: {}", 
                errors.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", "))
        })?;

        // Send connecting event
        let _ = self.event_sender.send(SSHEvent::Connected(self.id.clone())).await;

        // Resolve address
        let address = format!("{}:{}", self.config.hostname, self.config.port);
        let socket_addr: SocketAddr = address.parse()
            .map_err(|_| format!("Invalid address format: {}", address))?;

        // Create SSH client handler
        let client_handler = SSHClient {
            event_sender: self.event_sender.clone(),
            connection_id: self.id.clone(),
        };

        // Create SSH client configuration
        let ssh_config = Arc::new(client::Config {
            inactivity_timeout: Some(Duration::from_secs(300)), // 5 minutes
            ..<_>::default()
        });

        // Create SSH session using the address directly
        let mut session = client::connect(ssh_config, socket_addr, client_handler)
            .await
            .map_err(|e| format!("SSH connection failed: {}", e))?;

        // Authenticate based on the configured method
        let auth_result = match &self.config.auth_method {
            AuthMethod::Password => {
                let password = self.config.password.as_ref()
                    .ok_or("Password not provided for password authentication")?;
                
                session.authenticate_password(&self.config.username, password)
                    .await
                    .map_err(|e| format!("Password authentication failed: {}", e))
            }
            AuthMethod::PublicKey => {
                let key_path = self.config.private_key_path.as_ref()
                    .ok_or("Private key path not provided for public key authentication")?;
                
                self.authenticate_with_key(&mut session, key_path).await
            }
            AuthMethod::Agent => {
                self.authenticate_with_agent(&mut session).await
            }
        };

        match auth_result {
            Ok(true) => {
                // Authentication successful, update state
                let mut state = self.state.write().await;
                state.session = Some(session);
                state.connected = true;
                
                // Send connected event
                let _ = self.event_sender.send(SSHEvent::Connected(self.id.clone())).await;
                Ok(())
            }
            Ok(false) => {
                Err("Authentication failed: Invalid credentials".to_string())
            }
            Err(e) => {
                let _ = self.event_sender.send(SSHEvent::Error(self.id.clone(), e.clone())).await;
                Err(e)
            }
        }
    }

    async fn authenticate_with_key(
        &self,
        _session: &mut Handle<SSHClient>,
        key_path: &str,
    ) -> Result<bool, String> {
        // Check if key file exists
        if !Path::new(key_path).exists() {
            return Err(format!("Private key file not found: {}", key_path));
        }

        // Load private key
        let _key_pair = load_secret_key(key_path, None)
            .map_err(|e| format!("Failed to load private key: {}", e))?;

        // For now, return error as full implementation needs more work
        Err("Public key authentication not yet fully implemented".to_string())
    }

    async fn authenticate_with_agent(
        &self,
        _session: &mut Handle<SSHClient>,
    ) -> Result<bool, String> {
        // For now, return an error as SSH agent support needs more complex implementation
        // This will be implemented in a future task
        Err("SSH agent authentication not yet implemented".to_string())
    }

    pub async fn disconnect(&self) -> Result<(), String> {
        // Close all terminal sessions first
        self.terminal_manager.close_all_sessions_for_connection(&self.id).await?;
        
        let mut state = self.state.write().await;
        
        if let Some(session) = state.session.take() {
            // Close all channels first
            for (_, channel) in state.channels.drain() {
                let _ = channel.close().await;
            }
            
            // Disconnect the session
            let _ = session.disconnect(russh::Disconnect::ByApplication, "", "").await;
            state.connected = false;
            
            // Send disconnected event
            let _ = self.event_sender.send(SSHEvent::Disconnected(self.id.clone())).await;
        }
        
        Ok(())
    }

    pub async fn is_connected(&self) -> bool {
        let state = self.state.read().await;
        state.connected
    }

    // Channel management methods will be implemented in future tasks
    // For now, we focus on connection establishment only
    
    pub async fn get_session_info(&self) -> Option<String> {
        let state = self.state.read().await;
        if state.connected {
            Some(format!("Connected to {}@{}:{}", 
                self.config.username, 
                self.config.hostname, 
                self.config.port))
        } else {
            None
        }
    }

    /// Creates a new terminal session for this SSH connection
    pub async fn create_terminal_session(&self) -> Result<String, String> {
        let state = self.state.read().await;
        if let Some(_session) = &state.session {
            if state.connected {
                self.terminal_manager
                    .create_session(self.id.clone())
                    .await
            } else {
                Err("SSH connection is not active".to_string())
            }
        } else {
            Err("SSH session not available".to_string())
        }
    }

    /// Sends input to a terminal session
    pub async fn send_terminal_input(&self, terminal_id: &str, data: &[u8]) -> Result<(), String> {
        self.terminal_manager.send_input(terminal_id, data).await
    }

    /// Resizes a terminal session
    pub async fn resize_terminal(
        &self,
        terminal_id: &str,
        cols: u16,
        rows: u16,
        pixel_width: u16,
        pixel_height: u16,
    ) -> Result<(), String> {
        self.terminal_manager
            .resize_terminal(terminal_id, cols, rows, pixel_width, pixel_height)
            .await
    }

    /// Closes a terminal session
    pub async fn close_terminal_session(&self, terminal_id: &str) -> Result<(), String> {
        self.terminal_manager.close_session(terminal_id).await
    }

    /// Gets information about a terminal session
    pub async fn get_terminal_session(&self, terminal_id: &str) -> Option<TerminalSession> {
        self.terminal_manager.get_session(terminal_id).await
    }

    /// Lists all terminal sessions for this connection
    pub async fn list_terminal_sessions(&self) -> Vec<TerminalSession> {
        self.terminal_manager
            .list_sessions_for_connection(&self.id)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;
    use tokio::time::{timeout, Duration};

    // Helper function to create a test SSH configuration
    fn create_test_config() -> SSHConnectionConfig {
        SSHConnectionConfig {
            id: "test-connection".to_string(),
            name: "Test Connection".to_string(),
            hostname: "localhost".to_string(),
            port: 22,
            username: "testuser".to_string(),
            auth_method: AuthMethod::Password,
            private_key_path: None,
            password: Some("testpass".to_string()),
        }
    }

    // Helper function to create a test SSH configuration with public key auth
    fn create_test_config_pubkey() -> SSHConnectionConfig {
        SSHConnectionConfig {
            id: "test-connection-key".to_string(),
            name: "Test Connection Key".to_string(),
            hostname: "localhost".to_string(),
            port: 22,
            username: "testuser".to_string(),
            auth_method: AuthMethod::PublicKey,
            private_key_path: Some("/tmp/test_key".to_string()),
            password: None,
        }
    }

    #[tokio::test]
    async fn test_ssh_connection_creation() {
        let config = create_test_config();
        let (event_sender, _event_receiver) = mpsc::channel(10);
        
        let connection = SSHConnection::new(config.clone(), event_sender);
        
        assert_eq!(connection.id, config.id);
        assert_eq!(connection.config.hostname, config.hostname);
        assert_eq!(connection.config.port, config.port);
        assert_eq!(connection.config.username, config.username);
    }

    #[tokio::test]
    async fn test_ssh_connection_initial_state() {
        let config = create_test_config();
        let (event_sender, _event_receiver) = mpsc::channel(10);
        
        let connection = SSHConnection::new(config, event_sender);
        
        // Initially should not be connected
        assert!(!connection.is_connected().await);
    }

    #[tokio::test]
    async fn test_ssh_connection_validation_failure() {
        let mut config = create_test_config();
        config.hostname = "".to_string(); // Invalid hostname
        
        let (event_sender, _event_receiver) = mpsc::channel(10);
        let connection = SSHConnection::new(config, event_sender);
        
        // Connection should fail due to validation error
        let result = connection.connect().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Configuration validation failed"));
    }

    #[tokio::test]
    async fn test_ssh_connection_invalid_address() {
        let mut config = create_test_config();
        config.hostname = "invalid-hostname-that-does-not-exist-12345".to_string();
        
        let (event_sender, _event_receiver) = mpsc::channel(10);
        let connection = SSHConnection::new(config, event_sender);
        
        // Connection should fail due to invalid address
        let result = connection.connect().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ssh_connection_timeout() {
        let mut config = create_test_config();
        config.hostname = "192.0.2.1".to_string(); // RFC5737 test address (should timeout)
        config.port = 12345; // Non-standard port
        
        let (event_sender, _event_receiver) = mpsc::channel(10);
        let connection = SSHConnection::new(config, event_sender);
        
        // Connection should timeout
        let result = connection.connect().await;
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("timeout") || error_msg.contains("Failed to connect") || error_msg.contains("SSH connection failed"));
    }

    #[tokio::test]
    async fn test_ssh_connection_missing_password() {
        let mut config = create_test_config();
        config.password = None; // Missing password for password auth
        
        let (event_sender, _event_receiver) = mpsc::channel(10);
        let connection = SSHConnection::new(config, event_sender);
        
        let result = connection.connect().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Password is required"));
    }

    #[tokio::test]
    async fn test_ssh_connection_missing_private_key() {
        let mut config = create_test_config_pubkey();
        config.private_key_path = None; // Missing key path for pubkey auth
        
        let (event_sender, _event_receiver) = mpsc::channel(10);
        let connection = SSHConnection::new(config, event_sender);
        
        let result = connection.connect().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Private key path is required"));
    }

    #[tokio::test]
    async fn test_ssh_connection_nonexistent_private_key() {
        let config = create_test_config_pubkey();
        
        let (event_sender, _event_receiver) = mpsc::channel(10);
        let connection = SSHConnection::new(config, event_sender);
        
        let result = connection.connect().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Private key file does not exist"));
    }

    #[tokio::test]
    async fn test_disconnect_without_connection() {
        let config = create_test_config();
        let (event_sender, _event_receiver) = mpsc::channel(10);
        
        let connection = SSHConnection::new(config, event_sender);
        
        // Should be able to disconnect even if not connected
        let result = connection.disconnect().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_session_info_when_not_connected() {
        let config = create_test_config();
        let (event_sender, _event_receiver) = mpsc::channel(10);
        
        let connection = SSHConnection::new(config, event_sender);
        
        // Should return None when not connected
        let info = connection.get_session_info().await;
        assert!(info.is_none());
    }

    #[tokio::test]
    async fn test_connection_state_management() {
        let config = create_test_config();
        let (event_sender, _event_receiver) = mpsc::channel(10);
        
        let connection = SSHConnection::new(config, event_sender);
        
        // Initially should not be connected
        assert!(!connection.is_connected().await);
        
        // Session info should be None
        let info = connection.get_session_info().await;
        assert!(info.is_none());
    }

    #[tokio::test]
    async fn test_event_sending() {
        let (event_sender, mut event_receiver) = mpsc::channel(10);
        let config = create_test_config();
        
        let connection = SSHConnection::new(config, event_sender);
        
        // Try to connect (will fail but should send events)
        let _ = connection.connect().await;
        
        // Should receive at least one event
        let event = timeout(Duration::from_millis(100), event_receiver.recv()).await;
        assert!(event.is_ok());
    }

    #[tokio::test]
    async fn test_auth_method_validation() {
        // Test password auth validation
        let mut config = create_test_config();
        config.password = Some("".to_string()); // Empty password
        
        let validation_result = config.validate();
        assert!(validation_result.is_err());
        
        // Test public key auth validation
        let mut config = create_test_config_pubkey();
        config.private_key_path = Some("".to_string()); // Empty key path
        
        let validation_result = config.validate();
        assert!(validation_result.is_err());
        
        // Test agent auth (should not require additional validation)
        let mut config = create_test_config();
        config.auth_method = AuthMethod::Agent;
        config.password = None;
        
        let validation_result = config.validate();
        assert!(validation_result.is_ok());
    }

    #[tokio::test]
    async fn test_terminal_session_methods_without_connection() {
        let config = create_test_config();
        let (event_sender, _event_receiver) = mpsc::channel(10);
        
        let connection = SSHConnection::new(config, event_sender);
        
        // Terminal operations should fail when not connected
        let result = connection.create_terminal_session().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("SSH session not available"));
        
        let result = connection.send_terminal_input("test", b"input").await;
        assert!(result.is_err());
        
        let result = connection.resize_terminal("test", 80, 24, 640, 480).await;
        assert!(result.is_err());
        
        let result = connection.close_terminal_session("test").await;
        assert!(result.is_err());
        
        let session = connection.get_terminal_session("test").await;
        assert!(session.is_none());
        
        let sessions = connection.list_terminal_sessions().await;
        assert!(sessions.is_empty());
    }

    #[tokio::test]
    async fn test_terminal_manager_integration() {
        let config = create_test_config();
        let (event_sender, _event_receiver) = mpsc::channel(10);
        
        let connection = SSHConnection::new(config, event_sender);
        
        // Terminal manager should be available
        let sessions = connection.list_terminal_sessions().await;
        assert!(sessions.is_empty());
    }
}