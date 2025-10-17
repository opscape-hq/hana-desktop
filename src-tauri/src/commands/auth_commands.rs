#[tauri::command]
pub async fn get_authorization_status(
) -> Result<String, String> {
  Ok("Authorized".into())
}