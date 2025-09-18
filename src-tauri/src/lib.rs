use tauri::{TitleBarStyle, WebviewUrl, WebviewWindowBuilder};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn system_info() -> String {
    return std::env::consts::OS.to_string();
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(target_os = "macos")]
            let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("Hana")
                .inner_size(1100.0, 700.0)
                .resizable(true)
                .decorations(true)
                .title_bar_style(TitleBarStyle::Overlay)
                .hidden_title(true)
                .accept_first_mouse(true)
                .traffic_light_position(tauri::LogicalPosition::new(10.0, 18.0));

            #[cfg(target_os = "windows")]
            let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("Hana")
                .inner_size(1100.0, 700.0)
                .resizable(true)
                .decorations(false)
                .accept_first_mouse(true);

            let window = win_builder.build().map_err(|e| {
                eprintln!("Failed to build the window: {}", e);
                e
            })?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
