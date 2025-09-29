use crate::ssh::types::*;
use tauri::State;
use std::sync::Arc;

// Global SSH Manager state
pub type SSHManagerState = Arc<tauri::async_runtime::Mutex<crate::ssh::manager::SSHManager>>;

#[tauri::command]
pub async fn create_ssh_connection(
    config: SSHConnectionConfig,
    ssh_manager: State<'_, SSHManagerState>,
) -> Result<String, String> {
    let manager = ssh_manager.inner().lock().await;
    manager.create_connection(config).await
}

#[tauri::command]
pub async fn connect_ssh(
    connection_id: String,
    ssh_manager: State<'_, SSHManagerState>,
) -> Result<(), String> {
    let manager = ssh_manager.inner().lock().await;
    manager.connect_existing(&connection_id).await
}

#[tauri::command]
pub async fn disconnect_ssh(
    connection_id: String,
    ssh_manager: State<'_, SSHManagerState>,
) -> Result<(), String> {
    let manager = ssh_manager.inner().lock().await;
    manager.disconnect_connection(&connection_id).await
}

#[tauri::command]
pub async fn remove_ssh_connection(
    connection_id: String,
    ssh_manager: State<'_, SSHManagerState>,
) -> Result<(), String> {
    let manager = ssh_manager.inner().lock().await;
    manager.remove_connection(&connection_id).await
}

#[tauri::command]
pub async fn get_connection_state(
    connection_id: String,
    ssh_manager: State<'_, SSHManagerState>,
) -> Result<Option<SSHConnectionState>, String> {
    let manager = ssh_manager.inner().lock().await;
    Ok(manager.get_connection_state(&connection_id).await)
}

#[tauri::command]
pub async fn list_connection_states(
    ssh_manager: State<'_, SSHManagerState>,
) -> Result<Vec<SSHConnectionState>, String> {
    let manager = ssh_manager.inner().lock().await;
    Ok(manager.list_connection_states().await)
}

#[tauri::command]
pub async fn create_ssh_channel(
    connection_id: String,
    ssh_manager: State<'_, SSHManagerState>,
) -> Result<String, String> {
    let manager = ssh_manager.inner().lock().await;
    if let Some(_connection) = manager.get_connection(&connection_id).await {
        // Channel creation will be implemented in terminal session task
        Err("Channel creation not yet fully implemented".to_string())
    } else {
        Err(format!("Connection {} not found", connection_id))
    }
}

#[tauri::command]
pub async fn request_pty(
    connection_id: String,
    _channel_id_str: String,
    _cols: u16,
    _rows: u16,
    ssh_manager: State<'_, SSHManagerState>,
) -> Result<(), String> {
    let manager = ssh_manager.inner().lock().await;
    if let Some(_connection) = manager.get_connection(&connection_id).await {
        // For now, we'll need to store channel IDs differently
        // This is a simplified approach for the current implementation
        Err("PTY request not yet fully implemented".to_string())
    } else {
        Err(format!("Connection {} not found", connection_id))
    }
}

#[tauri::command]
pub async fn request_shell(
    connection_id: String,
    _channel_id_str: String,
    ssh_manager: State<'_, SSHManagerState>,
) -> Result<(), String> {
    let manager = ssh_manager.inner().lock().await;
    if let Some(_connection) = manager.get_connection(&connection_id).await {
        // Simplified for now - will be implemented in terminal session task
        Err("Shell request not yet fully implemented".to_string())
    } else {
        Err(format!("Connection {} not found", connection_id))
    }
}

#[tauri::command]
pub async fn send_terminal_input(
    connection_id: String,
    _channel_id_str: String,
    _data: String,
    ssh_manager: State<'_, SSHManagerState>,
) -> Result<(), String> {
    let manager = ssh_manager.inner().lock().await;
    if let Some(_connection) = manager.get_connection(&connection_id).await {
        // Simplified for now - will be implemented in terminal session task
        Err("Terminal input not yet fully implemented".to_string())
    } else {
        Err(format!("Connection {} not found", connection_id))
    }
}

