use crate::ssh::types::*;
use crate::ssh::connection::SSHConnection;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use uuid::Uuid;

pub struct SSHManager {
    connections: Arc<Mutex<ConnectionMap>>,
    event_sender: mpsc::Sender<SSHEvent>,
    event_receiver: Arc<Mutex<mpsc::Receiver<SSHEvent>>>,
}

impl SSHManager {
    pub fn new() -> Self {
        let (event_sender, event_receiver) = mpsc::channel(100);
        
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
            event_sender,
            event_receiver: Arc::new(Mutex::new(event_receiver)),
        }
    }

    pub async fn create_connection(&self, mut config: SSHConnectionConfig) -> Result<String, String> {
        // Generate ID if not provided
        if config.id.is_empty() {
            config.id = Uuid::new_v4().to_string();
        }

        let connection = SSHConnection::new(config.clone(), self.event_sender.clone());
        
        let mut connections = self.connections.lock().await;
        connections.insert(config.id.clone(), connection);
        
        Ok(config.id)
    }

    pub async fn get_connection(&self, connection_id: &str) -> Option<SSHConnection> {
        let connections = self.connections.lock().await;
        connections.get(connection_id).cloned()
    }

    pub async fn remove_connection(&self, connection_id: &str) -> Result<(), String> {
        let mut connections = self.connections.lock().await;
        if connections.remove(connection_id).is_some() {
            Ok(())
        } else {
            Err(format!("Connection {} not found", connection_id))
        }
    }

    pub async fn list_connections(&self) -> Vec<String> {
        let connections = self.connections.lock().await;
        connections.keys().cloned().collect()
    }
}