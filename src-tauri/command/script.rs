use crate::usecase::app::AppUsecase;

#[tauri::command]
pub fn script_run(id: String) {
    AppUsecase::new().run_script(&id);
}
