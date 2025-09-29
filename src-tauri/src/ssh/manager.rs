use crate::ssh::types::*;
use crate::ssh::connection::SSHConnection;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex, RwLock};
use uuid::Uuid;

pub struct SSHManager {
    connections: Arc<RwLock<HashMap<String, Arc<SSHConnection>>>>,
    connection_states: Arc<RwLock<HashMap<String, SSHConnectionState>>>,
    event_sender: mpsc::Sender<SSHEvent>,
    event_receiver: Arc<Mutex<mpsc::Receiver<SSHEvent>>>,
}

impl SSHManager {
    pub fn new() -> Self {
        let (event_sender, event_receiver) = mpsc::channel(100);
        
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            connection_states: Arc::new(RwLock::new(HashMap::new())),
            event_sender,
            event_receiver: Arc::new(Mutex::new(event_receiver)),
        }
    }

    pub async fn create_connection(&self, mut config: SSHConnectionConfig) -> Result<String, String> {
        // Validate configuration first
        config.validate().map_err(|errors| {
            format!("Configuration validation failed: {}", 
                errors.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", "))
        })?;

        // Generate ID if not provided
        if config.id.is_empty() {
            config.id = Uuid::new_v4().to_string();
        }

        // Create connection state
        let mut state = SSHConnectionState::new(config.id.clone());
        state.update_status(ConnectionStatus::Connecting);

        // Store initial state
        {
            let mut states = self.connection_states.write().await;
            states.insert(config.id.clone(), state);
        }

        // Create connection
        let connection = Arc::new(SSHConnection::new(config.clone(), self.event_sender.clone()));
        
        // Store connection
        {
            let mut connections = self.connections.write().await;
            connections.insert(config.id.clone(), connection.clone());
        }

        // Attempt to connect in background
        let connection_clone = connection.clone();
        let connection_id = config.id.clone();
        let states_clone = self.connection_states.clone();
        
        tokio::spawn(async move {
            match connection_clone.connect().await {
                Ok(()) => {
                    let mut states = states_clone.write().await;
                    if let Some(state) = states.get_mut(&connection_id) {
                        state.update_status(ConnectionStatus::Connected);
                    }
                }
                Err(error) => {
                    let mut states = states_clone.write().await;
                    if let Some(state) = states.get_mut(&connection_id) {
                        state.set_error(error);
                    }
                }
            }
        });
        
        Ok(config.id)
    }

    pub async fn connect_existing(&self, connection_id: &str) -> Result<(), String> {
        let connection = {
            let connections = self.connections.read().await;
            connections.get(connection_id).cloned()
                .ok_or_else(|| format!("Connection {} not found", connection_id))?
        };

        // Update state to connecting
        {
            let mut states = self.connection_states.write().await;
            if let Some(state) = states.get_mut(connection_id) {
                state.update_status(ConnectionStatus::Connecting);
            }
        }

        // Attempt connection
        match connection.connect().await {
            Ok(()) => {
                let mut states = self.connection_states.write().await;
                if let Some(state) = states.get_mut(connection_id) {
                    state.update_status(ConnectionStatus::Connected);
                }
                Ok(())
            }
            Err(error) => {
                let mut states = self.connection_states.write().await;
                if let Some(state) = states.get_mut(connection_id) {
                    state.set_error(error.clone());
                }
                Err(error)
            }
        }
    }

    pub async fn disconnect_connection(&self, connection_id: &str) -> Result<(), String> {
        let connection = {
            let connections = self.connections.read().await;
            connections.get(connection_id).cloned()
                .ok_or_else(|| format!("Connection {} not found", connection_id))?
        };

        // Disconnect
        connection.disconnect().await?;

        // Update state
        {
            let mut states = self.connection_states.write().await;
            if let Some(state) = states.get_mut(connection_id) {
                state.update_status(ConnectionStatus::Disconnected);
            }
        }

        Ok(())
    }

    pub async fn get_connection(&self, connection_id: &str) -> Option<Arc<SSHConnection>> {
        let connections = self.connections.read().await;
        connections.get(connection_id).cloned()
    }

    pub async fn get_connection_state(&self, connection_id: &str) -> Option<SSHConnectionState> {
        let states = self.connection_states.read().await;
        states.get(connection_id).cloned()
    }

    pub async fn remove_connection(&self, connection_id: &str) -> Result<(), String> {
        // First disconnect if connected
        if let Some(connection) = self.get_connection(connection_id).await {
            let _ = connection.disconnect().await; // Ignore errors during cleanup
        }

        // Remove from both maps
        let mut connections = self.connections.write().await;
        let mut states = self.connection_states.write().await;
        
        connections.remove(connection_id);
        states.remove(connection_id);
        
        Ok(())
    }

    pub async fn list_connections(&self) -> Vec<String> {
        let connections = self.connections.read().await;
        connections.keys().cloned().collect()
    }

    pub async fn list_connection_states(&self) -> Vec<SSHConnectionState> {
        let states = self.connection_states.read().await;
        states.values().cloned().collect()
    }

    pub async fn is_connected(&self, connection_id: &str) -> bool {
        if let Some(connection) = self.get_connection(connection_id).await {
            connection.is_connected().await
        } else {
            false
        }
    }

    // Event handling methods
    pub async fn get_event_receiver(&self) -> Arc<Mutex<mpsc::Receiver<SSHEvent>>> {
        self.event_receiver.clone()
    }

    pub async fn send_event(&self, event: SSHEvent) -> Result<(), String> {
        self.event_sender.send(event).await
            .map_err(|e| format!("Failed to send event: {}", e))
    }
}