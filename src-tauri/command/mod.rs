mod event;
mod workspace;
mod project;
mod subscribe;
mod script;

use tauri::Builder;
use tauri::Wry;

pub fn inject_commands(app: Builder<Wry>) -> Builder<Wry> {
    app
        .invoke_handler(tauri::generate_handler![
            event::event_publish,
            event::event_list,
            event::event_get,
            script::script_list,
            script::script_get,
            script::script_create,
            script::script_delete,
            script::script_run,
            subscribe::subscribe_list,
            subscribe::subscribe_get,
            subscribe::subscribe_create,
            subscribe::subscribe_delete,
            workspace::get_workspace,
            workspace::set_workspace,
            project::project_list,
            project::project_get,
            project::project_create,
            project::project_delete,
        ])
}