#[tauri::command]
pub async fn resize_terminal(
    connection_id: String,
    _channel_id_str: String,
    _cols: u16,
    _rows: u16,
    ssh_manager: State<'_, SSHManagerState>,
) -> Result<(), String> {
    let manager = ssh_manager.inner().lock().await;
    if let Some(_connection) = manager.get_connection(&connection_id).await {
        // Simplified for now - will be implemented in terminal session task
        Err("Terminal resize not yet fully implemented".to_string())
    } else {
        Err(format!("Connection {} not found", connection_id))
    }
}

#[tauri::command]
pub async fn close_ssh_channel(
    connection_id: String,
    _channel_id_str: String,
    ssh_manager: State<'_, SSHManagerState>,
) -> Result<(), String> {
    let manager = ssh_manager.inner().lock().await;
    if let Some(_connection) = manager.get_connection(&connection_id).await {
        // Simplified for now - will be implemented in terminal session task
        Err("Channel close not yet fully implemented".to_string())
    } else {
        Err(format!("Connection {} not found", connection_id))
    }
}

#[tauri::command]
pub async fn list_ssh_connections(
    ssh_manager: State<'_, SSHManagerState>,
) -> Result<Vec<String>, String> {
    let manager = ssh_manager.inner().lock().await;
    Ok(manager.list_connections().await)
}

// Terminal session management commands

#[tauri::command]
pub async fn create_terminal_session(
    connection_id: String,
    ssh_manager: State<'_, SSHManagerState>,
) -> Result<String, String> {
    let manager = ssh_manager.inner().lock().await;
    if let Some(connection) = manager.get_connection(&connection_id).await {
        connection.create_terminal_session().await
    } else {
        Err(format!("Connection {} not found", connection_id))
    }
}

#[tauri::command]
pub async fn send_terminal_input_to_session(
    connection_id: String,
    terminal_id: String,
    data: String,
    ssh_manager: State<'_, SSHManagerState>,
) -> Result<(), String> {
    let manager = ssh_manager.inner().lock().await;
    if let Some(connection) = manager.get_connection(&connection_id).await {
        connection.send_terminal_input(&terminal_id, data.as_bytes()).await
    } else {
        Err(format!("Connection {} not found", connection_id))
    }
}

#[tauri::command]
pub async fn resize_terminal_session(
    connection_id: String,
    terminal_id: String,
    cols: u16,
    rows: u16,
    pixel_width: u16,
    pixel_height: u16,
    ssh_manager: State<'_, SSHManagerState>,
) -> Result<(), String> {
    let manager = ssh_manager.inner().lock().await;
    if let Some(connection) = manager.get_connection(&connection_id).await {
        connection.resize_terminal(&terminal_id, cols, rows, pixel_width, pixel_height).await
    } else {
        Err(format!("Connection {} not found", connection_id))
    }
}

#[tauri::command]
pub async fn close_terminal_session(
    connection_id: String,
    terminal_id: String,
    ssh_manager: State<'_, SSHManagerState>,
) -> Result<(), String> {
    let manager = ssh_manager.inner().lock().await;
    if let Some(connection) = manager.get_connection(&connection_id).await {
        connection.close_terminal_session(&terminal_id).await
    } else {
        Err(format!("Connection {} not found", connection_id))
    }
}

#[tauri::command]
pub async fn get_terminal_session(
    connection_id: String,
    terminal_id: String,
    ssh_manager: State<'_, SSHManagerState>,
) -> Result<Option<TerminalSession>, String> {
    let manager = ssh_manager.inner().lock().await;
    if let Some(connection) = manager.get_connection(&connection_id).await {
        Ok(connection.get_terminal_session(&terminal_id).await)
    } else {
        Err(format!("Connection {} not found", connection_id))
    }
}

#[tauri::command]
pub async fn list_terminal_sessions(
    connection_id: String,
    ssh_manager: State<'_, SSHManagerState>,
) -> Result<Vec<TerminalSession>, String> {
    let manager = ssh_manager.inner().lock().await;
    if let Some(connection) = manager.get_connection(&connection_id).await {
        Ok(connection.list_terminal_sessions().await)
    } else {
        Err(format!("Connection {} not found", connection_id))
    }
}