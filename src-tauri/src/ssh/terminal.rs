use crate::ssh::types::*;
use russh::client::Msg;
use russh::Channel;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio::task::JoinHandle;

pub struct TerminalSessionManager {
    sessions: Arc<RwLock<HashMap<String, TerminalSessionData>>>,
    event_sender: mpsc::Sender<SSHEvent>,
}

struct TerminalSessionData {
    session: TerminalSession,
    ssh_channel: Option<Channel<Msg>>,
    input_task: Option<JoinHandle<()>>,
    output_task: Option<JoinHandle<()>>,
}

impl TerminalSessionManager {
    pub fn new(event_sender: mpsc::Sender<SSHEvent>) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            event_sender,
        }
    }

    /// Creates a new terminal session for the given SSH connection
    pub async fn create_session(
        &self,
        connection_id: String,
    ) -> Result<String, String> {
        let terminal_session = TerminalSession::new(connection_id.clone());
        let terminal_id = terminal_session.id.clone();

        // Create terminal session data (simplified for now)
        let session_data = TerminalSessionData {
            session: terminal_session,
            ssh_channel: None, // Will be set up later when we have proper SSH channel management
            input_task: None,
            output_task: None,
        };

        // Store session
        {
            let mut sessions = self.sessions.write().await;
            sessions.insert(terminal_id.clone(), session_data);
        }

        // Start I/O handling tasks
        self.start_io_tasks(&terminal_id, &connection_id).await?;

        // Send terminal created event
        let _ = self
            .event_sender
            .send(SSHEvent::TerminalCreated(connection_id, terminal_id.clone()))
            .await;

        Ok(terminal_id)
    }

    /// Starts the input/output handling tasks for a terminal session
    async fn start_io_tasks(&self, terminal_id: &str, _connection_id: &str) -> Result<(), String> {
        let sessions = self.sessions.clone();
        let _event_sender = self.event_sender.clone();
        let terminal_id = terminal_id.to_string();

        // Start output task - this will be handled by the SSH client handler
        // For now, we just create a placeholder task
        let output_task = {
            let sessions = sessions.clone();
            let terminal_id = terminal_id.clone();

            tokio::spawn(async move {
                // This task monitors the session and handles cleanup
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                    
                    // Check if session still exists
                    let sessions = sessions.read().await;
                    if !sessions.contains_key(&terminal_id) {
                        break;
                    }
                }
            })
        };

        // Start input task - handles input from frontend to SSH
        let input_task = {
            let sessions = sessions.clone();
            let terminal_id = terminal_id.clone();

            tokio::spawn(async move {
                // This task will be driven by external input via send_input method
                // For now, it's a placeholder that keeps the task alive
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    
                    // Check if session still exists
                    let sessions = sessions.read().await;
                    if !sessions.contains_key(&terminal_id) {
                        break;
                    }
                }
            })
        };

        // Store task handles
        {
            let mut sessions = self.sessions.write().await;
            if let Some(session_data) = sessions.get_mut(&terminal_id) {
                session_data.output_task = Some(output_task);
                session_data.input_task = Some(input_task);
            }
        }

        Ok(())
    }

    /// Sends input to a terminal session
    pub async fn send_input(&self, terminal_id: &str, data: &[u8]) -> Result<(), String> {
        let sessions = self.sessions.read().await;
        if let Some(session_data) = sessions.get(terminal_id) {
            // Send data to SSH channel
            if let Some(ref channel) = session_data.ssh_channel {
                channel
                    .data(data)
                    .await
                    .map_err(|e| format!("Failed to send data to SSH channel: {}", e))?;
                Ok(())
            } else {
                Err("SSH channel not available".to_string())
            }
        } else {
            Err(format!("Terminal session {} not found", terminal_id))
        }
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
        let mut sessions = self.sessions.write().await;
        if let Some(session_data) = sessions.get_mut(terminal_id) {
            // Update session size
            session_data.session.resize(cols, rows, pixel_width, pixel_height);

            // Resize SSH channel if available
            if let Some(ref channel) = session_data.ssh_channel {
                channel
                    .window_change(cols as u32, rows as u32, pixel_width as u32, pixel_height as u32)
                    .await
                    .map_err(|e| format!("Failed to resize SSH channel: {}", e))?;
            }

            // Send resize event
            let _ = self
                .event_sender
                .send(SSHEvent::TerminalResized(
                    session_data.session.connection_id.clone(),
                    terminal_id.to_string(),
                    cols,
                    rows,
                ))
                .await;

            Ok(())
        } else {
            Err(format!("Terminal session {} not found", terminal_id))
        }
    }

    /// Closes a terminal session
    pub async fn close_session(&self, terminal_id: &str) -> Result<(), String> {
        let mut sessions = self.sessions.write().await;
        if let Some(mut session_data) = sessions.remove(terminal_id) {
            // Deactivate session
            session_data.session.deactivate();

            // Close SSH channel
            if let Some(channel) = session_data.ssh_channel.take() {
                let _ = channel.close().await;
            }

            // Cancel I/O tasks
            if let Some(task) = session_data.input_task.take() {
                task.abort();
            }
            if let Some(task) = session_data.output_task.take() {
                task.abort();
            }

            // Send terminal closed event
            let _ = self
                .event_sender
                .send(SSHEvent::TerminalClosed(
                    session_data.session.connection_id.clone(),
                    terminal_id.to_string(),
                ))
                .await;

            Ok(())
        } else {
            Err(format!("Terminal session {} not found", terminal_id))
        }
    }

    /// Gets information about a terminal session
    pub async fn get_session(&self, terminal_id: &str) -> Option<TerminalSession> {
        let sessions = self.sessions.read().await;
        sessions.get(terminal_id).map(|data| data.session.clone())
    }

    /// Lists all terminal sessions for a connection
    pub async fn list_sessions_for_connection(&self, connection_id: &str) -> Vec<TerminalSession> {
        let sessions = self.sessions.read().await;
        sessions
            .values()
            .filter(|data| data.session.connection_id == connection_id)
            .map(|data| data.session.clone())
            .collect()
    }

    /// Closes all terminal sessions for a connection
    pub async fn close_all_sessions_for_connection(&self, connection_id: &str) -> Result<(), String> {
        let terminal_ids: Vec<String> = {
            let sessions = self.sessions.read().await;
            sessions
                .values()
                .filter(|data| data.session.connection_id == connection_id)
                .map(|data| data.session.id.clone())
                .collect()
        };

        for terminal_id in terminal_ids {
            let _ = self.close_session(&terminal_id).await; // Continue even if some fail
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_terminal_session_creation() {
        let session = TerminalSession::new("test-connection".to_string());
        
        assert_eq!(session.connection_id, "test-connection");
        assert!(session.is_active);
        assert_eq!(session.size.cols, 80);
        assert_eq!(session.size.rows, 24);
        assert!(!session.id.is_empty());
    }

    #[tokio::test]
    async fn test_terminal_session_resize() {
        let mut session = TerminalSession::new("test-connection".to_string());
        
        session.resize(120, 30, 960, 600);
        
        assert_eq!(session.size.cols, 120);
        assert_eq!(session.size.rows, 30);
        assert_eq!(session.size.pixel_width, 960);
        assert_eq!(session.size.pixel_height, 600);
    }

    #[tokio::test]
    async fn test_terminal_session_deactivate() {
        let mut session = TerminalSession::new("test-connection".to_string());
        
        assert!(session.is_active);
        
        session.deactivate();
        
        assert!(!session.is_active);
    }

    #[tokio::test]
    async fn test_terminal_session_manager_creation() {
        let (event_sender, _event_receiver) = mpsc::channel(10);
        let manager = TerminalSessionManager::new(event_sender);
        
        // Manager should be created successfully
        let sessions = manager.sessions.read().await;
        assert!(sessions.is_empty());
    }

    #[tokio::test]
    async fn test_terminal_size_default() {
        let size = TerminalSize::default();
        
        assert_eq!(size.cols, 80);
        assert_eq!(size.rows, 24);
        assert_eq!(size.pixel_width, 640);
        assert_eq!(size.pixel_height, 480);
    }

    #[tokio::test]
    async fn test_get_nonexistent_session() {
        let (event_sender, _event_receiver) = mpsc::channel(10);
        let manager = TerminalSessionManager::new(event_sender);
        
        let session = manager.get_session("nonexistent").await;
        assert!(session.is_none());
    }

    #[tokio::test]
    async fn test_close_nonexistent_session() {
        let (event_sender, _event_receiver) = mpsc::channel(10);
        let manager = TerminalSessionManager::new(event_sender);
        
        let result = manager.close_session("nonexistent").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[tokio::test]
    async fn test_send_input_to_nonexistent_session() {
        let (event_sender, _event_receiver) = mpsc::channel(10);
        let manager = TerminalSessionManager::new(event_sender);
        
        let result = manager.send_input("nonexistent", b"test").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[tokio::test]
    async fn test_resize_nonexistent_session() {
        let (event_sender, _event_receiver) = mpsc::channel(10);
        let manager = TerminalSessionManager::new(event_sender);
        
        let result = manager.resize_terminal("nonexistent", 80, 24, 640, 480).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[tokio::test]
    async fn test_list_sessions_for_connection() {
        let (event_sender, _event_receiver) = mpsc::channel(10);
        let manager = TerminalSessionManager::new(event_sender);
        
        // Should return empty list for nonexistent connection
        let sessions = manager.list_sessions_for_connection("nonexistent").await;
        assert!(sessions.is_empty());
    }

    #[tokio::test]
    async fn test_close_all_sessions_for_connection() {
        let (event_sender, _event_receiver) = mpsc::channel(10);
        let manager = TerminalSessionManager::new(event_sender);
        
        // Should succeed even for nonexistent connection
        let result = manager.close_all_sessions_for_connection("nonexistent").await;
        assert!(result.is_ok());
    }
}