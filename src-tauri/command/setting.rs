use crate::repository::rocks::RocksRepository;

#[tauri::command]
pub fn status() -> String {
    RocksRepository::check_connect()
}