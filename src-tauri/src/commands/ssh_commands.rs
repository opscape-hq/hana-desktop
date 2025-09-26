use crate::ssh::types::*;
use tauri::State;
use std::sync::Arc;
use tokio::sync::Mutex;

// Global SSH Manager state
pub type SSHManagerState = Arc<Mutex<crate::ssh::manager::SSHManager>>;

#[tauri::command]
pub async fn create_ssh_connection(
    config: SSHConnectionConfig,
    ssh_manager: State<'_, SSHManagerState>,
) -> Result<String, String> {
    let manager = ssh_manager.lock().await;
    manager.create_connection(config).await
}

#[tauri::command]
pub async fn disconnect_ssh(
    connection_id: String,
    ssh_manager: State<'_, SSHManagerState>,
) -> Result<(), String> {
    let manager = ssh_manager.lock().await;
    manager.remove_connection(&connection_id).await
}

#[tauri::command]
pub async fn send_terminal_input(
    connection_id: String,
    data: String,
    ssh_manager: State<'_, SSHManagerState>,
) -> Result<(), String> {
    let manager = ssh_manager.lock().await;
    if let Some(connection) = manager.get_connection(&connection_id).await {
        connection.send_data(data.as_bytes()).await
    } else {
        Err(format!("Connection {} not found", connection_id))
    }
}

#[tauri::command]
pub async fn resize_terminal(
    connection_id: String,
    cols: u16,
    rows: u16,
    ssh_manager: State<'_, SSHManagerState>,
) -> Result<(), String> {
    let manager = ssh_manager.lock().await;
    if let Some(connection) = manager.get_connection(&connection_id).await {
        connection.resize_terminal(cols, rows).await
    } else {
        Err(format!("Connection {} not found", connection_id))
    }
}

#[tauri::command]
pub async fn list_ssh_connections(
    ssh_manager: State<'_, SSHManagerState>,
) -> Result<Vec<String>, String> {
    let manager = ssh_manager.lock().await;
    Ok(manager.list_connections().await)
}