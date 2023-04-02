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
            subscribe::create_subscribe,
            script::create_script,
            script::run,
            workspace::get_workspace,
            workspace::put_workspace,
            project::list_projects,
            project::get_project,
            project::create_project,
            project::delete_project,
        ])
}