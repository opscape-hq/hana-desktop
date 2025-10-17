use tauri::{TitleBarStyle, WebviewUrl, WebviewWindowBuilder};
use std::sync::Arc;
use tokio::sync::Mutex;

mod ssh;
mod commands;

use ssh::manager::SSHManager;
use commands::ssh_commands::SSHManagerState;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn platform() -> String {
    return std::env::consts::OS.to_string();
}

#[cfg(mobile)]
#[tauri::mobile_entry_point]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|_app, argv, _cwd| {
          println!("a new app instance was opened with {argv:?} and the deep link event was already triggered");
          // when defining deep link schemes at runtime, you must also check `argv` here
        }));
    }

    builder = builder.plugin(tauri_plugin_deep_link::init());
}

#[cfg(desktop)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(target_os = "macos")]
            let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("Hana")
                .inner_size(1000.0, 600.0)
                .min_inner_size(700.0, 445.0)
                .resizable(true)
                .decorations(true)
                .title_bar_style(TitleBarStyle::Overlay)
                .hidden_title(true)
                .accept_first_mouse(true)
                .traffic_light_position(tauri::LogicalPosition::new(10.0, 24.0));

            #[cfg(target_os = "windows")]
            let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("Hana")
                .inner_size(1000.0, 600.0)
                .min_inner_size(700.0, 445.0)
                .resizable(true)
                .decorations(false)
                .accept_first_mouse(true);

            let _window = win_builder.build().map_err(|e| {
                eprintln!("Failed to build the window: {}", e);
                e
            })?;

            Ok(())
        })
        .manage(Arc::new(Mutex::new(SSHManager::new())) as SSHManagerState)
        .invoke_handler(tauri::generate_handler![
            platform,
            commands::ssh_commands::create_ssh_connection,
            commands::ssh_commands::connect_ssh,
            commands::ssh_commands::disconnect_ssh,
            commands::ssh_commands::remove_ssh_connection,
            commands::ssh_commands::get_connection_state,
            commands::ssh_commands::list_connection_states,
            commands::ssh_commands::create_ssh_channel,
            commands::ssh_commands::request_pty,
            commands::ssh_commands::request_shell,
            commands::ssh_commands::send_terminal_input,
            commands::ssh_commands::resize_terminal,
            commands::ssh_commands::close_ssh_channel,
            commands::ssh_commands::list_ssh_connections
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
